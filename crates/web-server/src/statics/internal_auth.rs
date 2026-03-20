use std::sync::OnceLock;

use crate::settings::InternalAuthConfig;

static INTERNAL_AUTH_CONFIG: OnceLock<InternalAuthConfig> = OnceLock::new();

pub fn init_internal_auth_config(cfg: InternalAuthConfig) {
    INTERNAL_AUTH_CONFIG
        .set(cfg)
        .expect("internal auth config already initialized");
}

pub fn get_internal_auth_config() -> &'static InternalAuthConfig {
    INTERNAL_AUTH_CONFIG
        .get()
        .expect("internal auth config not initialized")
}
