use coremidi::thru_connection::ThruConnectionParams;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = ThruConnectionParams::default();
    let bytes = params.to_bytes()?;
    let decoded = ThruConnectionParams::from_bytes(&bytes)?;
    println!(
        "thru_params bytes={} sources={} destinations={} maps={}",
        bytes.len(),
        decoded.sources.len(),
        decoded.destinations.len(),
        decoded.maps.len(),
    );
    Ok(())
}
