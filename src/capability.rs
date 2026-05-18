use core::ffi::{c_char, c_void};
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::endpoint::Midi2DeviceInfo;
use crate::error::{MidiError, MidiResult};
use crate::private;

extern "C" {
    fn cmr_ci_device_manager_constants_json() -> *mut c_char;
    fn cmr_ci_devices_json() -> *mut c_char;
    fn cmr_legacy_ci_profile_json(
        bytes: *const u8,
        length: usize,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    fn cmr_ci_profile_state_new(
        midi_channel: u8,
        use_midi_channel: bool,
        enabled_json: *const c_char,
        disabled_json: *const c_char,
        out_state: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ci_profile_state_json(state: *mut c_void) -> *mut c_char;
}

macro_rules! midi_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident : $repr:ty { $($variant:ident = $value:expr),+ $(,)? }) => {
        $(#[$meta])*
        /// Wraps matching CoreMIDI enum values.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr($repr)]
        $vis enum $name {
            $(#[doc = "Wraps the matching CoreMIDI case."] $variant = $value,)+
        }

        impl $name {
            /// Returns the raw CoreMIDI value for this enum.
            #[must_use]
            pub const fn as_raw(self) -> $repr {
                self as $repr
            }

            /// Converts a raw CoreMIDI value into this enum when it is known.
            #[must_use]
            pub const fn from_raw(raw: $repr) -> Option<Self> {
                match raw {
                    $($value => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

midi_enum!(
    #[allow(clippy::enum_variant_names)]
    pub enum CiProfileMessageType: u8 {
        ProfileInquiry = 0x20,
        ReplyToProfileInquiry = 0x21,
        SetProfileOn = 0x22,
        SetProfileOff = 0x23,
        ProfileEnabledReport = 0x24,
        ProfileDisabledReport = 0x25,
        ProfileAdded = 0x26,
        ProfileRemoved = 0x27,
        DetailsInquiry = 0x28,
        ReplyToDetailsInquiry = 0x29,
        ProfileSpecificData = 0x2F,
    }
);

midi_enum!(
    #[allow(clippy::enum_variant_names)]
    pub enum CiPropertyExchangeMessageType: u8 {
        InquiryPropertyExchangeCapabilities = 0x30,
        ReplyToPropertyExchangeCapabilities = 0x31,
        InquiryHasPropertyDataReserved = 0x32,
        InquiryReplyToHasPropertyDataReserved = 0x33,
        InquiryGetPropertyData = 0x34,
        ReplyToGetProperty = 0x35,
        InquirySetPropertyData = 0x36,
        ReplyToSetPropertyData = 0x37,
        Subscription = 0x38,
        ReplyToSubscription = 0x39,
        Notify = 0x3F,
    }
);

midi_enum!(
    #[allow(clippy::enum_variant_names)]
    pub enum CiProcessInquiryMessageType: u8 {
        InquiryProcessInquiryCapabilities = 0x40,
        ReplyToProcessInquiryCapabilities = 0x41,
        InquiryMidiMessageReport = 0x42,
        ReplyToMidiMessageReport = 0x43,
        EndOfMidiMessageReport = 0x44,
    }
);

midi_enum!(
    #[allow(clippy::enum_variant_names)]
    pub enum CiManagementMessageType: u8 {
        Discovery = 0x70,
        ReplyToDiscovery = 0x71,
        InquiryEndpointInformation = 0x72,
        ReplyToEndpointInformation = 0x73,
        MidiCiAck = 0x7D,
        InvalidateMuid = 0x7E,
        MidiCiNak = 0x7F,
    }
);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Mirrors the CoreMIDI CI device manager constants payload.
pub struct CiDeviceManagerConstants {
    /// Mirrors the CoreMIDI device added notification field.
    pub device_added_notification: String,
    /// Mirrors the CoreMIDI device removed notification field.
    pub device_removed_notification: String,
    /// Mirrors the CoreMIDI profile updated notification field.
    pub profile_updated_notification: String,
    /// Mirrors the CoreMIDI profile removed notification field.
    pub profile_removed_notification: String,
    /// Mirrors the CoreMIDI device object key field.
    pub device_object_key: String,
    /// Mirrors the CoreMIDI profile object key field.
    pub profile_object_key: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI CI profile info payload.
pub struct CiProfileInfo {
    /// Mirrors the CoreMIDI name field.
    pub name: String,
    #[serde(rename = "profileID", alias = "profileId")]
    /// Mirrors the CoreMIDI profile ID field.
    pub profile_id: Vec<u8>,
    /// Mirrors the CoreMIDI profile type field.
    pub profile_type: u8,
    /// Mirrors the CoreMIDI group offset field.
    pub group_offset: u8,
    /// Mirrors the CoreMIDI first channel field.
    pub first_channel: u8,
    /// Mirrors the CoreMIDI enabled channel count field.
    pub enabled_channel_count: u16,
    /// Mirrors the CoreMIDI total channel count field.
    pub total_channel_count: u16,
    /// Mirrors the CoreMIDI is enabled field.
    pub is_enabled: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI CI device info payload.
pub struct CiDeviceInfo {
    /// Mirrors the CoreMIDI device info field.
    pub device_info: Midi2DeviceInfo,
    /// Mirrors the CoreMIDI muid field.
    pub muid: u32,
    /// Mirrors the CoreMIDI supports protocol negotiation field.
    pub supports_protocol_negotiation: bool,
    /// Mirrors the CoreMIDI supports profile configuration field.
    pub supports_profile_configuration: bool,
    /// Mirrors the CoreMIDI supports property exchange field.
    pub supports_property_exchange: bool,
    /// Mirrors the CoreMIDI supports process inquiry field.
    pub supports_process_inquiry: bool,
    /// Mirrors the CoreMIDI max SysEx size field.
    pub max_sysex_size: usize,
    /// Mirrors the CoreMIDI max property exchange requests field.
    pub max_property_exchange_requests: usize,
    /// Mirrors the CoreMIDI device type field.
    pub device_type: u8,
    /// Mirrors the CoreMIDI profiles field.
    pub profiles: Vec<CiProfileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI legacy CI profile info payload.
pub struct LegacyCiProfileInfo {
    /// Mirrors the CoreMIDI name field.
    pub name: String,
    #[serde(rename = "profileID", alias = "profileId")]
    /// Mirrors the CoreMIDI profile ID field.
    pub profile_id: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Mirrors the CoreMIDI CI profile state info payload.
pub struct CiProfileStateInfo {
    /// Mirrors the CoreMIDI MIDI channel field.
    pub midi_channel: u8,
    /// Mirrors the CoreMIDI enabled profiles field.
    pub enabled_profiles: Vec<LegacyCiProfileInfo>,
    /// Mirrors the CoreMIDI disabled profiles field.
    pub disabled_profiles: Vec<LegacyCiProfileInfo>,
}

#[derive(Debug)]
/// Mirrors the CoreMIDI CI profile state payload.
pub struct CiProfileState {
    raw: *mut c_void,
}

impl CiProfileState {
    /// Wraps the CoreMIDI new operation for `CiProfileState`.
    pub fn new(
        midi_channel: Option<u8>,
        enabled_profiles: &[LegacyCiProfileInfo],
        disabled_profiles: &[LegacyCiProfileInfo],
    ) -> MidiResult<Self> {
        if let Some(channel) = midi_channel {
            if channel > 0x0F {
                return Err(MidiError::InvalidArgument(
                    "MIDI-CI profile-state channel must be in the range 0..=15".into(),
                ));
            }
        }
        validate_profile_ids(enabled_profiles)?;
        validate_profile_ids(disabled_profiles)?;

        let enabled_json = private::encode_json(enabled_profiles)?;
        let disabled_json = private::encode_json(disabled_profiles)?;
        let mut raw = ptr::null_mut();
        let mut error = ptr::null_mut();
        unsafe {
            private::swift_result(
                cmr_ci_profile_state_new(
                    midi_channel.unwrap_or_default(),
                    midi_channel.is_some(),
                    enabled_json.as_ptr(),
                    disabled_json.as_ptr(),
                    &mut raw,
                    &mut error,
                ),
                error,
            )?;
        }
        Ok(Self { raw })
    }

    /// Wraps taking a CoreMIDI snapshot for `CiProfileState`.
    pub fn snapshot(&self) -> MidiResult<CiProfileStateInfo> {
        unsafe { private::take_json(cmr_ci_profile_state_json(self.raw)) }
    }
}

impl Drop for CiProfileState {
    fn drop(&mut self) {
        unsafe { private::release_swift_object(self.raw) };
    }
}

/// Wraps the CoreMIDI CI device manager constants operation for `CiProfileState`.
pub fn ci_device_manager_constants() -> MidiResult<CiDeviceManagerConstants> {
    unsafe { private::take_json(cmr_ci_device_manager_constants_json()) }
}

/// Wraps the CoreMIDI discovered CI devices operation for `CiProfileState`.
pub fn discovered_ci_devices() -> MidiResult<Vec<CiDeviceInfo>> {
    unsafe { private::take_json(cmr_ci_devices_json()) }
}

/// Wraps the CoreMIDI legacy CI profile operation for `CiProfileState`.
pub fn legacy_ci_profile(
    profile_id_bytes: &[u8],
    name: Option<&str>,
) -> MidiResult<LegacyCiProfileInfo> {
    let name = name.map(private::to_cstring).transpose()?;
    let mut error = ptr::null_mut();
    let json = unsafe {
        cmr_legacy_ci_profile_json(
            profile_id_bytes.as_ptr(),
            profile_id_bytes.len(),
            name.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            &mut error,
        )
    };
    if !error.is_null() {
        return Err(unsafe { MidiError::Bridge(private::take_c_string(error)) });
    }
    unsafe { private::take_json(json) }
}

fn validate_profile_ids(profiles: &[LegacyCiProfileInfo]) -> MidiResult<()> {
    for profile in profiles {
        if profile.profile_id.len() != 5 {
            return Err(MidiError::InvalidArgument(format!(
                "MIDI-CI profile '{}' must contain exactly 5 profile-ID bytes",
                profile.name
            )));
        }
    }
    Ok(())
}
