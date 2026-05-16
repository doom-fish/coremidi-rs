# coremidi-rs

Safe Rust bindings for Apple's [CoreMIDI](https://developer.apple.com/documentation/coremidi) framework on macOS. The published Cargo package is `coremidi-rs`; the Rust library target is `coremidi`.

> **Status:** v0.2.0 expands the crate from basic client/port I/O to a multi-area CoreMIDI surface covering clients, endpoints, ports, packets / event lists, notifications, networking, properties, drivers, thru connections, setup APIs, and MIDI-CI / UMP capability snapshots.

## Highlights

- Safe Rust wrappers for the practical CoreMIDI API surface.
- Swift bridge for ObjC-only and modern APIs such as network sessions, UMP endpoint snapshots, and MIDI-CI discovery.
- Raw CoreMIDI / CoreFoundation C symbols behind the `raw-ffi` Cargo feature.
- Examples and tests for every logical area requested in the v0.2.0 expansion.
- Header-audit coverage test and `COVERAGE.md` for tracking SDK parity.

## Modules by area

| Area | Rust module | Notes |
| --- | --- | --- |
| Client | `coremidi::client` | `MidiClient`, notification-capable clients, restart |
| Endpoint | `coremidi::endpoint` | devices, entities, endpoints, virtual endpoints, UMP snapshots |
| Port | `coremidi::port` | input/output ports, protocol callbacks, flush |
| Packet / EventList | `coremidi::packet` | `PacketListBuffer`, `EventListBuffer`, iterators |
| Notification | `coremidi::notification` | typed CoreMIDI notification decoding |
| Network | `coremidi::network` | `MIDINetworkSession`, contacts, connections, BLE MIDI helpers |
| Property | `coremidi::property` | typed object/property helpers, lookup by unique ID |
| Driver | `coremidi::driver` | driver interface identifiers, driver-owned devices |
| ThruConnection | `coremidi::thru_connection` | parameter round-tripping and connection management |
| Setup | `coremidi::setup` | setup XML, serial-port driver queries, device/entity setup |
| Capability | `coremidi::capability` | MIDI-CI discovery snapshots and legacy profile helper |

## `raw-ffi` feature

By default, the crate exposes safe wrappers and raw CoreMIDI data types. To expose the raw C function symbols publicly, enable:

```toml
[dependencies]
coremidi-rs = { version = "0.2.0", features = ["raw-ffi"] }
```

Without `raw-ffi`, the raw function declarations stay crate-private and back the safe APIs.

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

## Examples

- `cargo run --example 01_loopback_smoke`
- `cargo run --example client_overview`
- `cargo run --example endpoint_snapshot`
- `cargo run --example port_create`
- `cargo run --example packet_buffers`
- `cargo run --example notification_decode`
- `cargo run --example network_session`
- `cargo run --example property_lookup`
- `cargo run --example driver_metadata`
- `cargo run --example thru_roundtrip`
- `cargo run --example setup_snapshot`
- `cargo run --example capability_snapshot`

## Validation

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

## Notes

- Some legacy setup / serial-port APIs are deprecated by Apple and may return framework status errors on modern systems even though the symbols remain bound.
- Modern UMP / MIDI-CI objects are availability-gated by macOS and return empty snapshots or framework errors when unavailable.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
