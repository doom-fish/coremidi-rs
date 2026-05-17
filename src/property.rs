use core::ffi::c_char;
use core::fmt;
use core::ptr;

use serde_json::Value;

use crate::cf::{string_from_cfstring, OwnedCFString};
use crate::endpoint::{MidiDevice, MidiEndpoint, MidiEntity};
use crate::error::{result_from_status, MidiError, MidiResult};
use crate::ffi;
use crate::private;

extern "C" {
    fn cmr_midi_object_get_data_property(
        object: ffi::MIDIObjectRef,
        property: *const core::ffi::c_void,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_midi_object_set_data_property(
        object: ffi::MIDIObjectRef,
        property: *const core::ffi::c_void,
        bytes: *const u8,
        len: usize,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_midi_object_get_dictionary_property_json(
        object: ffi::MIDIObjectRef,
        property: *const core::ffi::c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    fn cmr_midi_object_set_dictionary_property_json(
        object: ffi::MIDIObjectRef,
        property: *const core::ffi::c_void,
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_midi_object_get_properties_json(
        object: ffi::MIDIObjectRef,
        deep: bool,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MidiObjectType {
    Other,
    Device,
    Entity,
    Source,
    Destination,
    ExternalDevice,
    ExternalEntity,
    ExternalSource,
    ExternalDestination,
    Unknown(i32),
}

impl MidiObjectType {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            ffi::kMIDIObjectType_Other => Self::Other,
            ffi::kMIDIObjectType_Device => Self::Device,
            ffi::kMIDIObjectType_Entity => Self::Entity,
            ffi::kMIDIObjectType_Source => Self::Source,
            ffi::kMIDIObjectType_Destination => Self::Destination,
            ffi::kMIDIObjectType_ExternalDevice => Self::ExternalDevice,
            ffi::kMIDIObjectType_ExternalEntity => Self::ExternalEntity,
            ffi::kMIDIObjectType_ExternalSource => Self::ExternalSource,
            ffi::kMIDIObjectType_ExternalDestination => Self::ExternalDestination,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn raw(self) -> i32 {
        match self {
            Self::Other => ffi::kMIDIObjectType_Other,
            Self::Device => ffi::kMIDIObjectType_Device,
            Self::Entity => ffi::kMIDIObjectType_Entity,
            Self::Source => ffi::kMIDIObjectType_Source,
            Self::Destination => ffi::kMIDIObjectType_Destination,
            Self::ExternalDevice => ffi::kMIDIObjectType_ExternalDevice,
            Self::ExternalEntity => ffi::kMIDIObjectType_ExternalEntity,
            Self::ExternalSource => ffi::kMIDIObjectType_ExternalSource,
            Self::ExternalDestination => ffi::kMIDIObjectType_ExternalDestination,
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub const fn is_external(self) -> bool {
        matches!(
            self,
            Self::ExternalDevice
                | Self::ExternalEntity
                | Self::ExternalSource
                | Self::ExternalDestination
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiProperty {
    raw: ffi::CFStringRef,
}

impl fmt::Debug for MidiProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MidiProperty").field(&self.raw).finish()
    }
}

macro_rules! property_const {
    ($name:ident, $symbol:ident) => {
        #[must_use]
        pub fn $name() -> Self {
            unsafe { Self::from_raw(ffi::$symbol) }
        }
    };
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

    property_const!(name, kMIDIPropertyName);
    property_const!(manufacturer, kMIDIPropertyManufacturer);
    property_const!(model, kMIDIPropertyModel);
    property_const!(unique_id, kMIDIPropertyUniqueID);
    property_const!(device_id, kMIDIPropertyDeviceID);
    property_const!(receive_channels, kMIDIPropertyReceiveChannels);
    property_const!(transmit_channels, kMIDIPropertyTransmitChannels);
    property_const!(max_sysex_speed, kMIDIPropertyMaxSysExSpeed);
    property_const!(
        advance_schedule_time_usec,
        kMIDIPropertyAdvanceScheduleTimeMuSec
    );
    property_const!(is_embedded_entity, kMIDIPropertyIsEmbeddedEntity);
    property_const!(is_broadcast, kMIDIPropertyIsBroadcast);
    property_const!(single_realtime_entity, kMIDIPropertySingleRealtimeEntity);
    property_const!(connection_unique_id, kMIDIPropertyConnectionUniqueID);
    property_const!(offline, kMIDIPropertyOffline);
    property_const!(private, kMIDIPropertyPrivate);
    property_const!(driver_owner, kMIDIPropertyDriverOwner);
    property_const!(factory_patch_name_file, kMIDIPropertyFactoryPatchNameFile);
    property_const!(user_patch_name_file, kMIDIPropertyUserPatchNameFile);
    property_const!(name_configuration, kMIDIPropertyNameConfiguration);
    property_const!(
        name_configuration_dictionary,
        kMIDIPropertyNameConfigurationDictionary
    );
    property_const!(image, kMIDIPropertyImage);
    property_const!(driver_version, kMIDIPropertyDriverVersion);
    property_const!(supports_general_midi, kMIDIPropertySupportsGeneralMIDI);
    property_const!(supports_mmc, kMIDIPropertySupportsMMC);
    property_const!(can_route, kMIDIPropertyCanRoute);
    property_const!(receives_clock, kMIDIPropertyReceivesClock);
    property_const!(receives_mtc, kMIDIPropertyReceivesMTC);
    property_const!(receives_notes, kMIDIPropertyReceivesNotes);
    property_const!(
        receives_program_changes,
        kMIDIPropertyReceivesProgramChanges
    );
    property_const!(receives_bank_select_msb, kMIDIPropertyReceivesBankSelectMSB);
    property_const!(receives_bank_select_lsb, kMIDIPropertyReceivesBankSelectLSB);
    property_const!(transmits_clock, kMIDIPropertyTransmitsClock);
    property_const!(transmits_mtc, kMIDIPropertyTransmitsMTC);
    property_const!(transmits_notes, kMIDIPropertyTransmitsNotes);
    property_const!(
        transmits_program_changes,
        kMIDIPropertyTransmitsProgramChanges
    );
    property_const!(
        transmits_bank_select_msb,
        kMIDIPropertyTransmitsBankSelectMSB
    );
    property_const!(
        transmits_bank_select_lsb,
        kMIDIPropertyTransmitsBankSelectLSB
    );
    property_const!(pan_disrupts_stereo, kMIDIPropertyPanDisruptsStereo);
    property_const!(is_sampler, kMIDIPropertyIsSampler);
    property_const!(is_drum_machine, kMIDIPropertyIsDrumMachine);
    property_const!(is_mixer, kMIDIPropertyIsMixer);
    property_const!(is_effect_unit, kMIDIPropertyIsEffectUnit);
    property_const!(max_receive_channels, kMIDIPropertyMaxReceiveChannels);
    property_const!(max_transmit_channels, kMIDIPropertyMaxTransmitChannels);
    property_const!(driver_device_editor_app, kMIDIPropertyDriverDeviceEditorApp);
    property_const!(supports_show_control, kMIDIPropertySupportsShowControl);
    property_const!(display_name, kMIDIPropertyDisplayName);
    property_const!(protocol_id, kMIDIPropertyProtocolID);
    property_const!(ump_active_group_bitmap, kMIDIPropertyUMPActiveGroupBitmap);
    property_const!(
        ump_can_transmit_groupless,
        kMIDIPropertyUMPCanTransmitGroupless
    );
    property_const!(associated_endpoint, kMIDIPropertyAssociatedEndpoint);
    property_const!(driver_uses_serial, kMIDIDriverPropertyUsesSerial);
}

pub trait MidiObject {
    fn raw_object(&self) -> ffi::MIDIObjectRef;

    fn integer_property(&self, property: MidiProperty) -> MidiResult<i32> {
        object_integer_property(self.raw_object(), property)
    }

    fn set_integer_property(&self, property: MidiProperty, value: i32) -> MidiResult<()> {
        object_set_integer_property(self.raw_object(), property, value)
    }

    fn string_property(&self, property: MidiProperty) -> MidiResult<String> {
        object_string_property(self.raw_object(), property)
    }

    fn set_string_property(&self, property: MidiProperty, value: &str) -> MidiResult<()> {
        object_set_string_property(self.raw_object(), property, value)
    }

    fn data_property(&self, property: MidiProperty) -> MidiResult<Vec<u8>> {
        object_data_property(self.raw_object(), property)
    }

    fn set_data_property(&self, property: MidiProperty, data: &[u8]) -> MidiResult<()> {
        object_set_data_property(self.raw_object(), property, data)
    }

    fn dictionary_property_json(&self, property: MidiProperty) -> MidiResult<Value> {
        object_dictionary_property_json(self.raw_object(), property)
    }

    fn set_dictionary_property_json(
        &self,
        property: MidiProperty,
        value: &Value,
    ) -> MidiResult<()> {
        object_set_dictionary_property_json(self.raw_object(), property, value)
    }

    fn properties_json(&self, deep: bool) -> MidiResult<Value> {
        object_properties_json(self.raw_object(), deep)
    }

    fn remove_property(&self, property: MidiProperty) -> MidiResult<()> {
        object_remove_property(self.raw_object(), property)
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

pub(crate) fn object_set_integer_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
    value: i32,
) -> MidiResult<()> {
    result_from_status(unsafe {
        ffi::MIDIObjectSetIntegerProperty(object, property.as_raw(), value)
    })
}

pub(crate) fn object_string_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<String> {
    let mut value = ptr::null();
    result_from_status(unsafe {
        ffi::MIDIObjectGetStringProperty(object, property.as_raw(), &mut value)
    })?;
    let value = unsafe { OwnedCFString::from_owned_raw(value) };
    string_from_cfstring(value.as_raw())
}

pub(crate) fn object_set_string_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
    value: &str,
) -> MidiResult<()> {
    let value = OwnedCFString::new(value)?;
    result_from_status(unsafe {
        ffi::MIDIObjectSetStringProperty(object, property.as_raw(), value.as_raw())
    })
}

pub(crate) fn object_data_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<Vec<u8>> {
    let mut out_bytes = ptr::null_mut();
    let mut out_len = 0_usize;
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_midi_object_get_data_property(
                object,
                property.as_raw(),
                &mut out_bytes,
                &mut out_len,
                &mut error,
            ),
            error,
        )?;
        Ok(private::take_bytes(out_bytes, out_len))
    }
}

pub(crate) fn object_set_data_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
    data: &[u8],
) -> MidiResult<()> {
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_midi_object_set_data_property(
                object,
                property.as_raw(),
                data.as_ptr(),
                data.len(),
                &mut error,
            ),
            error,
        )
    }
}

pub(crate) fn object_dictionary_property_json(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<Value> {
    let mut error = ptr::null_mut();
    let json = unsafe {
        cmr_midi_object_get_dictionary_property_json(object, property.as_raw(), &mut error)
    };
    if !error.is_null() {
        return Err(unsafe { MidiError::Bridge(private::take_c_string(error)) });
    }
    unsafe { private::take_optional_json(json) }.map(|value| value.unwrap_or(Value::Null))
}

pub(crate) fn object_set_dictionary_property_json(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
    value: &Value,
) -> MidiResult<()> {
    let json = private::encode_json(value)?;
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_midi_object_set_dictionary_property_json(
                object,
                property.as_raw(),
                json.as_ptr(),
                &mut error,
            ),
            error,
        )
    }
}

pub(crate) fn object_properties_json(object: ffi::MIDIObjectRef, deep: bool) -> MidiResult<Value> {
    let mut error = ptr::null_mut();
    let json = unsafe { cmr_midi_object_get_properties_json(object, deep, &mut error) };
    if !error.is_null() {
        return Err(unsafe { MidiError::Bridge(private::take_c_string(error)) });
    }
    unsafe { private::take_optional_json(json) }.map(|value| value.unwrap_or(Value::Null))
}

pub(crate) fn object_remove_property(
    object: ffi::MIDIObjectRef,
    property: MidiProperty,
) -> MidiResult<()> {
    result_from_status(unsafe { ffi::MIDIObjectRemoveProperty(object, property.as_raw()) })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolvedMidiObject {
    Device(MidiDevice),
    Entity(MidiEntity),
    Endpoint(MidiEndpoint),
    Other {
        raw: ffi::MIDIObjectRef,
        object_type: MidiObjectType,
    },
}

pub fn object_find_by_unique_id(unique_id: i32) -> MidiResult<ResolvedMidiObject> {
    let mut object = 0;
    let mut object_type = ffi::kMIDIObjectType_Other;
    result_from_status(unsafe {
        ffi::MIDIObjectFindByUniqueID(unique_id, &mut object, &mut object_type)
    })?;

    let resolved = match MidiObjectType::from_raw(object_type) {
        MidiObjectType::Device | MidiObjectType::ExternalDevice => {
            ResolvedMidiObject::Device(unsafe { MidiDevice::from_raw(object) })
        }
        MidiObjectType::Entity | MidiObjectType::ExternalEntity => {
            ResolvedMidiObject::Entity(unsafe { MidiEntity::from_raw(object) })
        }
        MidiObjectType::Source
        | MidiObjectType::Destination
        | MidiObjectType::ExternalSource
        | MidiObjectType::ExternalDestination => {
            ResolvedMidiObject::Endpoint(unsafe { MidiEndpoint::from_raw(object) })
        }
        other => ResolvedMidiObject::Other {
            raw: object,
            object_type: other,
        },
    };

    Ok(resolved)
}
