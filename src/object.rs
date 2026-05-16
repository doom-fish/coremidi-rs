use core::fmt;
use core::marker::PhantomData;

use crate::cf::{string_from_cfstring, OwnedCFString};
use crate::error::{result_from_status, MidiResult};
use crate::ffi;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiProperty {
    raw: ffi::CFStringRef,
}

impl fmt::Debug for MidiProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MidiProperty").field(&self.raw).finish()
    }
}

impl MidiProperty {
    #[must_use]
    pub const unsafe fn from_raw(raw: ffi::CFStringRef) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn as_raw(self) -> ffi::CFStringRef {
        self.raw
    }

    #[must_use]
    pub fn name() -> Self {
        unsafe { Self::from_raw(ffi::kMIDIPropertyName) }
    }

    #[must_use]
    pub fn manufacturer() -> Self {
        unsafe { Self::from_raw(ffi::kMIDIPropertyManufacturer) }
    }

    #[must_use]
    pub fn model() -> Self {
        unsafe { Self::from_raw(ffi::kMIDIPropertyModel) }
    }

    #[must_use]
    pub fn unique_id() -> Self {
        unsafe { Self::from_raw(ffi::kMIDIPropertyUniqueID) }
    }
}

pub trait MidiObject {
    fn raw_object(&self) -> ffi::MIDIObjectRef;

    fn integer_property(&self, property: MidiProperty) -> MidiResult<i32> {
        object_integer_property(self.raw_object(), property)
    }

    fn string_property(&self, property: MidiProperty) -> MidiResult<String> {
        object_string_property(self.raw_object(), property)
    }

    fn name(&self) -> MidiResult<String> {
        self.string_property(MidiProperty::name())
    }

    fn manufacturer(&self) -> MidiResult<String> {
        self.string_property(MidiProperty::manufacturer())
    }

    fn model(&self) -> MidiResult<String> {
        self.string_property(MidiProperty::model())
    }

    fn unique_id(&self) -> MidiResult<i32> {
        self.integer_property(MidiProperty::unique_id())
    }
}

pub(crate) fn object_integer_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<i32> {
    let mut value = 0_i32;
    result_from_status(unsafe {
        ffi::MIDIObjectGetIntegerProperty(object, property.as_raw(), &mut value)
    })?;
    Ok(value)
}

pub(crate) fn object_string_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<String> {
    let mut value = core::ptr::null();
    result_from_status(unsafe {
        ffi::MIDIObjectGetStringProperty(object, property.as_raw(), &mut value)
    })?;
    let value = unsafe { OwnedCFString::from_owned_raw(value) };
    string_from_cfstring(value.as_raw())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiDevice {
    raw: ffi::MIDIDeviceRef,
}

impl MidiDevice {
    #[must_use]
    pub const unsafe fn from_raw(raw: ffi::MIDIDeviceRef) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(self) -> ffi::MIDIDeviceRef {
        self.raw
    }

    #[must_use]
    pub fn entity_count(self) -> usize {
        unsafe { ffi::MIDIDeviceGetNumberOfEntities(self.raw) }
    }

    #[must_use]
    pub fn entity(self, index: usize) -> Option<MidiEntity> {
        let raw = unsafe { ffi::MIDIDeviceGetEntity(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEntity::from_raw(raw) })
        }
    }

    #[must_use]
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
pub struct MidiEntity {
    raw: ffi::MIDIEntityRef,
}

impl MidiEntity {
    #[must_use]
    pub const unsafe fn from_raw(raw: ffi::MIDIEntityRef) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(self) -> ffi::MIDIEntityRef {
        self.raw
    }

    #[must_use]
    pub fn source_count(self) -> usize {
        unsafe { ffi::MIDIEntityGetNumberOfSources(self.raw) }
    }

    #[must_use]
    pub fn source(self, index: usize) -> Option<MidiEndpoint> {
        let raw = unsafe { ffi::MIDIEntityGetSource(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEndpoint::from_raw(raw) })
        }
    }

    #[must_use]
    pub fn destination_count(self) -> usize {
        unsafe { ffi::MIDIEntityGetNumberOfDestinations(self.raw) }
    }

    #[must_use]
    pub fn destination(self, index: usize) -> Option<MidiEndpoint> {
        let raw = unsafe { ffi::MIDIEntityGetDestination(self.raw, index) };
        if raw == 0 {
            None
        } else {
            Some(unsafe { MidiEndpoint::from_raw(raw) })
        }
    }
}

impl MidiObject for MidiEntity {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiEndpoint {
    raw: ffi::MIDIEndpointRef,
}

impl MidiEndpoint {
    #[must_use]
    pub const unsafe fn from_raw(raw: ffi::MIDIEndpointRef) -> Self {
        Self { raw }
    }

    #[must_use]
    pub const fn raw(self) -> ffi::MIDIEndpointRef {
        self.raw
    }
}

impl MidiObject for MidiEndpoint {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[must_use]
pub fn device_count() -> usize {
    unsafe { ffi::MIDIGetNumberOfDevices() }
}

#[must_use]
pub fn device(index: usize) -> Option<MidiDevice> {
    let raw = unsafe { ffi::MIDIGetDevice(index) };
    if raw == 0 {
        None
    } else {
        Some(unsafe { MidiDevice::from_raw(raw) })
    }
}

#[must_use]
pub fn devices() -> MidiDeviceIter {
    MidiDeviceIter {
        index: 0,
        count: device_count(),
        _marker: PhantomData,
    }
}

#[derive(Debug, Clone)]
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
        let current = device(self.index);
        self.index += 1;
        current
    }
}

#[derive(Debug, Clone)]
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
