use crate::state::ServiceState;
use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceVersions {
    gamespace: &'static str,
    runtime: &'static str,
    server: &'static str
}

impl Default for ServiceVersions {
    fn default() -> Self {
        Self {
            gamespace: aurorite_agsp::VERSION,
            runtime: aurorite_runtime::VERSION,
            server: crate::VERSION
        }
    }
}

#[derive(Serialize)]
pub struct ServiceInfo {
    state: ServiceState,
    modules: ServiceVersions
}

impl Default for ServiceInfo {
    fn default() -> Self {
        Self {
            state: ServiceState::Stale,
            modules: ServiceVersions::default()
        }
    }
}

