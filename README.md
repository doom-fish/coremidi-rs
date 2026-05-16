# coremidi-rs

Safe Rust bindings for Apple's [CoreMIDI](https://developer.apple.com/documentation/coremidi) framework — MIDI clients, ports, virtual endpoints, packets, and MIDI 2.0 event lists on macOS. The published Cargo package is `coremidi-rs`; the Rust library target is `coremidi`.

> **Status:** v0.1.0 ships the practical CoreMIDI surface for client setup, input/output ports, virtual sources and destinations, device/entity enumeration, property lookup, legacy `MIDIPacketList` sending, and MIDI 2.0 `MIDIEventList` construction.

## Highlights

- `MidiClient`, `MidiInputPort`, `MidiOutputPort`, `VirtualSource`, `VirtualDestination`
- Device / entity / endpoint enumeration via `devices()`, `MidiDevice::entities()`, `MidiEntity::source()`, and `MidiEntity::destination()`
- Object property helpers for `name`, `manufacturer`, `model`, `unique_id`, plus generic `string_property` / `integer_property`
- `PacketListBuffer` for safe `MIDIPacketListInit` / `MIDIPacketListAdd` packet construction
- `EventListBuffer` for MIDI 2.0 `MIDIEventListInit` / `MIDIEventListAdd`
- Raw CoreMIDI structs re-exported: `MIDIPacket`, `MIDIPacketList`, `MIDIEventPacket`, `MIDIEventList`, `MIDIUniversalMessage`
- Direct C callbacks (`MIDIReadProc`) for receive paths — no Swift bridge required

## Quick start

```rust,no_run
use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MidiClient::new("coremidi demo")?;
    let source = client.virtual_source("demo source")?;
    let destination = unsafe {
        client.virtual_destination(
            "demo destination",
            None,
            std::ptr::null_mut(),
        )?
    };
    let output = client.output_port("demo output")?;

    let mut packets = PacketListBuffer::with_capacity(1024);
    packets.add_packet(1, &[0x90, 60, 100])?;
    output.send(destination.endpoint(), &packets)?;
    source.received(&packets)?;
    Ok(())
}
```

## Surface overview

### Client + port management

- `MidiClient::new`, `MidiClient::with_notify`
- `MidiClient::input_port`, `MidiClient::input_port_with_protocol`, `MidiClient::output_port`
- `MidiClient::virtual_source`, `MidiClient::virtual_destination`
- `MidiInputPort::connect_source`, `connect_source_with_protocol_callback`, `disconnect_source`
- `MidiOutputPort::send`, `send_event_list`
- `VirtualSource::received`, `received_event_list`

### Enumeration + properties

- `device_count`, `device`, `devices`
- `MidiDevice::entity_count`, `entity`, `entities`
- `MidiEntity::source_count`, `source`, `destination_count`, `destination`
- `MidiObject::string_property`, `integer_property`, `name`, `manufacturer`, `model`, `unique_id`

### Packet construction

- `PacketListBuffer::with_capacity`, `add_packet`, `clear`
- `PacketListRef::iter`
- `EventListBuffer::with_capacity`, `add_packet_words`, `clear`
- `EventListRef::iter`
- Raw structs: `MIDIPacket`, `MIDIPacketList`, `MIDIEventPacket`, `MIDIEventList`, `MIDIUniversalMessage`

## Smoke example

Run the CoreMIDI loopback smoke example with:

```bash
cargo run --example 01_loopback_smoke
```

It creates a client, a virtual source, a virtual destination, and an input/output loopback path. A Middle-C Note On packet is injected into the virtual source, forwarded to the virtual destination, and verified from the destination callback before printing `✅ coremidi loopback OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
