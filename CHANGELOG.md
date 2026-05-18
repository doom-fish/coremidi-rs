# Changelog

## [0.5.0] - 2026-05-18

### Changed

- Re-export `CFIndex` and `CFTypeID` from `apple_cf::raw` instead of duplicating Core Foundation raw type aliases locally in `src/ffi/mod.rs`.
- Raised the `apple-cf` dependency range to `>=0.9, <0.10` to align with the shared raw definitions.

## [0.4.0] - 2026-05-18

### Changed

- Re-export Core Foundation `CF*Ref` aliases from `apple_cf::raw` instead of duplicating local typedefs in `src/ffi/mod.rs`, so the crate now follows the shared raw definitions for `CFTypeRef`, `CFStringRef`, `CFAllocatorRef`, `CFDataRef`, `CFDictionaryRef`, `CFArrayRef`, `CFPropertyListRef`, `CFRunLoopRef`, and `CFUUIDRef`.
- Raised the minimum `apple-cf` dependency to `0.8` because the raw re-exports now come from `apple_cf::raw`.

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.3.1] - 2026-05-18

### Fixed

- `protocol_receive_block_invoke` (port.rs): wrapped user-callback invocation in
  `catch_unwind` so a panic from a `MidiProtocolReadProc` cannot unwind across the
  C ABI boundary into the CoreMIDI server thread (was UB, now aborts cleanly).
- `OwnedEventList::copy_from`: added `# Real-time safety` doc section documenting
  the heap allocation that occurs on the CoreMIDI real-time server thread when this
  is called from `MidiEventStream` or `MidiVirtualDestinationStream`; added a note
  pointing to the low-allocation alternative (raw `MidiProtocolReadProc`).
- `copy_event_list_to_sender` (async_api.rs): added real-time allocation note in a
  `SAFETY:` comment.
- Added `SAFETY:` comments to all hot-path `unsafe` blocks in `async_api.rs`,
  `port.rs`, and `client.rs` (Drop implementations, callback trampolines, and
  raw-pointer casts).
- `MidiClientNotificationStream::drop` and `MidiClient::drop`: added comments
  documenting the `MIDIRestart` in-flight callback race and the ordering guarantee
  (Swift object released before sender/context freed).
- `doom-fish-utils` dependency version range widened from `"0.1"` to `">=0.1, <0.3"`
  to allow the next minor bump without a breaking change.

## [0.3.0] - 2026-05-17

### Added

- `async` Cargo feature gate backed by `doom-fish-utils::stream::BoundedAsyncStream<T>`.
- `async_api` module with 5 executor-agnostic stream surfaces:
  - `MidiEventStream` — async `OwnedEventList` stream from `MIDIInputPortCreateWithProtocol` receive block.
  - `MidiVirtualDestinationStream` — async `OwnedEventList` stream from `MIDIDestinationCreateWithProtocol`.
  - `MidiClientNotificationStream` — async `Notification` stream from `MIDIClientCreateWithBlock` notifications.
  - `MidiCIDiscoveryStream` — async `Vec<CiDeviceInfo>` stream from `MIDICIDeviceManager` KVO (macOS 15+ in the current SDK; `subscribe()` returns `None` on older systems).
  - `MidiThruConnectionStream` — async `()` stream that fires on each `ThruConnectionsChanged` notification.
- `OwnedEventList` value type (protocol + batched `Vec<MIDIEventPacket>`) to preserve bulk-buffer semantics.
- `swift-bridge/Sources/CoreMIDIBridge/AsyncStream.swift` with Swift thunks for virtual destination and CI discovery streams.
- Example `examples/14_async_streams.rs` and integration tests `tests/async_stream_tests.rs`.

## [0.2.1] - 2026-05-16

### Added

- `capability::ci_device_manager_constants`, `CiDeviceManagerConstants`, `CiProfileState`, and `CiProfileStateInfo` to surface MIDICIDeviceManager notifications / user-info keys plus per-channel legacy MIDI-CI profile-state snapshots.
- Typed MIDI-CI message sub-ID enums: `CiManagementMessageType`, `CiProcessInquiryMessageType`, `CiProfileMessageType`, and `CiPropertyExchangeMessageType`.
- `endpoint::UmpEndpointManager::constants` and `UmpEndpointManagerConstants` for MIDIUMPEndpointManager notifications / user-info keys.
- Typed UMP helper surface in `packet`: `MidiMessageType`, `MidiCvStatus`, `MidiSystemStatus`, `MidiSysExStatus`, `MidiUtilityStatus`, `UmpStreamMessageFormat`, `UmpStreamMessageStatus`, `MidiNoteAttribute`, `MidiProgramChangeOptions`, `MidiPerNoteManagementOptions`, and fixed-width `MidiMessage64` / `MidiMessage96` / `MidiMessage128` wrappers backed by raw `ffi::MIDIMessage_*` structs.

### Changed

- Closed all 30 remaining `COVERAGE_AUDIT.md` gaps and raised the tracked SDK coverage from 85.44% to 100.00%.
- Refreshed the README, examples, tests, and coverage docs for the expanded v0.2.1 surface.

## [0.2.0] - 2026-05-16

### Added

- Swift bridge build pipeline for CoreMIDI, including Swift-backed client notifications, network session access, UMP endpoint snapshots, thru-connection helpers, and MIDI-CI discovery helpers.
- New logical-area modules: `endpoint`, `port`, `notification`, `network`, `property`, `driver`, `thru_connection`, `setup`, and `capability`.
- Expanded safe wrappers for system and external device enumeration, property setters/getters, unique-ID lookup, driver-owned devices, setup/device/entity management, and thru-connection parameter round-tripping.
- Modern UMP helpers: `Midi2DeviceInfoHandle`, `MutableUmpFunctionBlock`, `MutableUmpEndpoint`, and `UmpEndpointManager` snapshots.
- Capability helpers for discovered MIDI-CI devices and legacy profile decoding.
- `raw-ffi` feature to publicly expose the raw CoreMIDI C function surface while keeping those declarations crate-private by default.
- One example and one integration test for each requested logical area, plus an expanded `tests/api_coverage.rs` header-audit test.
- `COVERAGE.md` documenting the v0.2.0 SDK coverage strategy and header mapping.

### Changed

- Reworked the public crate exports around logical-area modules instead of the original v0.1-only layout.
- Updated the README, examples, and tests to reflect the v0.2.0 surface.

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
