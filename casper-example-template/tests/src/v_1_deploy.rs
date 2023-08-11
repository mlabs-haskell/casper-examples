use casper_engine_test_support::ExecuteRequestBuilder;

use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, debug, misc, wasm};
use contract::constants;

#[test]
fn deploy() {
    let (account_addr, builder) = misc::deploy_contract();

    let contract = misc::get_contract(&builder, account_addr);
    let contract_keys = contract.named_keys();

    contract_keys
        .get(constants::append::ACCUM_VALUE)
        .expect("Accum value should exist after contract initialization");

    contract_keys
        .get(constants::registry::DICT)
        .expect("Registry dict should exist after contract initialization");
}

#[test]
fn can_not_deploy_second_time() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let execute_request =
        ExecuteRequestBuilder::standard(account_addr, wasm::CONTRACT_WASM, runtime_args! {})
            .build();

    builder.exec(execute_request).commit().expect_failure();
}

#[test]
fn can_not_init_second_time() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let execute_request =
        ExecuteRequestBuilder::standard(account_addr, wasm::CONTRACT_WASM, runtime_args! {})
            .build();

    // deployment logic calls `init` inside `call`
    builder.exec(execute_request).commit().expect_failure();

    let call_init = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::init::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_init).expect_failure().commit();

    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        1,
        "should throw an error corresponding to double initialization",
    );
}
