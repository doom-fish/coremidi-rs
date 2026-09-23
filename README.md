# coremidi-rs

Safe Rust bindings for Apple's [CoreMIDI](https://developer.apple.com/documentation/coremidi) framework on macOS. The published Cargo package is `coremidi-rs`; the Rust library target is `coremidi`.

> **Status:** v0.6.0 fixes use-after-free and aliasing bugs in the packet buffers and callback contexts, and adds `MidiEventReceiver`, a receive path that neither locks nor allocates on the CoreMIDI receive thread.

Requires macOS 11 or newer. Some APIs need a newer system and return an error on older ones: UMP endpoint and MIDI-CI discovery APIs (macOS 15), `MIDIEventPacketSysexBytesForGroup` (macOS 14), Bluetooth MIDI connection helpers (macOS 13).

## Highlights

- Safe Rust wrappers for the practical CoreMIDI API surface.
- Swift bridge for ObjC-only and modern APIs such as network sessions, UMP endpoint snapshots, and MIDI-CI discovery.
- Raw CoreMIDI / CoreFoundation C symbols behind the `raw-ffi` Cargo feature.
- Examples and tests for every logical area requested in the v0.2.1 surface, including the remaining MIDI-CI / UMP audit closures.
- Header-audit coverage test and `COVERAGE.md` for tracking SDK parity.

## Modules by area

| Area | Rust module | Notes |
| --- | --- | --- |
| Client | `coremidi::client` | `MidiClient`, notification-capable clients, restart |
| Endpoint | `coremidi::endpoint` | devices, entities, endpoints, virtual endpoints, UMP snapshots, manager constants |
| Port | `coremidi::port` | input/output ports, protocol callbacks, flush |
| Receiver | `coremidi::receiver` | `MidiEventReceiver`, a preallocated ring of received events |
| Packet / EventList | `coremidi::packet` | `PacketListBuffer`, `EventListBuffer`, typed UMP enums / fixed-width helpers |
| Notification | `coremidi::notification` | typed CoreMIDI notification decoding |
| Network | `coremidi::network` | `MIDINetworkSession`, contacts, connections, BLE MIDI helpers |
| Property | `coremidi::property` | typed object/property helpers, lookup by unique ID |
| Driver | `coremidi::driver` | driver interface identifiers, driver-owned devices |
| ThruConnection | `coremidi::thru_connection` | parameter round-tripping and connection management |
| Setup | `coremidi::setup` | device/entity setup; deprecated setup-XML and serial-port queries |
| Capability | `coremidi::capability` | MIDI-CI discovery snapshots, manager constants, message-type enums, and legacy profile/state helpers |

## `raw-ffi` feature

By default, the crate exposes safe wrappers and raw CoreMIDI data types. To expose the raw C function symbols publicly, enable:

```toml
[dependencies]
coremidi-rs = { version = "0.6.0", features = ["raw-ffi"] }
```

Without `raw-ffi`, the raw function declarations stay crate-private and back the safe APIs.

## Quick start

```rust,no_run
use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MidiClient::new("coremidi demo")?;
    let source = client.virtual_source("demo source")?;
    let (destination, received) =
        client.virtual_destination_with_receiver("demo destination", MidiProtocol::Midi1, 256)?;
    let output = client.output_port("demo output")?;

    let mut packets = PacketListBuffer::with_capacity(1024);
    packets.add_packet(1, &[0x90, 60, 100])?;

    output.send(destination.endpoint(), &packets)?;
    source.received(&packets)?;

    for _ in 0..100 {
        if let Some(event) = received.try_next() {
            println!("received UMP words {:08x?}", event.words());
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}
```

## Receiving MIDI

`MidiClient::input_port_with_receiver` and `MidiClient::virtual_destination_with_receiver` return
the port or destination together with a `MidiEventReceiver`. The receiver is a ring of `capacity`
fixed-size `MidiEventRecord`s (timestamp, protocol, source endpoint and up to 64 UMP words),
allocated when it is created. The CoreMIDI receive thread copies each event packet into the ring
without locking or allocating; when the ring is full it overwrites the oldest record and counts it
in `dropped_count()`.

```rust,no_run
use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MidiClient::new("coremidi receiver")?;
    let (input, receiver) = client.input_port_with_receiver("input", MidiProtocol::Midi2, 512)?;
    for source in coremidi::sources() {
        input.connect(source)?;
    }

    loop {
        while let Some(event) = receiver.try_next() {
            println!("{} {:?}: {:08x?}", event.timestamp(), event.source(), event.words());
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
```

`receiver.next()` is a future that resolves to the next record, and to `None` once the port or
destination is gone. Awaiting it registers a waker that the receive thread wakes, which runs
executor code on that thread; poll `try_next()` instead if the receive thread must never do that.

The other receive paths:

- `MidiInputPort::connect_source_with_protocol_callback` (`unsafe`) calls a raw callback on the
  receive thread while the port holds its connection-table lock. `disconnect_source`, connecting the
  same source again, and dropping the port each wait for a running callback, so the `ref_con` can
  be freed once they return. The callback must not connect, disconnect or drop its own port.
- `MidiClient::input_port` and `MidiClient::virtual_destination` (`unsafe`) take the legacy
  `MIDIReadProc`; CoreMIDI may still be running it when `disconnect_source` or `Drop` returns.
- The async `MidiEventStream` and `MidiVirtualDestinationStream` copy every event list into a
  heap-allocated `OwnedEventList` on the receive thread.

## Async API

Enable the `async` feature to get executor-agnostic event streams backed by
[`doom-fish-utils`](https://github.com/doom-fish/doom-fish-utils):

```toml
[dependencies]
coremidi-rs = { version = "0.6", features = ["async"] }
```

| Stream type | Item | Source |
|---|---|---|
| `MidiEventStream` | `OwnedEventList` (allocated on the receive thread) | `MIDIInputPortCreateWithProtocol` receive block |
| `MidiVirtualDestinationStream` | `OwnedEventList` (allocated on the receive thread) | `MIDIDestinationCreateWithProtocol` receive block |
| `MidiClientNotificationStream` | `Notification` | `MIDIClientCreateWithBlock` notification block |
| `MidiCIDiscoveryStream` | `Vec<CiDeviceInfo>` | `MIDICIDeviceManager` KVO (macOS 15+ in the current SDK) |
| `MidiThruConnectionStream` | `()` | `ThruConnectionsChanged` notification |

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
- `cargo run --example 14_async_streams --features async`

## Validation

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

The tests use virtual endpoints, so they need no MIDI hardware. `MIDIServer` exits as soon as no
process holds a CoreMIDI connection; a process that connects while it is exiting keeps failing
with status -304, so each test binary re-executes itself once when its first connection fails.

## Notes

- `current_setup_xml`, `serial_port_owner` and `serial_port_drivers` are `#[deprecated]`: they wrap APIs Apple marks "No longer supported" since macOS 10.6, and they return status -4 on current systems. `device_add_entity_deprecated` is deprecated in favour of `device_new_entity`.
- Modern UMP / MIDI-CI objects are availability-gated by macOS and return empty snapshots or framework errors when unavailable.
- Client notifications (`MidiClient::with_notification_handler`, `MidiClientNotificationStream`, `MidiThruConnectionStream`) arrive on the run loop of the thread that created the process's first CoreMIDI client, as the header documents for `MIDINotifyProc` (observed on macOS 27 for the block-based API too), so that run loop must be running.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
