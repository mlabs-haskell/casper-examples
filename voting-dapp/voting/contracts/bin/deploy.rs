use odra::client_env;
use odra::types::Address;
use std::str::FromStr;

use contracts::{deployed_contracts::DeployedGovernor, governor::GovernorDeployer};
const DEPLOY_COST: u64 = 155_000_000_000;

fn main() {
    // panic!("Disabled");
    //todo: get name from CLI
    let name = String::from("Test name");
    client_env::set_gas(DEPLOY_COST);
    let contract = GovernorDeployer::init(name);
    let address = contract.address();
    DeployedGovernor::new(contract.address().to_string()).save_to_file("./../governor.json")
}
