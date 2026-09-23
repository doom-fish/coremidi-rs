use core::ffi::{c_char, c_void};
use std::ptr;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;

use crate::cf::OwnedCFString;
use crate::endpoint::{VirtualDestination, VirtualSource};
use crate::error::{result_from_status, MidiResult};
use crate::ffi;
use crate::notification::Notification;
use crate::packet::MidiProtocol;
use crate::port::{MidiInputPort, MidiOutputPort};
use crate::private;
use crate::property::MidiObject;
use crate::receiver::{event_channel, MidiEventReceiver};

extern "C" {
    fn cmr_client_new_with_notifications(
        name: *const c_char,
        callback: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
        user_info: *mut c_void,
        context_retain: Option<unsafe extern "C" fn(*mut c_void)>,
        context_release: Option<unsafe extern "C" fn(*mut c_void)>,
        out_client: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_client_raw(client: *mut c_void) -> ffi::MIDIClientRef;
    fn cmr_client_restart(error_out: *mut *mut c_char) -> i32;
}

type NotificationHandler = Mutex<Box<dyn FnMut(Notification) + Send + 'static>>;

#[derive(Debug)]
/// Wraps `MIDIClientRef`.
pub struct MidiClient {
    raw: ffi::MIDIClientRef,
    bridged_client: Option<*mut c_void>,
    notification_context: Option<CallbackContext<NotificationHandler>>,
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
        let handler: Box<dyn FnMut(Notification) + Send + 'static> = Box::new(handler);
        let context = CallbackContext::new(Mutex::new(handler));
        let mut bridged_client = ptr::null_mut();
        let mut error = ptr::null_mut();

        unsafe {
            private::swift_result(
                cmr_client_new_with_notifications(
                    name.as_ptr(),
                    Some(notification_callback_trampoline),
                    context.as_ptr(),
                    Some(CallbackContext::<NotificationHandler>::RETAIN),
                    Some(CallbackContext::<NotificationHandler>::RELEASE),
                    &raw mut bridged_client,
                    &raw mut error,
                ),
                error,
            )
        }?;

        Ok(Self {
            raw: unsafe { cmr_client_raw(bridged_client) },
            bridged_client: Some(bridged_client),
            notification_context: Some(context),
        })
    }

    /// Wraps the CoreMIDI restart operation for `MidiClient`.
    pub fn restart() -> MidiResult<()> {
        let mut error = ptr::null_mut();
        unsafe { private::swift_result(cmr_client_restart(&raw mut error), error) }
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
            &raw mut raw,
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

    pub fn input_port_with_receiver(
        &self,
        name: &str,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<(MidiInputPort, MidiEventReceiver)> {
        let (sink, receiver) = event_channel(capacity)?;
        let port = MidiInputPort::new_with_receiver(self.raw, name, protocol, sink)?;
        Ok((port, receiver))
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

    pub fn virtual_destination_with_receiver(
        &self,
        name: &str,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<(VirtualDestination, MidiEventReceiver)> {
        let (sink, receiver) = event_channel(capacity)?;
        let destination = VirtualDestination::new_with_receiver(self.raw, name, protocol, sink)?;
        Ok((destination, receiver))
    }

    #[must_use]
    /// Returns the wrapped `MIDIClientRef`.
    pub const fn raw(&self) -> ffi::MIDIClientRef {
        self.raw
    }
}

impl Drop for MidiClient {
    fn drop(&mut self) {
        if let Some(context) = &self.notification_context {
            context.deactivate();
        }
        if let Some(client) = self.bridged_client.take() {
            unsafe { private::release_swift_object(client) };
        } else {
            let _ = unsafe { ffi::MIDIClientDispose(self.raw) };
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
    let _ = unsafe {
        CallbackContext::<NotificationHandler>::with(
            user_info,
            "MidiClient notification handler",
            |handler| {
                if let Some(notification) = Notification::from_bridge_payload(payload_json) {
                    let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
                    handler(notification);
                }
            },
        )
    };
}
