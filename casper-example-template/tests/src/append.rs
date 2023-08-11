use casper_engine_test_support::ExecuteRequestBuilder;
use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, misc, query};
use contract::constants;

#[test]
fn append() {
    let (account_addr, mut builder) = misc::deploy_contract();
    // call register first
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENTRYPOINT,
        runtime_args! {},
    )
    .build();

    builder.exec(call_register).expect_success().commit();

    let accum_value_key =
        misc::get_contract_key(&builder, account_addr, constants::append::ACCUM_VALUE);

    // call append entrypoint 1st time
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::append::ENTRYPOINT,
        runtime_args! {
            constants::append::ARG => "test-1"
        },
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let current_value: String = query::query_key(&builder, accum_value_key);
    assert_eq!(current_value, "test-1");

    // call append entrypoint 2nd time
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::append::ENTRYPOINT,
        runtime_args! {
            constants::append::ARG => "test-2"
        },
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let current_value: String = query::query_key(&builder, accum_value_key);
    assert_eq!(current_value, "test-1;test-2");
}

#[test]
fn unregistered_can_not_append() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_append = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::append::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_append).expect_failure().commit();
    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        3,
        "should throw an error corresponding to double registration",
    );
}
