# Changelog

## [0.1.0] - 2026-05-16

### Added

- Initial `coremidi-rs` release for macOS MIDI client I/O.
- `MidiClient`, `MidiInputPort`, `MidiOutputPort`, `VirtualSource`, and `VirtualDestination` wrappers.
- Device / entity / endpoint enumeration plus object-property helpers for names, manufacturer, model, and unique IDs.
- `PacketListBuffer` for safe `MIDIPacketListInit` / `MIDIPacketListAdd` construction and packet iteration.
- `EventListBuffer` for MIDI 2.0 `MIDIEventListInit` / `MIDIEventListAdd` construction and event iteration.
- Raw CoreMIDI FFI exports for the v0.1 surface, including `MIDIPacket`, `MIDIPacketList`, `MIDIEventPacket`, `MIDIEventList`, and `MIDIUniversalMessage`.
- Smoke example `examples/01_loopback_smoke.rs` covering virtual-source injection and virtual-destination verification.
- Header-audit test `tests/api_coverage.rs` to verify the declared CoreMIDI symbol set against the active SDK.
