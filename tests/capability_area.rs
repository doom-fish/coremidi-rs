use coremidi::capability::{discovered_ci_devices, legacy_ci_profile};

#[test]
fn capability_area_snapshot_calls_work() {
    assert!(discovered_ci_devices().is_ok());
    if let Ok(profile) = legacy_ci_profile(&[0, 0, 0, 0, 0], Some("Example Profile")) {
        assert_eq!(profile.profile_id.len(), 5);
    }
}
