use core::convert::TryInto;

use alloc::{
    borrow::ToOwned,
    collections::BTreeMap,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_event_standard::Schemas;
use casper_types::{ApiError, ContractHash, URef};

use crate::constants;
use crate::error::Error;

pub(crate) fn get_contract_hash() -> ContractHash {
    let uref: URef = runtime::get_key(constants::contract::KEY)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert()
        .try_into()
        .unwrap_or_revert();

    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}

pub(crate) fn caller_is_registered() -> bool {
    let registry = get_registration_map();
    let (is_registered, _) = caller_in_registry(&registry);
    is_registered
}

pub(crate) fn caller_in_registry(registration_map: &BTreeMap<String, bool>) -> (bool, String) {
    let account_hash = runtime::get_caller().to_string();
    let is_registered = registration_map
        .get(&account_hash)
        .unwrap_or(&false)
        .to_owned();

    (is_registered, account_hash)
}

pub(crate) fn get_registration_map() -> BTreeMap<String, bool> {
    let registration_map: BTreeMap<String, bool> =
        storage::named_dictionary_get(constants::registry::DICT, constants::registry::REGISTRY_MAP)
            .unwrap_or_revert()
            .unwrap_or_revert_with(Error::RegistrationMapNotFound);

    return registration_map;
}
