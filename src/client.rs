use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use crate::cf::OwnedCFString;
use crate::endpoint::{VirtualDestination, VirtualSource};
use crate::error::{result_from_status, MidiResult};
use crate::ffi;
use crate::notification::Notification;
use crate::packet::MidiProtocol;
use crate::port::{MidiInputPort, MidiOutputPort};
use crate::private;
use crate::property::MidiObject;

extern "C" {
    fn cmr_client_new_with_notifications(
        name: *const c_char,
        callback: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
        user_info: *mut c_void,
        out_client: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_client_raw(client: *mut c_void) -> ffi::MIDIClientRef;
    fn cmr_client_restart(error_out: *mut *mut c_char) -> i32;
}

struct NotificationContext {
    handler: Box<dyn FnMut(Notification) + Send + 'static>,
}

#[derive(Debug)]
pub struct MidiClient {
    raw: ffi::MIDIClientRef,
    bridged_client: Option<*mut c_void>,
    notification_context: Option<*mut NotificationContext>,
}

impl MidiClient {
    pub fn new(name: &str) -> MidiResult<Self> {
        unsafe { Self::with_notify(name, None, ptr::null_mut()) }
    }

    pub fn with_notification_handler(
        name: &str,
        handler: impl FnMut(Notification) + Send + 'static,
    ) -> MidiResult<Self> {
        let name = private::to_cstring(name)?;
        let context = Box::into_raw(Box::new(NotificationContext {
            handler: Box::new(handler),
        }));
        let mut bridged_client = ptr::null_mut();
        let mut error = ptr::null_mut();

        let result = unsafe {
            private::swift_result(
                cmr_client_new_with_notifications(
                    name.as_ptr(),
                    Some(notification_callback_trampoline),
                    context.cast(),
                    &mut bridged_client,
                    &mut error,
                ),
                error,
            )
        };

        match result {
            Ok(()) => Ok(Self {
                raw: unsafe { cmr_client_raw(bridged_client) },
                bridged_client: Some(bridged_client),
                notification_context: Some(context),
            }),
            Err(error) => {
                unsafe {
                    drop(Box::from_raw(context));
                }
                Err(error)
            }
        }
    }

    pub fn restart() -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe { private::swift_result(cmr_client_restart(&mut error), error) }
    }

    /// Create a `MIDIClientRef` with a CoreMIDI notification callback.
    ///
    /// # Safety
    ///
    /// `notify_proc` and `notify_ref_con` must remain valid for the lifetime of
    /// the client.
    pub unsafe fn with_notify(
        name: &str,
        notify_proc: ffi::MIDINotifyProc,
        notify_ref_con: *mut c_void,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(ffi::MIDIClientCreate(
            name.as_raw(),
            notify_proc,
            notify_ref_con,
            &mut raw,
        ))?;
        Ok(Self {
            raw,
            bridged_client: None,
            notification_context: None,
        })
    }

    pub fn output_port(&self, name: &str) -> MidiResult<MidiOutputPort> {
        MidiOutputPort::new(self.raw, name)
    }

    /// Create a legacy CoreMIDI input port using a direct `MIDIReadProc`.
    ///
    /// # Safety
    ///
    /// `read_proc` and `ref_con` must remain valid for the lifetime of the
    /// returned port.
    pub unsafe fn input_port(
        &self,
        name: &str,
        read_proc: ffi::MIDIReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<MidiInputPort> {
        MidiInputPort::new_legacy(self.raw, name, read_proc, ref_con)
    }

    pub fn input_port_with_protocol(
        &self,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<MidiInputPort> {
        MidiInputPort::new_with_protocol(self.raw, name, protocol)
    }

    pub fn virtual_source(&self, name: &str) -> MidiResult<VirtualSource> {
        VirtualSource::new(self.raw, name)
    }

    pub fn virtual_source_with_protocol(
        &self,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<VirtualSource> {
        VirtualSource::new_with_protocol(self.raw, name, protocol)
    }

    /// Create a virtual destination using a direct `MIDIReadProc` callback.
    ///
    /// # Safety
    ///
    /// `read_proc` and `ref_con` must remain valid for the lifetime of the
    /// returned destination.
    pub unsafe fn virtual_destination(
        &self,
        name: &str,
        read_proc: ffi::MIDIReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<VirtualDestination> {
        VirtualDestination::new(self.raw, name, read_proc, ref_con)
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIClientRef {
        self.raw
    }
}

impl Drop for MidiClient {
    fn drop(&mut self) {
        if let Some(client) = self.bridged_client.take() {
            unsafe { private::release_swift_object(client) };
        } else {
            let _ = unsafe { ffi::MIDIClientDispose(self.raw) };
        }

        if let Some(context) = self.notification_context.take() {
            unsafe {
                drop(Box::from_raw(context));
            }
        }
    }
}

impl MidiObject for MidiClient {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

unsafe extern "C" fn notification_callback_trampoline(user_info: *mut c_void, payload_json: *const c_char) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() || payload_json.is_null() {
            return;
        }

        let context = &mut *user_info.cast::<NotificationContext>();
        let payload = std::ffi::CStr::from_ptr(payload_json)
            .to_string_lossy()
            .into_owned();
        if let Ok(notification) = Notification::from_json_str(&payload) {
            (context.handler)(notification);
        }
    }));
}

