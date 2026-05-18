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
/// Wraps `MIDIClientRef`.
pub struct MidiClient {
    raw: ffi::MIDIClientRef,
    bridged_client: Option<*mut c_void>,
    notification_context: Option<*mut NotificationContext>,
}

impl MidiClient {
    /// Wraps the CoreMIDI new operation for `MidiClient`.
    pub fn new(name: &str) -> MidiResult<Self> {
        unsafe { Self::with_notify(name, None, ptr::null_mut()) }
    }

    /// Wraps the CoreMIDI with notification handler operation for `MidiClient`.
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

    /// Wraps the CoreMIDI restart operation for `MidiClient`.
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

    /// Wraps the CoreMIDI output port operation for `MidiClient`.
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

    /// Wraps the CoreMIDI input port with protocol operation for `MidiClient`.
    pub fn input_port_with_protocol(
        &self,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<MidiInputPort> {
        MidiInputPort::new_with_protocol(self.raw, name, protocol)
    }

    /// Wraps the CoreMIDI virtual source operation for `MidiClient`.
    pub fn virtual_source(&self, name: &str) -> MidiResult<VirtualSource> {
        VirtualSource::new(self.raw, name)
    }

    /// Wraps the CoreMIDI virtual source with protocol operation for `MidiClient`.
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
    /// Returns the wrapped `MIDIClientRef`.
    pub const fn raw(&self) -> ffi::MIDIClientRef {
        self.raw
    }
}

impl Drop for MidiClient {
    fn drop(&mut self) {
        if let Some(client) = self.bridged_client.take() {
            // SAFETY: `client` is an ARC-managed Swift object created in
            // `with_notification_handler`.  Releasing it disposes the
            // underlying `MIDIClientRef` and unregisters the notification
            // block.  The `notification_context` is freed afterwards (below)
            // so any in-flight `MIDIRestart` notification that races this
            // release still finds a live context and does not dereference
            // freed memory.
            //
            // Caveat: CoreMIDI does not guarantee synchronous draining of
            // in-flight blocks on disposal.  A callback queued concurrently
            // on the MIDI server thread (e.g. during `MIDIRestart`) could
            // theoretically call `notification_callback_trampoline` after the
            // context box is freed below.  A serial-queue barrier would be
            // required to eliminate this window entirely; it is not present in
            // this bridge.
            unsafe { private::release_swift_object(client) };
        } else {
            // SAFETY: `self.raw` is a valid `MIDIClientRef` created in
            // `with_notify` and has not been disposed before.
            let _ = unsafe { ffi::MIDIClientDispose(self.raw) };
        }

        if let Some(context) = self.notification_context.take() {
            // SAFETY: `context` was produced by `Box::into_raw` in
            // `with_notification_handler` and is freed exactly once here,
            // after the Swift client (and therefore the notification block)
            // has been released above.
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

unsafe extern "C" fn notification_callback_trampoline(
    user_info: *mut c_void,
    payload_json: *const c_char,
) {
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
