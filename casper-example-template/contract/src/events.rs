extern crate alloc;
use alloc::string::{String};


use casper_event_standard::{Event, Schemas};

#[derive(Event, Debug, PartialEq)]
pub struct SomeEvent {
    pub message: String,
}

pub (crate) fn init_events() {
    let schemas = Schemas::new().with::<SomeEvent>();
    casper_event_standard::init(schemas);
}