use tauri::AppHandle;
use tokio::select;

use self::{
    connection::verify_active_connections, purge_stats::purge_stats, version::poll_version,
};

// Upstream also polls instance configuration here (enterprise config sync), which this build does
// not ship. Configuration changes arrive through the `update_instance` command instead.
pub mod connection;
pub mod purge_stats;
pub mod version;

/// Runs all the client periodic tasks, finishing when any of them returns.
pub async fn run_periodic_tasks(app_handle: &AppHandle) {
    debug!(
        "Starting periodic tasks (version polling, stats purging and active connection verification)..."
    );
    select! {
        () = poll_version(app_handle.clone()) => {
            error!("Version polling task has stopped unexpectedly");
        }
        () = verify_active_connections(app_handle.clone()) => {
            error!("Active connection verification task has stopped unexpectedly");
        }
        () = purge_stats() => {
            error!("Stats purging task has stopped unexpectedly");
        }
    };
}
