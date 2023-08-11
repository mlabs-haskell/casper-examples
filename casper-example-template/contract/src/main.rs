//! This is simple example of Casper Contract with explainers and walkthrough commentaries.
//! This example shows:
//! - How to initialize contract during deployment
//! - How to read and write Named Keys
//! - How to work with Dictionary
//! - How to work with events

#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{contracts::NamedKeys, ApiError, RuntimeArgs};
use entry_points::mk_entry_points;

mod constants;
mod entry_points;
mod error;
mod events;
mod utils;

/// Main entry point that will be run during Deploy execution.
/// Beware: This code will be called inside `Account` context.
/// Putting any named keys or dictionaries will add them into Account context,
/// not into Contract context (so this keys and dictionaries will not be awailable
/// inside deployed contract)
#[no_mangle]
pub extern "C" fn call() {
    if runtime::get_key(constants::contract::ACCESS_UREF).is_some() {
        runtime::revert(error::Error::AlredayDeployed)
    }
    install_contract();
}

// Beware: This code will be called inside `Account` context.
// Putting any named keys or dictionaries will add them into Account context,
// not into Contract context (so this keys and dictionaries will not be awaitable
// inside deployed contract)
fn install_contract() {
    // Adding named key to contract that will be used to accumulate messages
    // from `append_phrase` entrypoint.
    // Note that it is not recommended to use named keys to store big amount of data,
    // prefer to use Dictionary (see https://docs.casper.network/concepts/dictionaries/).
    // Named key is used here for the demo purposes.
    let mut contract_keys = NamedKeys::new();

    // URef containing string value that will be used to accumulate messages from
    // `append_phrase` entrypoint.
    // URefs are used to store values on-chain,
    // see https://docs.casper.network/concepts/design/casper-design/#uref-head
    // Note, that it is possible to set AccessRights to URef,
    // e.g. to make it read or write only.
    // For more details see AccessRights in Rust docs and
    // https://docs.casper.network/concepts/design/casper-design/#uref-permissions
    let new_empty_val = storage::new_uref("");

    contract_keys.insert(
        constants::append::ACCUM_VALUE.to_string(),
        new_empty_val.into(),
    );

    // Creates upgradable contract.
    // For not-upgradable use "storage::new_locked_contract"
    // AFAIK, at the moment there is no way to tell if already deployed contract
    // is upgradable or not.
    let (contract_hash, contract_version) = storage::new_contract(
        mk_entry_points(),
        Some(contract_keys),
        Some(constants::contract::PACKAGE_NAME.to_string()),
        // Access URef required for at least upgrading the contract,
        // w/o it upgrade is not possible
        Some(constants::contract::ACCESS_UREF.to_string()),
    );

    runtime::put_key(constants::contract::KEY, contract_hash.into());
    runtime::put_key(
        constants::contract::VERSION_KEY,
        storage::new_uref(contract_version).into(),
    );

    // It is common practice to call something like `init()` entrypoint
    // to initialize Contract state. See "pub extern "C" fn init()" below for details.
    runtime::call_contract(
        contract_hash,
        constants::init::ENTRYPOINT,
        RuntimeArgs::new(),
    )
}

#[no_mangle]
pub extern "C" fn init() {
    // Beware: it is up to Contract author to make sure that Caontract
    // can not be initiated twice
    ensure_not_init();

    // Dictionary will be created in Contract context,
    // because `init()` is called as Contract entrypoint
    storage::new_dictionary(constants::registry::DICT).unwrap_or_revert();

    let empty_map: BTreeMap<String, bool> = BTreeMap::new();
    storage::named_dictionary_put(
        constants::registry::DICT,
        constants::registry::REGISTRY_MAP,
        empty_map,
    );

    // Initialize events provided by `casper-event-standard` lib.
    // Events are store in special Dictionary, so they are initialized inside the
    // Contract context to make this Dictionary available inside the Contract
    events::init_events();
}

fn ensure_not_init() {
    if runtime::get_key(constants::registry::DICT).is_some() {
        runtime::revert(error::Error::AlreadyInitialized)
    }
}

#[no_mangle]
pub extern "C" fn register_user_key() {
    let mut registry = utils::get_registration_map();
    let (is_registered, account_hash) = utils::caller_in_registry(&registry);

    if is_registered {
        runtime::revert(error::Error::UserAlreadyRegistered);
    }

    registry.insert(account_hash, true);

    storage::named_dictionary_put(
        constants::registry::DICT,
        constants::registry::REGISTRY_MAP,
        registry,
    );
}

#[no_mangle]
pub extern "C" fn append_phrase() {
    if !utils::caller_is_registered() {
        runtime::revert(error::Error::UnregisteredTriedToAdd)
    }

    let val_key = runtime::get_key(constants::append::ACCUM_VALUE)
        .unwrap_or_revert_with(error::Error::ValueKeyNotFound);
    let what_to_add: String = runtime::get_named_arg(constants::append::ARG);
    let mut current_value: String = storage::read_from_key(val_key)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);

    if !current_value.is_empty() {
        current_value.push(';');
    }
    current_value.push_str(&what_to_add);

    let key_uref = val_key
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::write(key_uref, current_value);
}

pub(crate) fn caller_is_registered() -> (bool, String) {
    let account_hash = runtime::get_caller().to_string();
    let key = account_hash.as_str();
    let is_registered = storage::named_dictionary_get(constants::registry::DICT, key)
        .unwrap_or_revert()
        .unwrap_or(false);
    (is_registered, account_hash)
}

#[no_mangle]
pub extern "C" fn emit_event() {
    let message: String = runtime::get_named_arg(constants::events::SOME_EVENT_MSG);
    let event = events::SomeEvent { message };
    casper_event_standard::emit(event);
}
