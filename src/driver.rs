use core::ffi::c_char;
use std::fmt;
use std::ptr;

use serde::Deserialize;

use crate::cf::OwnedCFString;
use crate::endpoint::MidiDevice;
use crate::error::{result_from_status, MidiResult};
use crate::ffi;

extern "C" {
    fn cmr_driver_interface_ids_json() -> *mut c_char;
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct DriverInterfaceIds {
    pub driver_type_id: String,
    pub driver_interface_id: String,
    pub driver_interface2_id: String,
    pub driver_interface3_id: String,
    pub uses_serial_property: String,
}

pub fn driver_interface_ids() -> MidiResult<DriverInterfaceIds> {
    unsafe { crate::private::take_json(cmr_driver_interface_ids_json()) }
}

#[must_use]
pub fn driver_io_run_loop_available() -> bool {
    unsafe { !ffi::MIDIGetDriverIORunLoop().is_null() }
}

pub struct DriverOwnedDevice {
    raw: ffi::MIDIDeviceRef,
}

impl fmt::Debug for DriverOwnedDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DriverOwnedDevice")
            .field("raw", &self.raw)
            .finish()
    }
}

impl DriverOwnedDevice {
    pub fn new(name: &str, manufacturer: &str, model: &str) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let manufacturer = OwnedCFString::new(manufacturer)?;
        let model = OwnedCFString::new(model)?;
        let mut raw = 0;
        result_from_status(unsafe {
            ffi::MIDIDeviceCreate(
                ptr::null_mut(),
                name.as_raw(),
                manufacturer.as_raw(),
                model.as_raw(),
                &mut raw,
            )
        })?;
        Ok(Self { raw })
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIDeviceRef {
        self.raw
    }

    #[must_use]
    pub const fn as_midi_device(&self) -> MidiDevice {
        unsafe { MidiDevice::from_raw(self.raw) }
    }
}

impl Drop for DriverOwnedDevice {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIDeviceDispose(self.raw) };
    }
}
