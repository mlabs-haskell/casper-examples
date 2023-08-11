use std::collections::BTreeMap;

use casper_engine_test_support::ExecuteRequestBuilder;
use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, misc, query};
use contract::constants;

#[test]
fn registration_updates_registration_map() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let key = account_addr.to_string();
    let reg_map: BTreeMap<String, bool> = query::named_dictionary(
        &builder,
        account_addr,
        constants::registry::DICT,
        constants::registry::REGISTRY_MAP,
    );
    let is_registered = reg_map.get(&key).unwrap().to_owned();
    assert_eq!(is_registered, true);
}

#[test]
fn can_not_register_twice() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_failure().commit();
    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        2,
        "should throw an error corresponding to double registration",
    );
}
