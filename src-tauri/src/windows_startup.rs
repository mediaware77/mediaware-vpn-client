//! Connect an enrolled location when the app is launched at Windows logon.
use crate::{
    appstate::AppState,
    commands,
    database::{models::location::Location, DB_POOL},
    ConnectionType,
};
pub use defguard_client_core::windows_startup::{launched_at_logon, set_enabled};
use defguard_client_core::{
    auto_connect::eligible, connection::active_connections::find_connection,
};
use std::time::Duration;
use tauri::{AppHandle, Manager};

/// Retry startup failures while the network/service becomes available. Stop after a successful
/// connection: a later manual disconnect must stay disconnected until the next launch.
pub async fn connect_at_logon(handle: AppHandle) {
    let cancellation = handle.state::<AppState>().startup_cancel.clone();
    for _ in 0..60 {
        if cancellation.is_cancelled() {
            return;
        }
        let id = handle
            .state::<AppState>()
            .app_config
            .lock()
            .unwrap()
            .auto_connect_location_id;
        if id == 0 {
            return;
        }
        let location = match Location::find_by_id(&*DB_POOL, id).await {
            Ok(Some(location)) if eligible(&location) => location,
            Ok(_) => {
                warn!("Automatic connection skipped: selected location is missing or requires MFA/posture");
                return;
            }
            Err(error) => {
                warn!("Automatic connection could not load location: {error}");
                return;
            }
        };
        if find_connection(id, ConnectionType::Location)
            .await
            .is_some()
        {
            return;
        }
        match commands::connect(location.id, ConnectionType::Location, handle.clone()).await {
            Ok(()) => {
                info!("Automatic Windows logon connection established");
                return;
            }
            Err(error) => warn!("Automatic Windows logon connection will retry: {error}"),
        }
        tokio::select! {
            _ = cancellation.cancelled() => return,
            _ = tokio::time::sleep(Duration::from_secs(10)) => {},
        }
    }
    warn!("Automatic Windows logon connection exhausted its retries; connect manually");
}
