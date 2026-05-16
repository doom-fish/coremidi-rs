use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MidiClient::new("coremidi client overview")?;
    let output = client.output_port("coremidi client overview output")?;
    println!("client={} output_port={}", client.raw(), output.raw());
    Ok(())
}
