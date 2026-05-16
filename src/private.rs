use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};

use serde::{de::DeserializeOwned, Serialize};

use crate::error::{MidiError, MidiResult};

extern "C" {
    fn cmr_object_release(ptr: *mut c_void);
}

pub(crate) fn to_cstring(value: &str) -> MidiResult<CString> {
    CString::new(value)
        .map_err(|_| MidiError::InvalidArgument("string contains an interior NUL byte".into()))
}

pub(crate) unsafe fn release_swift_object(ptr: *mut c_void) {
    if !ptr.is_null() {
        cmr_object_release(ptr);
    }
}

pub(crate) unsafe fn take_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let text = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    libc::free(ptr.cast());
    text
}

pub(crate) unsafe fn take_optional_c_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(take_c_string(ptr))
    }
}

pub(crate) unsafe fn take_bytes(ptr: *mut u8, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    let bytes = std::slice::from_raw_parts(ptr.cast_const(), len).to_vec();
    libc::free(ptr.cast());
    bytes
}

pub(crate) unsafe fn swift_result(status: i32, error_ptr: *mut c_char) -> MidiResult<()> {
    if status == 0 {
        return Ok(());
    }
    if !error_ptr.is_null() {
        return Err(MidiError::Bridge(take_c_string(error_ptr)));
    }
    if status <= -10_000 {
        return crate::error::result_from_status(status);
    }
    Err(MidiError::Bridge(format!("CoreMIDI bridge returned status {status}")))
}

pub(crate) fn encode_json<T: Serialize>(value: &T) -> MidiResult<CString> {
    let json = serde_json::to_string(value)
        .map_err(|error| MidiError::Serialization(error.to_string()))?;
    to_cstring(&json)
}

pub(crate) unsafe fn take_json<T: DeserializeOwned>(ptr: *mut c_char) -> MidiResult<T> {
    let json = take_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| MidiError::Serialization(error.to_string()))
}

pub(crate) unsafe fn take_optional_json<T: DeserializeOwned>(ptr: *mut c_char) -> MidiResult<Option<T>> {
    take_optional_c_string(ptr).map_or_else(
        || Ok(None),
        |json| {
            serde_json::from_str(&json)
                .map(Some)
                .map_err(|error| MidiError::Serialization(error.to_string()))
        },
    )
}
