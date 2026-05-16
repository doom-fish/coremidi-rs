use coremidi::capability::{
    ci_device_manager_constants, discovered_ci_devices, legacy_ci_profile, CiManagementMessageType,
    CiProcessInquiryMessageType, CiProfileMessageType, CiProfileState,
    CiPropertyExchangeMessageType, LegacyCiProfileInfo,
};

#[test]
fn capability_area_snapshot_calls_work() -> coremidi::MidiResult<()> {
    assert!(discovered_ci_devices().is_ok());

    let constants = ci_device_manager_constants()?;
    assert!(constants.device_object_key.is_empty() || constants.device_object_key.contains("MIDICIDevice"));
    assert_eq!(CiManagementMessageType::Discovery.as_raw(), 0x70);
    assert_eq!(CiProcessInquiryMessageType::InquiryMidiMessageReport.as_raw(), 0x42);
    assert_eq!(CiProfileMessageType::ProfileInquiry.as_raw(), 0x20);
    assert_eq!(CiPropertyExchangeMessageType::Notify.as_raw(), 0x3F);

    if let Ok(profile) = legacy_ci_profile(&[0x7E, 0, 0, 0, 0], Some("Example Profile")) {
        assert_eq!(profile.profile_id.len(), 5);
    }

    let example_profile = LegacyCiProfileInfo {
        name: String::from("Example Profile"),
        profile_id: vec![0x7E, 0, 0, 0, 0],
    };
    let state = CiProfileState::new(Some(3), std::slice::from_ref(&example_profile), &[])?;
    let snapshot = state.snapshot()?;
    assert_eq!(snapshot.midi_channel, 3);
    assert_eq!(snapshot.enabled_profiles.len(), 1);
    assert_eq!(snapshot.enabled_profiles[0], example_profile);
    assert!(snapshot.disabled_profiles.is_empty());
    Ok(())
}
