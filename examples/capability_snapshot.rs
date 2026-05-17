use coremidi::capability::{
    ci_device_manager_constants, discovered_ci_devices, legacy_ci_profile, CiManagementMessageType,
    CiProfileState, LegacyCiProfileInfo,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let constants = ci_device_manager_constants()?;
    let devices = discovered_ci_devices()?;
    println!(
        "discovered_ci_devices={} device_added_notification={} profile_object_key={} management_discovery={:#x}",
        devices.len(),
        constants.device_added_notification,
        constants.profile_object_key,
        CiManagementMessageType::Discovery.as_raw(),
    );

    let example_profile = legacy_ci_profile(&[0x7E, 0, 0, 0, 0], Some("Example Profile"))
        .unwrap_or_else(|_| LegacyCiProfileInfo {
            name: String::from("Example Profile"),
            profile_id: vec![0x7E, 0, 0, 0, 0],
        });
    let state = CiProfileState::new(Some(0), std::slice::from_ref(&example_profile), &[])?;
    let snapshot = state.snapshot()?;
    println!(
        "profile_state channel={} enabled_profiles={} first_name={}",
        snapshot.midi_channel,
        snapshot.enabled_profiles.len(),
        snapshot
            .enabled_profiles
            .first()
            .map_or("", |profile| profile.name.as_str()),
    );
    Ok(())
}
