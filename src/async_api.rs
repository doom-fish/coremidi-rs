#![cfg(feature = "async")]

use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::OnceLock;

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::capability::{discovered_ci_devices, CiDeviceInfo};
use crate::cf::OwnedCFString;
use crate::error::{result_from_status, MidiError, MidiResult};
use crate::ffi;
use crate::notification::Notification;
use crate::packet::MidiProtocol;
use crate::private;

extern "C" {
    fn cmr_client_new_with_notifications(
        name: *const c_char,
        callback: Option<unsafe extern "C" fn(*mut c_void, *const c_char)>,
        user_info: *mut c_void,
        out_client: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_vdest_stream_create(
        client: ffi::MIDIClientRef,
        name: *const c_char,
        protocol: ffi::MIDIProtocolID,
        callback: Option<unsafe extern "C" fn(*const ffi::MIDIEventList, *mut c_void)>,
        ctx: *mut c_void,
        out_endpoint: *mut ffi::MIDIEndpointRef,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ci_discovery_subscribe(
        callback: Option<unsafe extern "C" fn(*mut c_void)>,
        ctx: *mut c_void,
    ) -> *mut c_void;
    fn cmr_ci_discovery_unsubscribe(handle: *mut c_void);
    static _NSConcreteGlobalBlock: c_void;
}

#[derive(Debug, Clone)]
pub struct OwnedEventList {
    pub protocol: MidiProtocol,
    pub packets: Vec<ffi::MIDIEventPacket>,
}

impl OwnedEventList {
    /// Copy an event list from a raw pointer. Returns `None` if the pointer is
    /// null or the event list carries an unknown protocol ID.
    ///
    /// # Real-time safety
    ///
    /// This function allocates a [`Vec`] to hold the copied packets.  When
    /// called from the `MidiEventStream` or `MidiVirtualDestinationStream`
    /// receive callbacks it therefore **allocates on the CoreMIDI real-time
    /// server thread**.  If strict real-time behaviour is required, use a
    /// raw [`MidiInputPort`](crate::port::MidiInputPort) with a custom
    /// [`MidiProtocolReadProc`](crate::port::MidiProtocolReadProc) that
    /// operates on the borrowed `*const MIDIEventList` directly, without
    /// copying.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a valid, fully initialised `MIDIEventList` for at
    /// least the duration of the call.
    pub unsafe fn copy_from(ptr: *const ffi::MIDIEventList) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        let protocol_raw = ptr::addr_of!((*ptr).protocol).read_unaligned();
        let protocol = MidiProtocol::from_raw(protocol_raw)?;
        let num_packets = ptr::addr_of!((*ptr).numPackets).read_unaligned() as usize;
        let mut packets = Vec::with_capacity(num_packets);
        let mut packet_ptr = ptr::addr_of!((*ptr).packet).cast::<ffi::MIDIEventPacket>();

        for _ in 0..num_packets {
            packets.push(packet_ptr.read_unaligned());
            packet_ptr = ffi::MIDIEventPacketNext(packet_ptr);
        }

        Some(Self { protocol, packets })
    }
}

#[derive(Debug)]
pub struct MidiEventStream {
    source: ffi::MIDIEndpointRef,
    port: ffi::MIDIPortRef,
    stream: BoundedAsyncStream<OwnedEventList>,
    sender_ptr: *mut AsyncStreamSender<OwnedEventList>,
}

impl MidiEventStream {
    pub fn subscribe(
        client: ffi::MIDIClientRef,
        source: ffi::MIDIEndpointRef,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<Self> {
        if capacity == 0 {
            return Err(MidiError::InvalidArgument(
                "async stream capacity must be > 0".into(),
            ));
        }

        let port_name = format!("coremidi-rs async event stream {source}");
        let name = OwnedCFString::new(&port_name)?;
        let (stream, sender_ptr) = new_stream_pair(capacity);
        let mut port = 0;

        let create_result = result_from_status(unsafe {
            ffi::MIDIInputPortCreateWithProtocol(
                client,
                name.as_raw(),
                protocol.as_raw(),
                &mut port,
                event_stream_receive_block(),
            )
        });

        if let Err(error) = create_result {
            unsafe { drop_sender(sender_ptr) };
            return Err(error);
        }

        let connect_result = result_from_status(unsafe {
            ffi::MIDIPortConnectSource(port, source, sender_ptr.cast::<c_void>())
        });

        match connect_result {
            Ok(()) => Ok(Self {
                source,
                port,
                stream,
                sender_ptr,
            }),
            Err(error) => {
                let _ = unsafe { ffi::MIDIPortDispose(port) };
                unsafe { drop_sender(sender_ptr) };
                Err(error)
            }
        }
    }
}

impl Drop for MidiEventStream {
    fn drop(&mut self) {
        // SAFETY: `self.port` and `self.source` are valid refs created during
        // `subscribe`.  Disconnecting and disposing the port before dropping
        // the sender ensures CoreMIDI delivers no further callbacks that
        // dereference `sender_ptr`.
        let _ = unsafe { ffi::MIDIPortDisconnectSource(self.port, self.source) };
        let _ = unsafe { ffi::MIDIPortDispose(self.port) };
        // SAFETY: `sender_ptr` was produced by `Box::into_raw` in
        // `new_stream_pair` and is freed exactly once here.
        unsafe { drop_sender(self.sender_ptr) };
    }
}

unsafe impl Send for MidiEventStream {}
unsafe impl Sync for MidiEventStream {}

#[derive(Debug)]
pub struct MidiVirtualDestinationStream {
    endpoint: ffi::MIDIEndpointRef,
    stream: BoundedAsyncStream<OwnedEventList>,
    sender_ptr: *mut AsyncStreamSender<OwnedEventList>,
}

impl MidiVirtualDestinationStream {
    pub fn create(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<Self> {
        if capacity == 0 {
            return Err(MidiError::InvalidArgument(
                "async stream capacity must be > 0".into(),
            ));
        }

        let name = private::to_cstring(name)?;
        let (stream, sender_ptr) = new_stream_pair(capacity);
        let mut endpoint = 0;
        let mut error = ptr::null_mut();

        let result = unsafe {
            private::swift_result(
                cmr_vdest_stream_create(
                    client,
                    name.as_ptr(),
                    protocol.as_raw(),
                    Some(virtual_destination_stream_callback),
                    sender_ptr.cast::<c_void>(),
                    &mut endpoint,
                    &mut error,
                ),
                error,
            )
        };

        match result {
            Ok(()) => Ok(Self {
                endpoint,
                stream,
                sender_ptr,
            }),
            Err(error) => {
                unsafe { drop_sender(sender_ptr) };
                Err(error)
            }
        }
    }

    #[must_use]
    pub const fn endpoint(&self) -> ffi::MIDIEndpointRef {
        self.endpoint
    }
}

impl Drop for MidiVirtualDestinationStream {
    fn drop(&mut self) {
        // SAFETY: `self.endpoint` is a valid `MIDIEndpointRef` created in
        // `create` via the Swift bridge.  Disposing the endpoint before
        // dropping the sender ensures no further callbacks reference `sender_ptr`.
        let _ = unsafe { ffi::MIDIEndpointDispose(self.endpoint) };
        // SAFETY: `sender_ptr` was produced by `Box::into_raw` in
        // `new_stream_pair` and is freed exactly once here.
        unsafe { drop_sender(self.sender_ptr) };
    }
}

unsafe impl Send for MidiVirtualDestinationStream {}
unsafe impl Sync for MidiVirtualDestinationStream {}

#[derive(Debug)]
pub struct MidiClientNotificationStream {
    bridged_client: *mut c_void,
    stream: BoundedAsyncStream<Notification>,
    sender_ptr: *mut AsyncStreamSender<Notification>,
}

impl MidiClientNotificationStream {
    pub fn subscribe(name: &str, capacity: usize) -> MidiResult<Self> {
        if capacity == 0 {
            return Err(MidiError::InvalidArgument(
                "async stream capacity must be > 0".into(),
            ));
        }

        let name = private::to_cstring(name)?;
        let (stream, sender_ptr) = new_stream_pair(capacity);
        let mut bridged_client = ptr::null_mut();
        let mut error = ptr::null_mut();

        let result = unsafe {
            private::swift_result(
                cmr_client_new_with_notifications(
                    name.as_ptr(),
                    Some(notification_stream_callback),
                    sender_ptr.cast::<c_void>(),
                    &mut bridged_client,
                    &mut error,
                ),
                error,
            )
        };

        match result {
            Ok(()) => Ok(Self {
                bridged_client,
                stream,
                sender_ptr,
            }),
            Err(error) => {
                unsafe { drop_sender(sender_ptr) };
                Err(error)
            }
        }
    }
}

impl Drop for MidiClientNotificationStream {
    fn drop(&mut self) {
        // SAFETY: `self.bridged_client` is an ARC-managed Swift object.
        // Releasing it disposes the underlying `MIDIClientRef` and tears down
        // the notification block.  The sender is dropped afterwards so that
        // any in-flight callback that races the ARC release (e.g. a
        // `kMIDIMsgSetupChanged` fired during `MIDIRestart`) still finds a
        // live sender and does not dereference freed memory.
        //
        // Note: CoreMIDI does not guarantee that in-flight callbacks queued
        // before disposal are drained synchronously.  The `sender_ptr` null-
        // check inside `notification_stream_callback` provides a best-effort
        // guard, but strict safety against a `MIDIRestart`-concurrent drop
        // would require a barrier (e.g. a serial dispatch queue) not currently
        // present in this bridge.
        unsafe { private::release_swift_object(self.bridged_client) };
        // SAFETY: `sender_ptr` was produced by `Box::into_raw` in
        // `new_stream_pair` and is freed exactly once here.
        unsafe { drop_sender(self.sender_ptr) };
    }
}

unsafe impl Send for MidiClientNotificationStream {}
unsafe impl Sync for MidiClientNotificationStream {}

#[derive(Debug)]
pub struct MidiCIDiscoveryStream {
    handle: *mut c_void,
    stream: BoundedAsyncStream<Vec<CiDeviceInfo>>,
    sender_ptr: *mut AsyncStreamSender<Vec<CiDeviceInfo>>,
}

impl MidiCIDiscoveryStream {
    #[must_use]
    pub fn subscribe(capacity: usize) -> Option<Self> {
        if capacity == 0 {
            return None;
        }

        let (stream, sender_ptr) = new_stream_pair(capacity);
        let handle = unsafe {
            cmr_ci_discovery_subscribe(
                Some(ci_discovery_stream_callback),
                sender_ptr.cast::<c_void>(),
            )
        };

        if handle.is_null() {
            unsafe { drop_sender(sender_ptr) };
            return None;
        }

        Some(Self {
            handle,
            stream,
            sender_ptr,
        })
    }
}

impl Drop for MidiCIDiscoveryStream {
    fn drop(&mut self) {
        // SAFETY: `self.handle` is a non-null opaque pointer returned by
        // `cmr_ci_discovery_subscribe`.  Unsubscribing before dropping the
        // sender matches the subscribe ordering and ensures no further
        // callbacks reference `sender_ptr`.
        unsafe { cmr_ci_discovery_unsubscribe(self.handle) };
        // SAFETY: `sender_ptr` was produced by `Box::into_raw` in
        // `new_stream_pair` and is freed exactly once here.
        unsafe { drop_sender(self.sender_ptr) };
    }
}

unsafe impl Send for MidiCIDiscoveryStream {}
unsafe impl Sync for MidiCIDiscoveryStream {}

#[derive(Debug)]
pub struct MidiThruConnectionStream {
    bridged_client: *mut c_void,
    stream: BoundedAsyncStream<()>,
    sender_ptr: *mut AsyncStreamSender<()>,
}

impl MidiThruConnectionStream {
    pub fn subscribe(name: &str, capacity: usize) -> MidiResult<Self> {
        if capacity == 0 {
            return Err(MidiError::InvalidArgument(
                "async stream capacity must be > 0".into(),
            ));
        }

        let name = private::to_cstring(name)?;
        let (stream, sender_ptr) = new_stream_pair(capacity);
        let mut bridged_client = ptr::null_mut();
        let mut error = ptr::null_mut();

        let result = unsafe {
            private::swift_result(
                cmr_client_new_with_notifications(
                    name.as_ptr(),
                    Some(thru_connection_stream_callback),
                    sender_ptr.cast::<c_void>(),
                    &mut bridged_client,
                    &mut error,
                ),
                error,
            )
        };

        match result {
            Ok(()) => Ok(Self {
                bridged_client,
                stream,
                sender_ptr,
            }),
            Err(error) => {
                unsafe { drop_sender(sender_ptr) };
                Err(error)
            }
        }
    }
}

impl Drop for MidiThruConnectionStream {
    fn drop(&mut self) {
        // SAFETY: same ordering rationale as `MidiClientNotificationStream`.
        // Release the Swift-managed MIDI client before the sender so that any
        // in-flight `ThruConnectionsChanged` notification still finds a live
        // sender.  See the `MidiClientNotificationStream` drop comment for the
        // `MIDIRestart` race caveat.
        unsafe { private::release_swift_object(self.bridged_client) };
        // SAFETY: `sender_ptr` was produced by `Box::into_raw` in
        // `new_stream_pair` and is freed exactly once here.
        unsafe { drop_sender(self.sender_ptr) };
    }
}

unsafe impl Send for MidiThruConnectionStream {}
unsafe impl Sync for MidiThruConnectionStream {}

macro_rules! impl_stream_accessors {
    ($name:ident, $item:ty) => {
        impl $name {
            #[must_use]
            pub const fn next(&self) -> NextItem<'_, $item> {
                self.stream.next()
            }

            #[must_use]
            pub fn try_next(&self) -> Option<$item> {
                self.stream.try_next()
            }

            #[must_use]
            pub fn buffered_count(&self) -> usize {
                self.stream.buffered_count()
            }
        }
    };
}

impl_stream_accessors!(MidiEventStream, OwnedEventList);
impl_stream_accessors!(MidiVirtualDestinationStream, OwnedEventList);
impl_stream_accessors!(MidiClientNotificationStream, Notification);
impl_stream_accessors!(MidiCIDiscoveryStream, Vec<CiDeviceInfo>);
impl_stream_accessors!(MidiThruConnectionStream, ());

fn new_stream_pair<T>(capacity: usize) -> (BoundedAsyncStream<T>, *mut AsyncStreamSender<T>) {
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    (stream, Box::into_raw(Box::new(sender)))
}

unsafe fn drop_sender<T>(sender_ptr: *mut AsyncStreamSender<T>) {
    if !sender_ptr.is_null() {
        // SAFETY: caller guarantees `sender_ptr` was produced by
        // `Box::into_raw` and has not been freed before.
        drop(Box::from_raw(sender_ptr));
    }
}

unsafe fn copy_event_list_to_sender(evtlist: *const ffi::MIDIEventList, ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }

    // SAFETY: `ctx` is the `AsyncStreamSender<OwnedEventList>` pointer stored
    // as `connRefCon` via `MIDIPortConnectSource`.  The sender outlives all
    // callbacks because the port/endpoint is disposed before the sender is
    // freed (see the Drop implementations).  We hold a shared reference only
    // for the duration of this call; the sender's internal channel handles
    // concurrent access.
    //
    // Real-time note: `OwnedEventList::copy_from` allocates a Vec here on the
    // CoreMIDI real-time server thread.  See `OwnedEventList::copy_from` for
    // the trade-off and the low-allocation alternative.
    let sender = &*ctx.cast::<AsyncStreamSender<OwnedEventList>>();
    if let Some(event_list) = OwnedEventList::copy_from(evtlist) {
        sender.push(event_list);
    }
}

unsafe fn parse_notification(payload_json: *const c_char) -> Option<Notification> {
    if payload_json.is_null() {
        return None;
    }

    let payload = CStr::from_ptr(payload_json).to_string_lossy().into_owned();
    Notification::from_json_str(&payload).ok()
}

extern "C" fn event_stream_receive_block_invoke(
    _block: *const c_void,
    evtlist: *const ffi::MIDIEventList,
    src_conn_ref_con: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        copy_event_list_to_sender(evtlist, src_conn_ref_con);
    }));
}

unsafe extern "C" fn virtual_destination_stream_callback(
    evtlist: *const ffi::MIDIEventList,
    ctx: *mut c_void,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        copy_event_list_to_sender(evtlist, ctx);
    }));
}

unsafe extern "C" fn notification_stream_callback(
    user_info: *mut c_void,
    payload_json: *const c_char,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        if user_info.is_null() {
            return;
        }

        let sender = &*user_info.cast::<AsyncStreamSender<Notification>>();
        if let Some(notification) = parse_notification(payload_json) {
            sender.push(notification);
        }
    }));
}

unsafe extern "C" fn thru_connection_stream_callback(
    user_info: *mut c_void,
    payload_json: *const c_char,
) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        if user_info.is_null() {
            return;
        }

        let sender = &*user_info.cast::<AsyncStreamSender<()>>();
        if matches!(
            parse_notification(payload_json),
            Some(Notification::ThruConnectionsChanged)
        ) {
            sender.push(());
        }
    }));
}

unsafe extern "C" fn ci_discovery_stream_callback(ctx: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        if ctx.is_null() {
            return;
        }

        let sender = &*ctx.cast::<AsyncStreamSender<Vec<CiDeviceInfo>>>();
        if let Ok(devices) = discovered_ci_devices() {
            sender.push(devices);
        }
    }));
}

#[repr(C)]
struct BlockDescriptor {
    reserved: usize,
    size: usize,
}

#[repr(C)]
struct GlobalReceiveBlock {
    isa: *const c_void,
    flags: i32,
    reserved: i32,
    invoke: extern "C" fn(*const c_void, *const ffi::MIDIEventList, *mut c_void),
    descriptor: *const BlockDescriptor,
}

unsafe impl Send for GlobalReceiveBlock {}
unsafe impl Sync for GlobalReceiveBlock {}

const BLOCK_IS_GLOBAL: i32 = 1 << 28;

static EVENT_STREAM_BLOCK_DESCRIPTOR: BlockDescriptor = BlockDescriptor {
    reserved: 0,
    size: core::mem::size_of::<GlobalReceiveBlock>(),
};

fn event_stream_receive_block() -> *const c_void {
    static BLOCK: OnceLock<GlobalReceiveBlock> = OnceLock::new();
    std::ptr::from_ref(BLOCK.get_or_init(|| GlobalReceiveBlock {
        isa: ptr::addr_of!(_NSConcreteGlobalBlock).cast(),
        flags: BLOCK_IS_GLOBAL,
        reserved: 0,
        invoke: event_stream_receive_block_invoke,
        descriptor: &EVENT_STREAM_BLOCK_DESCRIPTOR,
    }))
    .cast::<c_void>()
}
