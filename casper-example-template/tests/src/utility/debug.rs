use casper_engine_test_support::InMemoryWasmTestBuilder;
use casper_types::account::AccountHash;

pub(crate) fn print_keys(builder: &InMemoryWasmTestBuilder, account_addr: AccountHash) -> () {
    let account = builder.get_expected_account(account_addr);
    let account_named_keys = account.named_keys();
    println!(
        "Named keys for {}:\n{:#?}",
        account_addr, &account_named_keys
    );
}
