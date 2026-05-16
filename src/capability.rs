use core::ffi::c_char;
use std::ptr;

use serde::Deserialize;

use crate::endpoint::Midi2DeviceInfo;
use crate::error::MidiResult;
use crate::private;

extern "C" {
    fn cmr_ci_devices_json() -> *mut c_char;
    fn cmr_legacy_ci_profile_json(
        bytes: *const u8,
        length: usize,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
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

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LegacyCiProfileInfo {
    pub name: String,
    #[serde(rename = "profileID", alias = "profileId")]
    pub profile_id: Vec<u8>,
}

pub fn discovered_ci_devices() -> MidiResult<Vec<CiDeviceInfo>> {
    unsafe { private::take_json(cmr_ci_devices_json()) }
}

pub fn legacy_ci_profile(profile_id_bytes: &[u8], name: Option<&str>) -> MidiResult<LegacyCiProfileInfo> {
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
        return Err(unsafe { crate::error::MidiError::Bridge(private::take_c_string(error)) });
    }
    unsafe { private::take_json(json) }
}
