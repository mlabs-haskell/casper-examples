use hex::encode;
use odra::client_env;
use odra::types::{Address, CallArgs, OdraType};

use contracts::{deployed_contracts::DeployedGovernor, governor::GovernorDeployer};
const DEPLOY_COST: u64 = 155_000_000_000;
const PROPOSAL_COST: u64 = 4_000_000_000;

fn main() {
    // panic!("Disabled");
    client_env::set_gas(DEPLOY_COST);
    let mut governor = GovernorDeployer::init("test".to_string());

    let mut args = CallArgs::new();
    args.insert("add_amount", 1i32);

    let contract_data = (
        "hash-c41dd9173ba2edf1f61552e7443607b39eec112266ef28aac514e9a87b240e20".into(),
        ("counter_inc".into(), encode(&args.serialize().unwrap())),
    );

    client_env::set_gas(PROPOSAL_COST);
    governor.new_proposal("First proposal".to_string(), contract_data.clone());

    client_env::set_gas(PROPOSAL_COST);
    governor.new_proposal("Second proposal".to_string(), contract_data.clone());

    let p0 = governor.get_proposal(0);
    println!("Proposal 0: {:#?}", p0);

    DeployedGovernor::new(governor.address().to_string()).save_to_file("./../governor.json");
}
