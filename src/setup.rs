use crate::cf::{string_from_cfstring, OwnedCFString};
use crate::driver::DriverOwnedDevice;
use crate::endpoint::{MidiDevice, MidiEntity};
use crate::error::{result_from_status, MidiResult};
use crate::ffi;
use crate::packet::MidiProtocol;

pub fn current_setup_xml() -> MidiResult<Vec<u8>> {
    let mut setup = 0;
    result_from_status(unsafe { ffi::MIDISetupGetCurrent(&mut setup) })?;

    let mut data = core::ptr::null();
    let result = unsafe { ffi::MIDISetupToData(setup, &mut data) };
    let dispose_result = unsafe { ffi::MIDISetupDispose(setup) };
    result_from_status(result)?;
    result_from_status(dispose_result)?;

    if data.is_null() {
        return Ok(Vec::new());
    }

    let bytes = unsafe {
        let len = ffi::CFDataGetLength(data);
        let len = usize::try_from(len).unwrap_or(0);
        let ptr = ffi::CFDataGetBytePtr(data);
        let bytes = std::slice::from_raw_parts(ptr, len).to_vec();
        ffi::CFRelease(data.cast());
        bytes
    };
    Ok(bytes)
}

pub fn serial_port_owner(port_name: &str) -> MidiResult<Option<String>> {
    let port_name = OwnedCFString::new(port_name)?;
    let mut owner = core::ptr::null();
    result_from_status(unsafe { ffi::MIDIGetSerialPortOwner(port_name.as_raw(), &mut owner) })?;
    if owner.is_null() {
        return Ok(None);
    }

    let value = string_from_cfstring(owner)?;
    unsafe { ffi::CFRelease(owner.cast()) };
    Ok(Some(value))
}

pub fn serial_port_drivers() -> MidiResult<Vec<String>> {
    let mut array = core::ptr::null();
    result_from_status(unsafe { ffi::MIDIGetSerialPortDrivers(&mut array) })?;
    if array.is_null() {
        return Ok(Vec::new());
    }

    let mut drivers = Vec::new();
    unsafe {
        let count = ffi::CFArrayGetCount(array);
        for index in 0..count {
            let value = ffi::CFArrayGetValueAtIndex(array, index);
            if !value.is_null() {
                drivers.push(string_from_cfstring(value.cast())?);
            }
        }
        ffi::CFRelease(array.cast());
    }
    Ok(drivers)
}

pub fn add_driver_device(device: DriverOwnedDevice) -> MidiResult<MidiDevice> {
    let raw = device.raw();
    result_from_status(unsafe { ffi::MIDISetupAddDevice(raw) })?;
    std::mem::forget(device);
    Ok(unsafe { MidiDevice::from_raw(raw) })
}

pub fn remove_device(device: MidiDevice) -> MidiResult<()> {
    result_from_status(unsafe { ffi::MIDISetupRemoveDevice(device.raw()) })
}

pub fn add_external_device_named(
    name: &str,
    manufacturer: &str,
    model: &str,
) -> MidiResult<MidiDevice> {
    let name = OwnedCFString::new(name)?;
    let manufacturer = OwnedCFString::new(manufacturer)?;
    let model = OwnedCFString::new(model)?;
    let mut raw = 0;
    result_from_status(unsafe {
        ffi::MIDIExternalDeviceCreate(
            name.as_raw(),
            manufacturer.as_raw(),
            model.as_raw(),
            &mut raw,
        )
    })?;
    result_from_status(unsafe { ffi::MIDISetupAddExternalDevice(raw) })?;
    Ok(unsafe { MidiDevice::from_raw(raw) })
}

pub fn remove_external_device(device: MidiDevice) -> MidiResult<()> {
    result_from_status(unsafe { ffi::MIDISetupRemoveExternalDevice(device.raw()) })
}

pub fn device_new_entity(
    device: MidiDevice,
    name: &str,
    protocol: MidiProtocol,
    embedded: bool,
    num_source_endpoints: usize,
    num_destination_endpoints: usize,
) -> MidiResult<MidiEntity> {
    let name = OwnedCFString::new(name)?;
    let mut raw = 0;
    result_from_status(unsafe {
        ffi::MIDIDeviceNewEntity(
            device.raw(),
            name.as_raw(),
            protocol.as_raw(),
            u8::from(embedded),
            num_source_endpoints,
            num_destination_endpoints,
            &mut raw,
        )
    })?;
    Ok(unsafe { MidiEntity::from_raw(raw) })
}

pub fn device_add_entity_deprecated(
    device: MidiDevice,
    name: &str,
    embedded: bool,
    num_source_endpoints: usize,
    num_destination_endpoints: usize,
) -> MidiResult<MidiEntity> {
    let name = OwnedCFString::new(name)?;
    let mut raw = 0;
    result_from_status(unsafe {
        ffi::MIDIDeviceAddEntity(
            device.raw(),
            name.as_raw(),
            u8::from(embedded),
            num_source_endpoints,
            num_destination_endpoints,
            &mut raw,
        )
    })?;
    Ok(unsafe { MidiEntity::from_raw(raw) })
}

pub fn device_remove_entity(device: MidiDevice, entity: MidiEntity) -> MidiResult<()> {
    result_from_status(unsafe { ffi::MIDIDeviceRemoveEntity(device.raw(), entity.raw()) })
}

pub fn entity_set_endpoint_counts(
    entity: MidiEntity,
    num_source_endpoints: usize,
    num_destination_endpoints: usize,
) -> MidiResult<()> {
    result_from_status(unsafe {
        ffi::MIDIEntityAddOrRemoveEndpoints(
            entity.raw(),
            num_source_endpoints,
            num_destination_endpoints,
        )
    })
}
