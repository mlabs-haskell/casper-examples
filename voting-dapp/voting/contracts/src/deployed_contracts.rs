use odra::types::Address;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployedGovernor {
    package_key: String,
    package_hash: String,
}

impl DeployedGovernor {
    pub fn new(package_hash: String) -> Self {
        DeployedGovernor {
            package_key: "governor_package_hash".to_string(),
            package_hash,
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let res = serde_json::to_string(self).unwrap();
        fs::write(path, res).unwrap();
    }

    pub fn load_from_file(path: &str) -> Self {
        let governor_json = fs::read_to_string(path).expect("Should read governor data from file");
        serde_json::from_str(&governor_json).expect("Should parse JSON with contract data")
    }

    pub fn get_package_hash(&self) -> &str {
        &self.package_hash
    }

    pub fn get_package_hash_address(&self) -> Address {
        Address::from_str(self.get_package_hash()).expect("Should be able to parse address from {}")
    }
}
