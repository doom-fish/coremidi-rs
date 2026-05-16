# CoreMIDI SDK coverage (v0.2.0)

This crate follows the v0.2.0 “100% Apple SDK coverage” playbook by splitting the CoreMIDI surface into logical areas, pairing each area with a Rust module, Swift bridge file when needed, an example, and an integration test.

## Logical-area coverage

| Area | Rust module | Swift bridge | Example | Test | Coverage notes |
| --- | --- | --- | --- | --- | --- |
| Client | `src/client.rs` | `Client.swift` | `examples/client_overview.rs` | `tests/client_area.rs` | Client creation, notification clients, restart |
| Endpoint | `src/endpoint.rs` | `Endpoint.swift` | `examples/endpoint_snapshot.rs` | `tests/endpoint_area.rs` | devices, entities, endpoints, virtual endpoints, UMP snapshots |
| Port | `src/port.rs` | `Port.swift` | `examples/port_create.rs` | `tests/port_area.rs` | input/output ports, protocol receive callbacks, flush |
| Packet / EventList | `src/packet.rs` | `PacketEventList.swift` | `examples/packet_buffers.rs` | `tests/packet_area.rs` | `MIDIPacketList`, `MIDIEventList`, iterators, MIDI 1.0 / 2.0 buffers |
| Notification | `src/notification.rs` | `Notification.swift` | `examples/notification_decode.rs` | `tests/notification_area.rs` | typed notification decoding from raw pointers and bridge JSON |
| Network | `src/network.rs` | `Network.swift` | `examples/network_session.rs` | `tests/network_area.rs` | `MIDINetworkSession`, hosts, connections, BLE MIDI helpers |
| Property | `src/property.rs` | `Property.swift` | `examples/property_lookup.rs` | `tests/property_area.rs` | string/int/data/dictionary/property-list getters/setters, lookup by unique ID |
| Driver | `src/driver.rs` | `Driver.swift` | `examples/driver_metadata.rs` | `tests/driver_area.rs` | driver interface IDs, driver-owned device helpers |
| ThruConnection | `src/thru_connection.rs` | `ThruConnection.swift` | `examples/thru_roundtrip.rs` | `tests/thru_connection_area.rs` | params initialize / serialize / deserialize / create / find |
| Setup | `src/setup.rs` | `Setup.swift` | `examples/setup_snapshot.rs` | `tests/setup_area.rs` | setup XML, serial-port ownership / drivers, device/entity setup APIs |
| Capability | `src/capability.rs` | `Capability.swift` | `examples/capability_snapshot.rs` | `tests/capability_area.rs` | MIDI-CI discovery snapshots plus legacy profile helper |

## Header / framework mapping

| Apple surface | Coverage path |
| --- | --- |
| `MIDIServices.h` | raw C declarations in `src/ffi/mod.rs` + safe wrappers in `client`, `endpoint`, `port`, `packet`, `property`, `notification` |
| `MIDISetup.h` | raw C declarations in `src/ffi/mod.rs` + safe wrappers in `setup` |
| `MIDIDriver.h` | raw C declarations in `src/ffi/mod.rs` + safe wrappers in `driver` |
| `MIDIThruConnection.h` | raw C declarations in `src/ffi/mod.rs` + safe wrappers in `thru_connection` |
| `MIDIBluetoothConnection.h` | Swift bridge in `Network.swift` + safe wrappers in `network` |
| `MIDINetworkSession.h` | Swift bridge in `Network.swift` + safe wrappers in `network` |
| `MIDIUMPEndpointManager.h` / related modern UMP classes | Swift bridge in `Endpoint.swift` + safe wrappers in `endpoint` |
| `MIDICapabilityInquiry.h` / MIDI-CI modern classes | Swift bridge in `Capability.swift` + safe snapshot helpers in `capability` |

## Raw C surface

- All declared CoreMIDI / CoreFoundation C symbols live in `src/ffi/mod.rs`.
- Public exposure of raw CoreMIDI functions is gated behind the `raw-ffi` Cargo feature.
- The expanded `tests/api_coverage.rs` audit compares the declared C symbol surface against the active macOS SDK headers:
  - `MIDIServices.h`
  - `MIDISetup.h`
  - `MIDIDriver.h`
  - `MIDIThruConnection.h`
  - `MIDIBluetoothConnection.h`

## Notes

- ObjC-only APIs (for example `MIDINetworkSession`, UMP endpoint manager classes, and modern MIDI-CI discovery objects) are bridged through Swift rather than exposed as raw Rust FFI.
- Some legacy setup / serial-port APIs are deprecated by Apple and can still report runtime status errors on current systems even though the symbols are bound and wrapped.
- Inline helper behavior is covered by Rust-side helpers such as `MIDIPacketNext`, `MIDIEventPacketNext`, and `MIDIThruConnectionParams::to_bytes` / `from_bytes`.
