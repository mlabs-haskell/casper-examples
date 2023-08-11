use casper_engine_test_support::ExecuteRequestBuilder;
use casper_event_standard::casper_types::bytesrepr::{ToBytes, FromBytes};
use casper_types::{runtime_args, RuntimeArgs, bytesrepr::Bytes};

use crate::utility::{misc::{self, get_contract}, debug};
use contract::events::SomeEvent;
use contract::constants;

#[test]
fn event_emitted() {
    let (account_addr, mut builder) = misc::deploy_contract();

    let call_emit_event = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        "emit_event",
        runtime_args! {constants::events::SOME_EVENT_MSG => "message-1"},
    )
    .build();
    builder.exec(call_emit_event).expect_success().commit();

    let contract = get_contract(&builder, account_addr);
    let seed_uref = *contract
        .named_keys()
        .get(casper_event_standard::EVENTS_DICT)
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");

    let stored_event_bytes: Bytes = builder
        .query_dictionary_item(None, seed_uref, "0")
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap();

    let expected_event = SomeEvent {
        message: String::from("message-1"),
    };

    let stored_event = SomeEvent::from_bytes(&stored_event_bytes).unwrap().0;
    assert_eq!(expected_event, stored_event);

}
