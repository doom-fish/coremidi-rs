use coremidi::network::NetworkSession;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = NetworkSession::default();
    let constants = NetworkSession::constants()?;
    println!(
        "bonjour_service_type={} enabled={} policy={:?} contacts={} connections={}",
        constants.bonjour_service_type,
        session.is_enabled(),
        session.connection_policy()?,
        session.contacts()?.len(),
        session.connections()?.len(),
    );
    Ok(())
}
