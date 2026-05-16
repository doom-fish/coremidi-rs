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
pub mod capability;
pub mod client;
pub mod driver;
pub mod endpoint;
pub mod error;
pub mod ffi;
pub mod network;
pub mod notification;
pub mod object;
pub mod packet;
pub mod port;
pub mod property;
pub mod setup;
pub mod thru_connection;
pub(crate) mod private;

pub use capability::{
    ci_device_manager_constants, discovered_ci_devices, legacy_ci_profile, CiDeviceInfo,
    CiDeviceManagerConstants, CiManagementMessageType, CiProcessInquiryMessageType,
    CiProfileInfo, CiProfileMessageType, CiProfileState, CiProfileStateInfo,
    CiPropertyExchangeMessageType, LegacyCiProfileInfo,
};
pub use client::MidiClient;
pub use driver::{driver_interface_ids, driver_io_run_loop_available, DriverInterfaceIds, DriverOwnedDevice};
pub use endpoint::{
    destination, destination_count, destinations, device, device_count, devices, external_device,
    external_device_count, external_devices, source, source_count, sources, Midi2DeviceInfo,
    Midi2DeviceInfoHandle, MidiDevice, MidiDeviceIter, MidiEndpoint, MidiEndpointIter, MidiEntity,
    MutableUmpEndpoint, MutableUmpFunctionBlock, UmpEndpointInfo, UmpEndpointManager,
    UmpEndpointManagerConstants, UmpFunctionBlockInfo, VirtualDestination, VirtualSource,
};
pub use error::{MidiError, MidiResult, MidiStatus};
pub use ffi::{
    MIDIEventList, MIDIEventPacket, MIDINotification, MIDIPacket, MIDIPacketList, MIDIReadProc,
    MIDIUniversalMessage,
};
pub use network::{
    activate_bluetooth_connections, disconnect_bluetooth, NetworkConnection,
    NetworkConnectionPolicy, NetworkConstants, NetworkHost, NetworkSession,
};
pub use notification::{Notification, NotificationMessageId};
pub use packet::{
    EventIter, EventListBuffer, EventListRef, MidiCvStatus, MidiEventPacketRef, MidiMessage128,
    MidiMessage64, MidiMessage96, MidiMessageType, MidiNoteAttribute,
    MidiPerNoteManagementOptions, MidiPacketRef, MidiProgramChangeOptions, MidiProtocol,
    MidiSysExStatus, MidiSystemStatus, MidiUtilityStatus, PacketIter, PacketListBuffer,
    PacketListRef, UmpStreamMessageFormat, UmpStreamMessageStatus,
};
pub use port::{flush_output, MidiInputPort, MidiOutputPort, MidiProtocolReadProc};
pub use property::{object_find_by_unique_id, MidiObject, MidiObjectType, MidiProperty, ResolvedMidiObject};
pub use setup::{
    add_driver_device, add_external_device_named, current_setup_xml, device_add_entity_deprecated,
    device_new_entity, device_remove_entity, entity_set_endpoint_counts, remove_device,
    remove_external_device, serial_port_drivers, serial_port_owner,
};
pub use thru_connection::{
    MidiControlTransform, MidiControlType, MidiTransform, MidiTransformKind, ThruConnection,
    ThruConnectionEndpoint, ThruConnectionParams,
};

/// Common imports for users of this crate.
pub mod prelude {
    pub use crate::{
        device, device_count, devices, destination, destination_count, destinations,
        external_device, external_device_count, external_devices, source, source_count, sources,
        EventListBuffer, EventListRef, MidiClient, MidiDevice, MidiEndpoint, MidiEntity,
        MidiError, MidiEventPacketRef, MidiInputPort, MidiObject, MidiOutputPort, MidiPacketRef,
        MidiProperty, MidiProtocol, MidiProtocolReadProc, MidiResult, MidiStatus,
        Notification, NotificationMessageId, PacketListBuffer, PacketListRef, VirtualDestination,
        VirtualSource,
    };
}
