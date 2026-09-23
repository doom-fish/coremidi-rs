mod common;

use coremidi::prelude::*;

#[test]
fn client_area_smoke() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("client area smoke")?;
    let output = client.output_port("client area output")?;
    assert_ne!(client.raw(), 0);
    assert_ne!(output.raw(), 0);
    Ok(())
}
