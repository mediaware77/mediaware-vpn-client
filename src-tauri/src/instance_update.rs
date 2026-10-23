//! Applies a device configuration fetched from the Edge to an instance that is already enrolled.
//!
//! Upstream keeps this logic in its enterprise config-sync module, which is not part of this
//! build. This is an independent implementation on top of the open data models.

use defguard_client_core::database::models::{
    instance::Instance,
    location::{infer_mfa_method, Location},
    Id,
};
use defguard_client_proto::defguard::client_types::DeviceConfigResponse;
use sqlx::{Sqlite, Transaction};

use crate::{error::Error, into_location};

/// Updates `instance` and its locations from `response`.
///
/// Locations are matched by their network ID: new ones are added, missing ones are removed and
/// the rest are updated. Settings chosen by the user (route all traffic, MFA method) are kept.
/// Returns whether any location was added, removed or changed.
pub async fn apply_device_config(
    transaction: &mut Transaction<'_, Sqlite>,
    instance: &mut Instance<Id>,
    response: DeviceConfigResponse,
) -> Result<bool, Error> {
    if let Some(info) = response.instance {
        let fresh = Instance::from(info);
        instance.name = fresh.name;
        instance.uuid = fresh.uuid;
        instance.url = fresh.url;
        instance.proxy_url = fresh.proxy_url;
        instance.username = fresh.username;
        instance.client_traffic_policy = fresh.client_traffic_policy;
        instance.enterprise_enabled = fresh.enterprise_enabled;
        instance.disable_tunnels = fresh.disable_tunnels;
        instance.openid_display_name = fresh.openid_display_name;
    }
    if response.token.is_some() {
        instance.token = response.token;
    }
    instance.save(&mut **transaction).await?;

    let mut current = Location::find_by_instance_id(&mut **transaction, instance.id, true).await?;
    let mut changed = false;

    for config in response.configs {
        let incoming = into_location(config, instance.id);
        let Some(position) = current
            .iter()
            .position(|location| location.network_id == incoming.network_id)
        else {
            debug!("Adding location {} to instance {}", incoming.name, instance.name);
            incoming.save(&mut **transaction).await?;
            changed = true;
            continue;
        };

        let mut location = current.swap_remove(position);
        let before = location.clone();
        location.name = incoming.name;
        location.address = incoming.address;
        location.pubkey = incoming.pubkey;
        location.endpoint = incoming.endpoint;
        location.allowed_ips = incoming.allowed_ips;
        location.dns = incoming.dns;
        location.keepalive_interval = incoming.keepalive_interval;
        location.location_mfa_mode = incoming.location_mfa_mode;
        location.service_location_mode = incoming.service_location_mode;
        location.posture_check_required = incoming.posture_check_required;
        location.mfa_method = infer_mfa_method(location.location_mfa_mode, location.mfa_method);

        if location != before {
            debug!("Updating location {location}");
            location.save(&mut **transaction).await?;
            changed = true;
        }
    }

    for stale in current {
        debug!("Removing location {stale}, no longer present in the instance config");
        stale.delete(&mut **transaction).await?;
        changed = true;
    }

    Ok(changed)
}
