#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's [CoreMIDI](https://developer.apple.com/documentation/coremidi)
//! framework on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::copy_iterator,
    clippy::doc_markdown,
    clippy::incompatible_msrv,
    clippy::len_without_is_empty,
    clippy::manual_slice_size_calculation,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::ptr_as_ptr,
    clippy::ref_as_ptr,
    clippy::redundant_pub_crate
)]

pub(crate) mod cf;
pub mod client;
pub mod error;
pub mod ffi;
pub mod object;
pub mod packet;

pub use client::{
    MidiClient, MidiInputPort, MidiOutputPort, MidiProtocolReadProc, VirtualDestination,
    VirtualSource,
};
pub use error::{MidiError, MidiResult, MidiStatus};
pub use ffi::{
    MIDIEventList, MIDIEventPacket, MIDINotification, MIDIPacket, MIDIPacketList, MIDIReadProc,
    MIDIUniversalMessage,
};
pub use object::{
    device, device_count, devices, MidiDevice, MidiEndpoint, MidiEntity, MidiObject, MidiProperty,
};
pub use packet::{
    EventListBuffer, EventListRef, MidiEventPacketRef, MidiPacketRef, MidiProtocol,
    PacketListBuffer, PacketListRef,
};

/// Common imports for users of this crate.
pub mod prelude {
    pub use crate::{
        device, device_count, devices, EventListBuffer, EventListRef, MidiClient, MidiDevice,
        MidiEndpoint, MidiEntity, MidiError, MidiEventPacketRef, MidiInputPort, MidiObject,
        MidiOutputPort, MidiPacketRef, MidiProperty, MidiProtocol, MidiProtocolReadProc,
        MidiResult, MidiStatus, PacketListBuffer, PacketListRef, VirtualDestination, VirtualSource,
    };
}
