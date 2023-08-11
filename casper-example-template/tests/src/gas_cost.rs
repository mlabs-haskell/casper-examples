use casper_engine_test_support::ExecuteRequestBuilder;
use casper_types::{Gas, runtime_args, RuntimeArgs};
use contract::constants;

use crate::utility::misc;

#[test]
fn install_cost_regression() {
    let (_, builder) = misc::deploy_contract();
    let gas = builder.last_exec_gas_cost();
    let expected_gas: Gas = Gas::from(47456416310 as u64);
    assert_eq!(gas, expected_gas);
}

#[test]
fn register_cost_regression(){
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENTRYPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_success().commit();
    let gas = builder.last_exec_gas_cost();
    // let expected_gas: Gas = Gas::from(312402510 as u64);
    let expected_gas: Gas = Gas::from(479379480 as u64);
    assert_eq!(gas, expected_gas);
}
