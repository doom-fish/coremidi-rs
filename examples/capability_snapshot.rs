use coremidi::capability::{discovered_ci_devices, legacy_ci_profile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let devices = discovered_ci_devices()?;
    println!("discovered_ci_devices={}", devices.len());
    match legacy_ci_profile(&[0, 0, 0, 0, 0], Some("Example Profile")) {
        Ok(profile) => println!("legacy_ci_profile name={} bytes={}", profile.name, profile.profile_id.len()),
        Err(error) => println!("legacy_ci_profile unavailable: {error}"),
    }
    Ok(())
}
