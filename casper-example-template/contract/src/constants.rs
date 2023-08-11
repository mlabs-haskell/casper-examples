pub mod contract {
    pub const PACKAGE_NAME: &str = "add_with_registry_package";
    pub const ACCESS_UREF: &str = "add_with_registry_uref";
    pub const KEY: &str = "add_with_registry_contract_key";
    pub const VERSION_KEY: &str = "add_with_registry_version";
}

pub mod init {
    // endoint value should match with "nf_name" in  "pub extern "C" fn nf_name"
    pub const ENTRYPOINT: &str = "init";
}

pub mod registry {
    pub const ENTRYPOINT: &str = "register_user_key";
    pub const DICT: &str = "contract_dict";
    pub const REGISTRY_MAP: &str = "registry_map";
}

pub mod append {
    pub const ENTRYPOINT: &str = "append_phrase";
    pub const ARG: &str = "what_to_append";
    pub const ACCUM_VALUE: &str = "accumulator_value";
}

pub mod events {
    pub const SOME_EVENT_MSG: &str = "some_event_message";
}
