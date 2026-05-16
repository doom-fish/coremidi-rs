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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr($repr)]
        $vis enum $name {
            $($variant = $value,)+
        }

        impl $name {
            #[must_use]
            pub const fn as_raw(self) -> $repr {
                self as $repr
            }

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
pub struct CiDeviceManagerConstants {
    pub device_added_notification: String,
    pub device_removed_notification: String,
    pub profile_updated_notification: String,
    pub profile_removed_notification: String,
    pub device_object_key: String,
    pub profile_object_key: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CiProfileInfo {
    pub name: String,
    #[serde(rename = "profileID", alias = "profileId")]
    pub profile_id: Vec<u8>,
    pub profile_type: u8,
    pub group_offset: u8,
    pub first_channel: u8,
    pub enabled_channel_count: u16,
    pub total_channel_count: u16,
    pub is_enabled: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CiDeviceInfo {
    pub device_info: Midi2DeviceInfo,
    pub muid: u32,
    pub supports_protocol_negotiation: bool,
    pub supports_profile_configuration: bool,
    pub supports_property_exchange: bool,
    pub supports_process_inquiry: bool,
    pub max_sysex_size: usize,
    pub max_property_exchange_requests: usize,
    pub device_type: u8,
    pub profiles: Vec<CiProfileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LegacyCiProfileInfo {
    pub name: String,
    #[serde(rename = "profileID", alias = "profileId")]
    pub profile_id: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CiProfileStateInfo {
    pub midi_channel: u8,
    pub enabled_profiles: Vec<LegacyCiProfileInfo>,
    pub disabled_profiles: Vec<LegacyCiProfileInfo>,
}

#[derive(Debug)]
pub struct CiProfileState {
    raw: *mut c_void,
}

impl CiProfileState {
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

    pub fn snapshot(&self) -> MidiResult<CiProfileStateInfo> {
        unsafe { private::take_json(cmr_ci_profile_state_json(self.raw)) }
    }
}

impl Drop for CiProfileState {
    fn drop(&mut self) {
        unsafe { private::release_swift_object(self.raw) };
    }
}

pub fn ci_device_manager_constants() -> MidiResult<CiDeviceManagerConstants> {
    unsafe { private::take_json(cmr_ci_device_manager_constants_json()) }
}

pub fn discovered_ci_devices() -> MidiResult<Vec<CiDeviceInfo>> {
    unsafe { private::take_json(cmr_ci_devices_json()) }
}

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
