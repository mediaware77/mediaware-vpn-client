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
        use LocationMfaMode::{Disabled, External, Internal};
        use ServiceLocationMode::{AlwaysOn, Disabled as Regular, PreLogon};
        let cases = [
            (Disabled, Regular, false, true),
            (Internal, Regular, false, false),
            (External, Regular, false, false),
            (Disabled, Regular, true, false),
            (Disabled, PreLogon, false, false),
            (Disabled, AlwaysOn, false, false),
        ];
        for (mfa, service, posture, expected) in cases {
            assert_eq!(
                policy_allows(mfa, service, posture),
                expected,
                "{mfa:?}/{service:?}/posture={posture}"
            );
        }
    }
}
