use core::ffi::{c_char, c_void};
use core::marker::PhantomData;
use core::ptr;
use std::fmt;

use serde::Deserialize;

use crate::cf::OwnedCFString;
use crate::error::{result_from_status, MidiResult};
use crate::ffi;
use crate::packet::{EventListBuffer, MidiProtocol, PacketListBuffer};
use crate::private;
use crate::property::MidiObject;

extern "C" {
    fn cmr_ump_endpoint_manager_constants_json() -> *mut c_char;
    fn cmr_ump_endpoint_manager_endpoints_json() -> *mut c_char;
    fn cmr_ump_device_info_new(
        manufacturer1: u8,
        manufacturer2: u8,
        manufacturer3: u8,
        family: u16,
        model_number: u16,
        revision1: u8,
        revision2: u8,
        revision3: u8,
        revision4: u8,
        out_info: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_device_info_json(info: *mut c_void) -> *mut c_char;
    fn cmr_ump_mutable_function_block_new(
        name: *const c_char,
        direction: i32,
        first_group: u8,
        total_groups_spanned: u8,
        max_sysex8_streams: u8,
        midi1_info: i32,
        ui_hint: i32,
        is_enabled: bool,
        out_block: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_function_block_json(block: *mut c_void) -> *mut c_char;
    fn cmr_ump_function_block_set_enabled(
        block: *mut c_void,
        is_enabled: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_function_block_set_name(
        block: *mut c_void,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_function_block_reconfigure(
        block: *mut c_void,
        first_group: u8,
        direction: i32,
        midi1_info: i32,
        ui_hint: i32,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_mutable_endpoint_new(
        name: *const c_char,
        device_info: *mut c_void,
        product_instance_id: *const c_char,
        midi_protocol: i32,
        callback: Option<MidiEventListReceiveProc>,
        user_info: *mut c_void,
        out_endpoint: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_endpoint_json(endpoint: *mut c_void) -> *mut c_char;
    fn cmr_ump_mutable_endpoint_set_name(
        endpoint: *mut c_void,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_mutable_endpoint_register_function_blocks(
        endpoint: *mut c_void,
        function_blocks: *const *mut c_void,
        count: usize,
        mark_as_static: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ump_mutable_endpoint_set_enabled(
        endpoint: *mut c_void,
        is_enabled: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
}

/// Callback signature matching the corresponding CoreMIDI receive proc.
pub type MidiEventListReceiveProc =
    unsafe extern "C" fn(*mut c_void, *const ffi::MIDIEventList, *mut c_void);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIDeviceRef`.
pub struct MidiDevice {
    raw: ffi::MIDIDeviceRef,
}

impl MidiDevice {
    #[must_use]
    /// Wraps an existing `MIDIDeviceRef`.
    pub const unsafe fn from_raw(raw: ffi::MIDIDeviceRef) -> Self {
        Self { raw }
    }

    #[must_use]
    /// Returns the wrapped `MIDIDeviceRef`.
    pub const fn raw(self) -> ffi::MIDIDeviceRef {
        self.raw
    }

    #[must_use]
    /// Wraps `MIDIDeviceGetNumberOfEntities`.
    pub fn entity_count(self) -> usize {
        unsafe { ffi::MIDIDeviceGetNumberOfEntities(self.raw) }
    }

    #[must_use]
    /// Wraps `MIDIDeviceGetEntity`.
    pub fn entity(self, index: usize) -> Option<MidiEntity> {
        let raw = unsafe { ffi::MIDIDeviceGetEntity(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEntity::from_raw(raw) })
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI entities operation for `MidiDevice`.
    pub fn entities(self) -> MidiEntityIter {
        MidiEntityIter {
            device: self,
            index: 0,
            count: self.entity_count(),
        }
    }
}

impl MidiObject for MidiDevice {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIEntityRef`.
pub struct MidiEntity {
    raw: ffi::MIDIEntityRef,
}

impl MidiEntity {
    #[must_use]
    /// Wraps an existing `MIDIEntityRef`.
    pub const unsafe fn from_raw(raw: ffi::MIDIEntityRef) -> Self {
        Self { raw }
    }

    #[must_use]
    /// Returns the wrapped `MIDIEntityRef`.
    pub const fn raw(self) -> ffi::MIDIEntityRef {
        self.raw
    }

    #[must_use]
    /// Wraps `MIDIEntityGetNumberOfSources`.
    pub fn source_count(self) -> usize {
        unsafe { ffi::MIDIEntityGetNumberOfSources(self.raw) }
    }

    #[must_use]
    /// Wraps `MIDIEntityGetSource`.
    pub fn source(self, index: usize) -> Option<MidiEndpoint> {
        let raw = unsafe { ffi::MIDIEntityGetSource(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEndpoint::from_raw(raw) })
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI sources operation for `MidiEntity`.
    pub fn sources(self) -> MidiEndpointIter {
        MidiEndpointIter {
            kind: EndpointIterKind::EntitySources(self),
            index: 0,
            count: self.source_count(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps `MIDIEntityGetNumberOfDestinations`.
    pub fn destination_count(self) -> usize {
        unsafe { ffi::MIDIEntityGetNumberOfDestinations(self.raw) }
    }

    #[must_use]
    /// Wraps `MIDIEntityGetDestination`.
    pub fn destination(self, index: usize) -> Option<MidiEndpoint> {
        let raw = unsafe { ffi::MIDIEntityGetDestination(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEndpoint::from_raw(raw) })
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI destinations operation for `MidiEntity`.
    pub fn destinations(self) -> MidiEndpointIter {
        MidiEndpointIter {
            kind: EndpointIterKind::EntityDestinations(self),
            index: 0,
            count: self.destination_count(),
            _marker: PhantomData,
        }
    }

    /// Wraps `MIDIEntityGetDevice`.
    pub fn device(self) -> MidiResult<MidiDevice> {
        let mut raw = 0;
        result_from_status(unsafe { ffi::MIDIEntityGetDevice(self.raw, &mut raw) })?;
        Ok(unsafe { MidiDevice::from_raw(raw) })
    }
}

impl MidiObject for MidiEntity {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIEndpointRef`.
pub struct MidiEndpoint {
    raw: ffi::MIDIEndpointRef,
}

impl MidiEndpoint {
    #[must_use]
    /// Wraps an existing `MIDIEndpointRef`.
    pub const unsafe fn from_raw(raw: ffi::MIDIEndpointRef) -> Self {
        Self { raw }
    }

    #[must_use]
    /// Returns the wrapped `MIDIEndpointRef`.
    pub const fn raw(self) -> ffi::MIDIEndpointRef {
        self.raw
    }

    /// Wraps `MIDIEndpointGetEntity`.
    pub fn entity(self) -> MidiResult<Option<MidiEntity>> {
        let mut raw = 0;
        result_from_status(unsafe { ffi::MIDIEndpointGetEntity(self.raw, &mut raw) })?;
        Ok((raw != 0).then(|| unsafe { MidiEntity::from_raw(raw) }))
    }
}

impl MidiObject for MidiEndpoint {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[must_use]
/// Wraps `MIDIGetNumberOfDevices`.
pub fn device_count() -> usize {
    unsafe { ffi::MIDIGetNumberOfDevices() }
}

#[must_use]
/// Wraps `MIDIGetDevice`.
pub fn device(index: usize) -> Option<MidiDevice> {
    let raw = unsafe { ffi::MIDIGetDevice(index) };
    if raw == 0 {
        None
    } else {
        Some(unsafe { MidiDevice::from_raw(raw) })
    }
}

#[must_use]
/// Wraps the CoreMIDI devices operation for `MidiEndpoint`.
pub fn devices() -> MidiDeviceIter {
    MidiDeviceIter {
        index: 0,
        count: device_count(),
        _marker: PhantomData,
    }
}

#[must_use]
/// Wraps `MIDIGetNumberOfSources`.
pub fn source_count() -> usize {
    unsafe { ffi::MIDIGetNumberOfSources() }
}

#[must_use]
/// Wraps `MIDIGetSource`.
pub fn source(index: usize) -> Option<MidiEndpoint> {
    let raw = unsafe { ffi::MIDIGetSource(index) };
    if raw == 0 {
        None
    } else {
        Some(unsafe { MidiEndpoint::from_raw(raw) })
    }
}

#[must_use]
/// Wraps the CoreMIDI sources operation for `MidiEndpoint`.
pub fn sources() -> MidiEndpointIter {
    MidiEndpointIter {
        kind: EndpointIterKind::SystemSources,
        index: 0,
        count: source_count(),
        _marker: PhantomData,
    }
}

#[must_use]
/// Wraps `MIDIGetNumberOfDestinations`.
pub fn destination_count() -> usize {
    unsafe { ffi::MIDIGetNumberOfDestinations() }
}

#[must_use]
/// Wraps `MIDIGetDestination`.
pub fn destination(index: usize) -> Option<MidiEndpoint> {
    let raw = unsafe { ffi::MIDIGetDestination(index) };
    if raw == 0 {
        None
    } else {
        Some(unsafe { MidiEndpoint::from_raw(raw) })
    }
}

#[must_use]
/// Wraps the CoreMIDI destinations operation for `MidiEndpoint`.
pub fn destinations() -> MidiEndpointIter {
    MidiEndpointIter {
        kind: EndpointIterKind::SystemDestinations,
        index: 0,
        count: destination_count(),
        _marker: PhantomData,
    }
}

#[must_use]
/// Wraps `MIDIGetNumberOfExternalDevices`.
pub fn external_device_count() -> usize {
    unsafe { ffi::MIDIGetNumberOfExternalDevices() }
}

#[must_use]
/// Wraps `MIDIGetExternalDevice`.
pub fn external_device(index: usize) -> Option<MidiDevice> {
    let raw = unsafe { ffi::MIDIGetExternalDevice(index) };
    if raw == 0 {
        None
    } else {
        Some(unsafe { MidiDevice::from_raw(raw) })
    }
}

#[must_use]
/// Wraps the CoreMIDI external devices operation for `MidiEndpoint`.
pub fn external_devices() -> MidiDeviceIter {
    MidiDeviceIter {
        index: 0,
        count: external_device_count(),
        _marker: PhantomData,
    }
}

#[derive(Debug, Clone)]
/// Iterates CoreMIDI MIDI device values.
pub struct MidiDeviceIter {
    index: usize,
    count: usize,
    _marker: PhantomData<()>,
}

impl Iterator for MidiDeviceIter {
    type Item = MidiDevice;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let current = device(self.index).or_else(|| external_device(self.index));
        self.index += 1;
        current
    }
}

#[derive(Debug, Clone)]
/// Iterates CoreMIDI MIDI entity values.
pub struct MidiEntityIter {
    device: MidiDevice,
    index: usize,
    count: usize,
}

impl Iterator for MidiEntityIter {
    type Item = MidiEntity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let current = self.device.entity(self.index);
        self.index += 1;
        current
    }
}

#[derive(Debug)]
enum EndpointIterKind {
    SystemSources,
    SystemDestinations,
    EntitySources(MidiEntity),
    EntityDestinations(MidiEntity),
}

/// Iterates CoreMIDI MIDI endpoint values.
pub struct MidiEndpointIter {
    kind: EndpointIterKind,
    index: usize,
    count: usize,
    _marker: PhantomData<()>,
}

impl fmt::Debug for MidiEndpointIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MidiEndpointIter")
            .field("kind", &self.kind)
            .field("index", &self.index)
            .field("count", &self.count)
            .finish()
    }
}

impl Iterator for MidiEndpointIter {
    type Item = MidiEndpoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        let current = match self.kind {
            EndpointIterKind::SystemSources => source(self.index),
            EndpointIterKind::SystemDestinations => destination(self.index),
            EndpointIterKind::EntitySources(entity) => entity.source(self.index),
            EndpointIterKind::EntityDestinations(entity) => entity.destination(self.index),
        };
        self.index += 1;
        current
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI virtual source payload.
pub struct VirtualSource {
    raw: ffi::MIDIEndpointRef,
}

impl VirtualSource {
    pub(crate) fn new(client: ffi::MIDIClientRef, name: &str) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe { ffi::MIDISourceCreate(client, name.as_raw(), &mut raw) })?;
        Ok(Self { raw })
    }

    pub(crate) fn new_with_protocol(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe {
            ffi::MIDISourceCreateWithProtocol(client, name.as_raw(), protocol.as_raw(), &mut raw)
        })?;
        Ok(Self { raw })
    }

    /// Wraps `MIDIReceived`.
    pub fn received(&self, packets: &PacketListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDIReceived(self.raw, packets.as_ptr()) })
    }

    /// Wraps `MIDIReceivedEventList`.
    pub fn received_event_list(&self, events: &EventListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDIReceivedEventList(self.raw, events.as_ptr()) })
    }

    #[must_use]
    /// Wraps the CoreMIDI endpoint operation for `VirtualSource`.
    pub const fn endpoint(&self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(self.raw) }
    }

    #[must_use]
    /// Returns the wrapped `MIDIEndpointRef`.
    pub const fn raw(&self) -> ffi::MIDIEndpointRef {
        self.raw
    }
}

impl Drop for VirtualSource {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIEndpointDispose(self.raw) };
    }
}

impl MidiObject for VirtualSource {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
/// Wraps `MIDIEndpointRef`.
pub struct VirtualDestination {
    raw: ffi::MIDIEndpointRef,
}

impl VirtualDestination {
    pub(crate) unsafe fn new(
        client: ffi::MIDIClientRef,
        name: &str,
        read_proc: ffi::MIDIReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(ffi::MIDIDestinationCreate(
            client,
            name.as_raw(),
            read_proc,
            ref_con,
            &mut raw,
        ))?;
        Ok(Self { raw })
    }

    #[must_use]
    /// Wraps the CoreMIDI endpoint operation for `VirtualDestination`.
    pub const fn endpoint(&self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(self.raw) }
    }

    #[must_use]
    /// Returns the wrapped `MIDIEndpointRef`.
    pub const fn raw(&self) -> ffi::MIDIEndpointRef {
        self.raw
    }
}

impl Drop for VirtualDestination {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIEndpointDispose(self.raw) };
    }
}

impl MidiObject for VirtualDestination {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Wraps `MIDIEndpointRef`.
pub struct Midi2DeviceInfo {
    #[serde(rename = "manufacturerID", alias = "manufacturerId")]
    /// Mirrors the CoreMIDI manufacturer ID field.
    pub manufacturer_id: [u8; 3],
    /// Mirrors the CoreMIDI family field.
    pub family: u16,
    /// Mirrors the CoreMIDI model number field.
    pub model_number: u16,
    /// Mirrors the CoreMIDI revision level field.
    pub revision_level: [u8; 4],
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI UMP function block info payload.
pub struct UmpFunctionBlockInfo {
    /// Mirrors the CoreMIDI name field.
    pub name: String,
    #[serde(rename = "functionBlockID", alias = "functionBlockId")]
    /// Mirrors the CoreMIDI function block ID field.
    pub function_block_id: u8,
    /// Mirrors the CoreMIDI direction field.
    pub direction: i32,
    /// Mirrors the CoreMIDI first group field.
    pub first_group: u8,
    /// Mirrors the CoreMIDI total groups spanned field.
    pub total_groups_spanned: u8,
    #[serde(rename = "maxSysEx8Streams", alias = "maxSysex8Streams")]
    /// Mirrors the CoreMIDI max sysex8 streams field.
    pub max_sysex8_streams: u8,
    /// Mirrors the CoreMIDI midi1 info field.
    pub midi1_info: i32,
    /// Mirrors the CoreMIDI ui hint field.
    pub ui_hint: i32,
    /// Mirrors the CoreMIDI is enabled field.
    pub is_enabled: bool,
    #[serde(rename = "ciDeviceMUID", alias = "ciDeviceMuid")]
    /// Mirrors the CoreMIDI CI device muid field.
    pub ci_device_muid: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI UMP endpoint info payload.
pub struct UmpEndpointInfo {
    /// Mirrors the CoreMIDI name field.
    pub name: String,
    /// Mirrors the CoreMIDI MIDI protocol field.
    pub midi_protocol: i32,
    #[serde(rename = "supportedMIDIProtocols", alias = "supportedMidiProtocols")]
    /// Mirrors the CoreMIDI supported MIDI protocols field.
    pub supported_midi_protocols: u8,
    /// Mirrors the CoreMIDI MIDI destination field.
    pub midi_destination: u32,
    /// Mirrors the CoreMIDI MIDI source field.
    pub midi_source: u32,
    /// Mirrors the CoreMIDI device info field.
    pub device_info: Midi2DeviceInfo,
    #[serde(rename = "productInstanceID", alias = "productInstanceId")]
    /// Mirrors the CoreMIDI product instance ID field.
    pub product_instance_id: String,
    /// Mirrors the CoreMIDI has static function blocks field.
    pub has_static_function_blocks: bool,
    #[serde(
        rename = "hasJRTSReceiveCapability",
        alias = "hasJrtsReceiveCapability"
    )]
    /// Mirrors the CoreMIDI has jrts receive capability field.
    pub has_jrts_receive_capability: bool,
    #[serde(
        rename = "hasJRTSTransmitCapability",
        alias = "hasJrtsTransmitCapability"
    )]
    /// Mirrors the CoreMIDI has jrts transmit capability field.
    pub has_jrts_transmit_capability: bool,
    /// Mirrors the CoreMIDI endpoint type field.
    pub endpoint_type: u8,
    /// Mirrors the CoreMIDI function blocks field.
    pub function_blocks: Vec<UmpFunctionBlockInfo>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Mirrors the CoreMIDI UMP endpoint manager constants payload.
pub struct UmpEndpointManagerConstants {
    /// Mirrors the CoreMIDI endpoint added notification field.
    pub endpoint_added_notification: String,
    /// Mirrors the CoreMIDI endpoint removed notification field.
    pub endpoint_removed_notification: String,
    /// Mirrors the CoreMIDI endpoint updated notification field.
    pub endpoint_updated_notification: String,
    /// Mirrors the CoreMIDI function block updated notification field.
    pub function_block_updated_notification: String,
    /// Mirrors the CoreMIDI endpoint object key field.
    pub endpoint_object_key: String,
    /// Mirrors the CoreMIDI function block object key field.
    pub function_block_object_key: String,
}

/// Mirrors the CoreMIDI UMP endpoint manager payload.
pub struct UmpEndpointManager;

impl UmpEndpointManager {
    /// Wraps the CoreMIDI constants operation for `UmpEndpointManager`.
    pub fn constants() -> MidiResult<UmpEndpointManagerConstants> {
        unsafe { private::take_json(cmr_ump_endpoint_manager_constants_json()) }
    }

    /// Wraps the CoreMIDI endpoints operation for `UmpEndpointManager`.
    pub fn endpoints() -> MidiResult<Vec<UmpEndpointInfo>> {
        unsafe { private::take_json(cmr_ump_endpoint_manager_endpoints_json()) }
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI MIDI 2 device info handle payload.
pub struct Midi2DeviceInfoHandle {
    raw: *mut c_void,
}

impl Midi2DeviceInfoHandle {
    /// Wraps the CoreMIDI new operation for `Midi2DeviceInfoHandle`.
    pub fn new(
        manufacturer_id: [u8; 3],
        family: u16,
        model_number: u16,
        revision_level: [u8; 4],
    ) -> MidiResult<Self> {
        let mut raw = ptr::null_mut();
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_device_info_new(
                    manufacturer_id[0],
                    manufacturer_id[1],
                    manufacturer_id[2],
                    family,
                    model_number,
                    revision_level[0],
                    revision_level[1],
                    revision_level[2],
                    revision_level[3],
                    &mut raw,
                    &mut error,
                ),
                error,
            )?;
        }
        Ok(Self { raw })
    }

    /// Wraps taking a CoreMIDI snapshot for `Midi2DeviceInfoHandle`.
    pub fn snapshot(&self) -> MidiResult<Midi2DeviceInfo> {
        unsafe { private::take_json(cmr_ump_device_info_json(self.raw)) }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}

impl Drop for Midi2DeviceInfoHandle {
    fn drop(&mut self) {
        unsafe { private::release_swift_object(self.raw) };
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI mutable UMP function block payload.
pub struct MutableUmpFunctionBlock {
    raw: *mut c_void,
}

impl MutableUmpFunctionBlock {
    #[allow(clippy::too_many_arguments)]
    /// Wraps the CoreMIDI new operation for `MutableUmpFunctionBlock`.
    pub fn new(
        name: &str,
        direction: i32,
        first_group: u8,
        total_groups_spanned: u8,
        max_sysex8_streams: u8,
        midi1_info: i32,
        ui_hint: i32,
        is_enabled: bool,
    ) -> MidiResult<Self> {
        let name = private::to_cstring(name)?;
        let mut raw = ptr::null_mut();
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_mutable_function_block_new(
                    name.as_ptr(),
                    direction,
                    first_group,
                    total_groups_spanned,
                    max_sysex8_streams,
                    midi1_info,
                    ui_hint,
                    is_enabled,
                    &mut raw,
                    &mut error,
                ),
                error,
            )?;
        }
        Ok(Self { raw })
    }

    /// Wraps taking a CoreMIDI snapshot for `MutableUmpFunctionBlock`.
    pub fn snapshot(&self) -> MidiResult<UmpFunctionBlockInfo> {
        unsafe { private::take_json(cmr_ump_function_block_json(self.raw)) }
    }

    /// Wraps the CoreMIDI set enabled operation for `MutableUmpFunctionBlock`.
    pub fn set_enabled(&self, is_enabled: bool) -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_function_block_set_enabled(self.raw, is_enabled, &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI set name operation for `MutableUmpFunctionBlock`.
    pub fn set_name(&self, name: &str) -> MidiResult<()> {
        let name = private::to_cstring(name)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_function_block_set_name(self.raw, name.as_ptr(), &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI reconfigure operation for `MutableUmpFunctionBlock`.
    pub fn reconfigure(
        &self,
        first_group: u8,
        direction: i32,
        midi1_info: i32,
        ui_hint: i32,
    ) -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_function_block_reconfigure(
                    self.raw,
                    first_group,
                    direction,
                    midi1_info,
                    ui_hint,
                    &mut error,
                ),
                error,
            )
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}

impl Drop for MutableUmpFunctionBlock {
    fn drop(&mut self) {
        unsafe { private::release_swift_object(self.raw) };
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI mutable UMP endpoint payload.
pub struct MutableUmpEndpoint {
    raw: *mut c_void,
}

impl MutableUmpEndpoint {
    /// Wraps the CoreMIDI new operation for `MutableUmpEndpoint`.
    pub unsafe fn new(
        name: &str,
        device_info: &Midi2DeviceInfoHandle,
        product_instance_id: &str,
        protocol: MidiProtocol,
        callback: MidiEventListReceiveProc,
        user_info: *mut c_void,
    ) -> MidiResult<Self> {
        let name = private::to_cstring(name)?;
        let product_instance_id = private::to_cstring(product_instance_id)?;
        let mut raw = ptr::null_mut();
        let mut error = ptr::null_mut();
        private::swift_result(
            cmr_ump_mutable_endpoint_new(
                name.as_ptr(),
                device_info.as_ptr(),
                product_instance_id.as_ptr(),
                protocol.as_raw(),
                Some(callback),
                user_info,
                &mut raw,
                &mut error,
            ),
            error,
        )?;
        Ok(Self { raw })
    }

    /// Wraps taking a CoreMIDI snapshot for `MutableUmpEndpoint`.
    pub fn snapshot(&self) -> MidiResult<UmpEndpointInfo> {
        unsafe { private::take_json(cmr_ump_endpoint_json(self.raw)) }
    }

    /// Wraps the CoreMIDI set name operation for `MutableUmpEndpoint`.
    pub fn set_name(&self, name: &str) -> MidiResult<()> {
        let name = private::to_cstring(name)?;
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_mutable_endpoint_set_name(self.raw, name.as_ptr(), &mut error),
                error,
            )
        }
    }

    /// Wraps the CoreMIDI register function blocks operation for `MutableUmpEndpoint`.
    pub fn register_function_blocks(
        &self,
        function_blocks: &[MutableUmpFunctionBlock],
        mark_as_static: bool,
    ) -> MidiResult<()> {
        let raw_blocks: Vec<*mut c_void> = function_blocks.iter().map(Self::raw_block).collect();
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_mutable_endpoint_register_function_blocks(
                    self.raw,
                    raw_blocks.as_ptr(),
                    raw_blocks.len(),
                    mark_as_static,
                    &mut error,
                ),
                error,
            )
        }
    }

    fn raw_block(block: &MutableUmpFunctionBlock) -> *mut c_void {
        block.as_ptr()
    }

    /// Wraps the CoreMIDI set enabled operation for `MutableUmpEndpoint`.
    pub fn set_enabled(&self, is_enabled: bool) -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ump_mutable_endpoint_set_enabled(self.raw, is_enabled, &mut error),
                error,
            )
        }
    }
}

impl Drop for MutableUmpEndpoint {
    fn drop(&mut self) {
        unsafe { private::release_swift_object(self.raw) };
    }
}
