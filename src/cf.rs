use std::ffi::{CStr, CString};

use crate::error::{MidiError, MidiResult};
use crate::ffi;

pub(crate) struct OwnedCFString(ffi::CFStringRef);

impl OwnedCFString {
    pub(crate) fn new(value: &str) -> MidiResult<Self> {
        let c_string = CString::new(value).map_err(|_| {
            MidiError::InvalidArgument("string contains an interior NUL byte".into())
        })?;
        let raw = unsafe {
            ffi::CFStringCreateWithCString(
                ffi::kCFAllocatorDefault,
                c_string.as_ptr(),
                ffi::kCFStringEncodingUTF8,
            )
        };
        if raw.is_null() {
            Err(MidiError::CoreFoundation(
                "CFStringCreateWithCString returned NULL".into(),
            ))
        } else {
            Ok(Self(raw))
        }
    }

    pub(crate) const unsafe fn from_owned_raw(raw: ffi::CFStringRef) -> Self {
        Self(raw)
    }

    pub(crate) const fn as_raw(&self) -> ffi::CFStringRef {
        self.0
    }
}

impl Drop for OwnedCFString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::CFRelease(self.0.cast()) };
        }
    }
}

pub(crate) fn string_from_cfstring(raw: ffi::CFStringRef) -> MidiResult<String> {
    if raw.is_null() {
        return Err(MidiError::CoreFoundation("null CFStringRef".into()));
    }

    let length = unsafe { ffi::CFStringGetLength(raw) };
    let capacity =
        unsafe { ffi::CFStringGetMaximumSizeForEncoding(length, ffi::kCFStringEncodingUTF8) } + 1;
    let capacity = usize::try_from(capacity).unwrap_or(0).max(1);
    let mut buffer = vec![0_i8; capacity];
    let ok = unsafe {
        ffi::CFStringGetCString(
            raw,
            buffer.as_mut_ptr(),
            isize::try_from(buffer.len()).unwrap_or(isize::MAX),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if !ok {
        return Err(MidiError::CoreFoundation(
            "CFStringGetCString returned false".into(),
        ));
    }

    let cstr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    Ok(String::from_utf8_lossy(cstr.to_bytes()).into_owned())
}
