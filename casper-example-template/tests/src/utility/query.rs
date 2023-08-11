use casper_engine_test_support::InMemoryWasmTestBuilder;
use casper_types::{account::AccountHash, bytesrepr::FromBytes, CLTyped, Key};

use super::misc::get_contract;

pub fn query_key<R>(builder: &InMemoryWasmTestBuilder, some_key: Key) -> R
where
    R: CLTyped + FromBytes,
{
    builder
        .query(None, some_key, &[])
        .expect(format!("Should be able to query {:#?}", some_key).as_str())
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t::<R>()
        .expect(format!("Could not into {}", std::any::type_name::<R>()).as_str())
}

pub fn named_dictionary<R>(
    builder: &InMemoryWasmTestBuilder,
    account_hash: AccountHash,
    dict_name: &str,
    some_key: &str,
) -> R
where
    R: CLTyped + FromBytes,
{
    let contract = get_contract(builder, account_hash);
    let dict_seed_uref = *contract
        .named_keys()
        .get_key_value(dict_name)
        .expect(format!("should return key-value pair for key `{}`", dict_name).as_str())
        .1
        .as_uref()
        .expect("should be URef");

    builder
        .query_dictionary_item(None, dict_seed_uref, some_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}
