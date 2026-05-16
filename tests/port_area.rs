use coremidi::prelude::*;

#[test]
fn port_area_create_ports() -> MidiResult<()> {
    let client = MidiClient::new("port area smoke")?;
    let output = client.output_port("port area output")?;
    let input = client.input_port_with_protocol("port area input", MidiProtocol::Midi1)?;
    assert_ne!(output.raw(), 0);
    assert_ne!(input.raw(), 0);
    Ok(())
}
