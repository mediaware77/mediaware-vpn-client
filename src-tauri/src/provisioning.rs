use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// Zero-touch provisioning is an enterprise feature of the upstream client and is not part of
/// this build. The type and the startup hook stay so the frontend contract does not change; the
/// hook never finds a configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProvisioningConfig {
    pub enrollment_url: String,
    pub enrollment_token: String,
}

pub async fn handle_client_initialization(_app_handle: &AppHandle) -> Option<ProvisioningConfig> {
    None
}
