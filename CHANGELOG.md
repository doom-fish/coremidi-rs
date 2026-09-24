# Changelog

All notable changes to `coremidi-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - Unreleased

### Security

- `PacketListBuffer` and `EventListBuffer` derived `Clone` over a raw pointer into their
  storage, so `add_packet` on a clone wrote into the original buffer, or into freed memory once
  the original was dropped. They now keep the write position as an offset into their own storage.
- The async notification and thru-connection streams gave the Swift notification block no
  reference to their context and freed it right after releasing the client; the event,
  virtual-destination and MIDI-CI discovery streams handed CoreMIDI or KVO an unretained sender
  that `Drop` freed straight after unregistering. Dropping a stream while a delivery was running
  was a use-after-free. Each stream context is now a `CallbackContext` that the block or observer
  holds a reference to, and `Drop` deactivates it before unregistering.
- Dropping a `MidiInputPort` freed its protocol connection contexts right after
  `MIDIPortDispose`, while a callback could still be running on the receive thread.

### Fixed

- `NetworkSession::network_port` declared the bridge's return value as `i32` while Swift
  returns an `Int`; the declaration now matches.
- `MidiInputPort::disconnect_source` frees the connection context that
  `connect_source_with_protocol_callback` created (one leaked per connect/disconnect cycle).
  Disconnecting, connecting the same source again (which replaces its callback) and dropping the
  port all wait for a running callback, so the caller's `ref_con` can be freed afterwards.
- `MidiClient::with_notification_handler` runs the handler behind a mutex (notifications on
  different threads aliased the `FnMut`), contains panics, and stops calling it once the client
  is dropped.
- `setup::current_setup_xml` no longer disposes the setup returned by `MIDISetupGetCurrent`,
  which the system owns, and checks `CFDataGetBytePtr` for NULL.
- `ffi::MIDIPacketNext` read the packed `length` field through a pointer that is misaligned on
  x86_64. It and `ffi::MIDIEventPacketNext` now use unaligned reads.
- `COVERAGE_AUDIT.md` listed 15 wrapped symbols, including `MIDIDestinationCreate`, as exempt and
  counted `MIDIEventListForEachEvent` as wrapped. Both audit files now say what their numbers
  measure.

### Changed

- **Breaking:** `MidiInputPort::connect_source` returns `MidiError::Unsupported` unless the port
  was created with `MidiClient::input_port` (`MIDIReadProc`). Protocol ports passed the caller's
  `connRefCon` to a callback that read it as an internal context.
- **Breaking:** The Swift bridge declares macOS 11 as its minimum. The crate already linked the
  macOS 11 MIDI 2.0 functions, so it could not run on macOS 10.15.
- Protocol input ports and destinations with receive blocks are created by the Swift bridge; the
  block holds a reference to the Rust context until CoreMIDI releases it. Connections are keyed
  by source endpoint rather than by a heap pointer passed as `connRefCon`.
- `PacketListBuffer`, `EventListBuffer` and `MidiInputPort` are now `Send` and `Sync`.
- `MidiEventStream` and `MidiVirtualDestinationStream` document that they allocate an
  `OwnedEventList` on the receive thread.
- `doom-fish-utils` (`>=0.4.1, <0.5`) is now a required dependency, and the `async` feature no
  longer enables it. `apple-cf` moves to `>=0.11, <0.12`, and `rust-version` is now 1.82.

### Deprecated

- `setup::current_setup_xml`, `setup::serial_port_owner` and `setup::serial_port_drivers` wrap
  APIs that have been "No longer supported" since macOS 10.6 (they return -4 on current systems).
  `setup::device_add_entity_deprecated` is superseded by `setup::device_new_entity`.

### Added

- `receiver` module with `MidiEventReceiver` and `MidiEventRecord`, created by
  `MidiClient::input_port_with_receiver` and `MidiClient::virtual_destination_with_receiver`;
  `MidiInputPort::connect` connects a source to a receiver port. The receive thread copies each
  event packet into a preallocated ring of fixed-size records without locking or allocating, and
  overwrites the oldest record when the ring is full (`dropped_count()`).

## [0.5.4] - 2026-06-06

- Hardened the client notification FFI (the Swift notification block holds a reference to the
  handler context) and bounded the `MIDIEventPacket` copy in `OwnedEventList::copy_from` to the
  packet's word count.

## [0.5.3] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.5.2] - 2026-05-18

### Changed

- Added one-line rustdoc coverage notes for every public item outside `src/ffi/`, raising `cargo rustdoc --all-features -- -Z unstable-options --show-coverage` from 0.8% to 100.0%.

## [0.5.1] - 2026-05-18

### Changed

- chore: re-export OS primitives (Boolean, OSStatus) from apple-cf

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
