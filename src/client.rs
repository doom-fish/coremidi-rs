use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::atomic::{fence, AtomicUsize, Ordering};

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
        context_retain: Option<unsafe extern "C" fn(*mut c_void)>,
        context_release: Option<unsafe extern "C" fn(*mut c_void)>,
        out_client: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_client_raw(client: *mut c_void) -> ffi::MIDIClientRef;
    fn cmr_client_restart(error_out: *mut *mut c_char) -> i32;
}

struct NotificationContext {
    handler: Box<dyn FnMut(Notification) + Send + 'static>,
    ref_count: AtomicUsize,
}

impl NotificationContext {
    fn new(handler: Box<dyn FnMut(Notification) + Send + 'static>) -> *mut Self {
        Box::into_raw(Box::new(Self {
            handler,
            ref_count: AtomicUsize::new(1),
        }))
    }

    /// Increment the reference count.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a valid, live `NotificationContext`.
    unsafe fn retain(ptr: *mut Self) {
        if ptr.is_null() {
            return;
        }
        unsafe { &(*ptr).ref_count }.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement the reference count, freeing the context when it reaches zero.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a valid, live `NotificationContext`. After this call
    /// the caller must not use `ptr` if the context was freed.
    unsafe fn release(ptr: *mut Self) {
        if ptr.is_null() {
            return;
        }
        let prev = unsafe { &(*ptr).ref_count }.fetch_sub(1, Ordering::Release);
        if prev == 1 {
            // Acquire fence pairs with the Release stores from every other
            // thread's `fetch_sub` so the freeing thread observes all of their
            // prior writes. This is the canonical Arc-style refcount drop and
            // is required for soundness on weakly-ordered architectures.
            fence(Ordering::Acquire);
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

// C trampoline handed to Swift so the notification block can take a +1
// reference on the Rust `NotificationContext` for the duration of its own
// lifetime. This keeps the context alive while any notification can still be
// dispatched on it, even though CoreMIDI does not synchronously drain
// in-flight notification blocks on `MIDIClientDispose`.
unsafe extern "C" fn notification_context_retain(context: *mut c_void) {
    unsafe { NotificationContext::retain(context.cast::<NotificationContext>()) };
}

// C trampoline handed to Swift, invoked from the notification block's owning
// object `deinit` to drop the +1 taken in `notification_context_retain`.
unsafe extern "C" fn notification_context_release(context: *mut c_void) {
    unsafe { NotificationContext::release(context.cast::<NotificationContext>()) };
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
        let context = NotificationContext::new(Box::new(handler));
        let mut bridged_client = ptr::null_mut();
        let mut error = ptr::null_mut();

        let result = unsafe {
            private::swift_result(
                cmr_client_new_with_notifications(
                    name.as_ptr(),
                    Some(notification_callback_trampoline),
                    context.cast(),
                    Some(notification_context_retain),
                    Some(notification_context_release),
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
                    NotificationContext::release(context);
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
            // underlying `MIDIClientRef` and drops CoreMIDI's reference to the
            // notification block, whose owning Swift object holds a +1 on the
            // `NotificationContext` (taken in `init` via
            // `notification_context_retain`, dropped in `deinit` via
            // `notification_context_release`).
            //
            // CoreMIDI does not guarantee synchronous draining of in-flight
            // notification blocks on disposal, but that is safe here: while a
            // block invocation is in flight, ARC keeps the block (and the
            // object it strongly captures) alive for the duration of the call,
            // so that object's +1 on the context is held throughout. The
            // context box is therefore freed only once both this Rust
            // `MidiClient` and the Swift block-owning object have released
            // their references — never while a callback can still observe it.
            unsafe { private::release_swift_object(client) };
        } else {
            // SAFETY: `self.raw` is a valid `MIDIClientRef` created in
            // `with_notify` and has not been disposed before.
            let _ = unsafe { ffi::MIDIClientDispose(self.raw) };
        }

        if let Some(context) = self.notification_context.take() {
            // SAFETY: `context` was produced by `NotificationContext::new` in
            // `with_notification_handler`. This drops the +1 owned by the Rust
            // `MidiClient`; the box is freed only when the Swift block-owning
            // object has also released its reference, so any in-flight
            // notification still finds a live context.
            unsafe {
                NotificationContext::release(context);
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

        let context = user_info.cast::<NotificationContext>();
        let payload = std::ffi::CStr::from_ptr(payload_json)
            .to_string_lossy()
            .into_owned();
        if let Ok(notification) = Notification::from_json_str(&payload) {
            // Borrow only the `handler` field; the `ref_count` atomic may be
            // touched concurrently by retain/release through shared refs.
            ((*context).handler)(notification);
        }
    }));
}
