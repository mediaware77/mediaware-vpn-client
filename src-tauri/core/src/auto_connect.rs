//! Policy shared by Windows startup and its settings command.
use crate::database::models::{
    location::{Location, LocationMfaMode, ServiceLocationMode},
    Id,
};

pub fn eligible(location: &Location<Id>) -> bool {
    policy_allows(
        location.location_mfa_mode,
        location.service_location_mode,
        location.posture_check_required,
    )
}

fn policy_allows(mfa: LocationMfaMode, service: ServiceLocationMode, posture: bool) -> bool {
    mfa == LocationMfaMode::Disabled && service == ServiceLocationMode::Disabled && !posture
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_only_regular_locations_without_authentication_requirements() {
        for mfa in [
            LocationMfaMode::Disabled,
            LocationMfaMode::Internal,
            LocationMfaMode::External,
        ] {
            for service in [
                ServiceLocationMode::Disabled,
                ServiceLocationMode::PreLogon,
                ServiceLocationMode::AlwaysOn,
            ] {
                for posture in [false, true] {
                    assert_eq!(
                        policy_allows(mfa, service, posture),
                        mfa == LocationMfaMode::Disabled
                            && service == ServiceLocationMode::Disabled
                            && !posture
                    );
                }
            }
        }
    }
}
