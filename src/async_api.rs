#![cfg(feature = "async")]

use core::ffi::{c_char, c_void};
use std::ptr;

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::capability::{discovered_ci_devices, CiDeviceInfo};
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
        context_retain: Option<unsafe extern "C" fn(*mut c_void)>,
        context_release: Option<unsafe extern "C" fn(*mut c_void)>,
        out_client: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    fn cmr_ci_discovery_subscribe(
        callback: Option<unsafe extern "C" fn(*mut c_void)>,
        ctx: *mut c_void,
        context_retain: Option<unsafe extern "C" fn(*mut c_void)>,
        context_release: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut c_void;
    fn cmr_ci_discovery_unsubscribe(handle: *mut c_void);
}

type StreamContext<T> = CallbackContext<AsyncStreamSender<T>>;

#[derive(Debug, Clone)]
/// Wraps `MIDIEventList`.
pub struct OwnedEventList {
    /// Mirrors the CoreMIDI protocol field.
    pub protocol: MidiProtocol,
    /// Mirrors the CoreMIDI packets field.
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
    /// server thread**.  If strict real-time behaviour is required, receive
    /// through a [`MidiEventReceiver`](crate::receiver::MidiEventReceiver)
    /// from [`MidiClient::input_port_with_receiver`](crate::MidiClient::input_port_with_receiver)
    /// or [`MidiClient::virtual_destination_with_receiver`](crate::MidiClient::virtual_destination_with_receiver),
    /// which copy each packet into a preallocated ring without allocating.
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
            // CoreMIDI packs event packets compactly: each occupies only
            // `wordCount` words, not the full fixed-size `[u32; 64]` declared on
            // `MIDIEventPacket`.  Reading the whole struct by value would read
            // past the end of the variable-length event-list buffer for the
            // trailing packet (an out-of-bounds read on the real-time server
            // thread).  Copy the header plus only the valid words instead,
            // clamping `wordCount` to the struct capacity.
            let time_stamp = ptr::addr_of!((*packet_ptr).timeStamp).read_unaligned();
            let word_count = ptr::addr_of!((*packet_ptr).wordCount).read_unaligned();
            let valid = (word_count as usize).min(64);
            let mut packet = ffi::MIDIEventPacket {
                timeStamp: time_stamp,
                wordCount: word_count,
                words: [0; 64],
            };
            let words_src = ptr::addr_of!((*packet_ptr).words).cast::<u32>();
            for (i, slot) in packet.words[..valid].iter_mut().enumerate() {
                *slot = words_src.add(i).read_unaligned();
            }
            packets.push(packet);
            packet_ptr = ffi::MIDIEventPacketNext(packet_ptr);
        }

        Some(Self { protocol, packets })
    }
}

#[derive(Debug)]
/// Wraps `MIDIEventList`.
///
/// Each received event list is copied into a heap-allocated [`OwnedEventList`] on the
/// CoreMIDI receive thread; use
/// [`MidiClient::input_port_with_receiver`](crate::MidiClient::input_port_with_receiver)
/// to receive without allocating.
pub struct MidiEventStream {
    source: ffi::MIDIEndpointRef,
    port: ffi::MIDIPortRef,
    stream: BoundedAsyncStream<OwnedEventList>,
    context: StreamContext<OwnedEventList>,
}

impl MidiEventStream {
    /// Wraps `MIDIInputPortCreateWithProtocol`.
    pub fn subscribe(
        client: ffi::MIDIClientRef,
        source: ffi::MIDIEndpointRef,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<Self> {
        let (stream, context) = new_stream_pair(capacity)?;
        let port = private::create_receive_object(
            private::cmr_input_port_create_with_protocol,
            client,
            &format!("coremidi-rs async event stream {source}"),
            protocol,
            event_list_stream_callback,
            &context,
        )?;

        let connect_result = result_from_status(unsafe {
            ffi::MIDIPortConnectSource(port, source, ptr::null_mut())
        });
        if let Err(error) = connect_result {
            context.deactivate();
            let _ = unsafe { ffi::MIDIPortDispose(port) };
            return Err(error);
        }

        Ok(Self {
            source,
            port,
            stream,
            context,
        })
    }
}

impl Drop for MidiEventStream {
    fn drop(&mut self) {
        self.context.deactivate();
        let _ = unsafe { ffi::MIDIPortDisconnectSource(self.port, self.source) };
        let _ = unsafe { ffi::MIDIPortDispose(self.port) };
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI MIDI virtual destination stream payload.
///
/// Each received event list is copied into a heap-allocated [`OwnedEventList`] on the
/// CoreMIDI receive thread; use
/// [`MidiClient::virtual_destination_with_receiver`](crate::MidiClient::virtual_destination_with_receiver)
/// to receive without allocating.
pub struct MidiVirtualDestinationStream {
    endpoint: ffi::MIDIEndpointRef,
    stream: BoundedAsyncStream<OwnedEventList>,
    context: StreamContext<OwnedEventList>,
}

impl MidiVirtualDestinationStream {
    /// Wraps `MIDIDestinationCreateWithProtocol`.
    pub fn create(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
        capacity: usize,
    ) -> MidiResult<Self> {
        let (stream, context) = new_stream_pair(capacity)?;
        let endpoint = private::create_receive_object(
            private::cmr_destination_create_with_protocol,
            client,
            name,
            protocol,
            event_list_stream_callback,
            &context,
        )?;
        Ok(Self {
            endpoint,
            stream,
            context,
        })
    }

    #[must_use]
    /// Returns the wrapped `MIDIEndpointRef`.
    pub const fn endpoint(&self) -> ffi::MIDIEndpointRef {
        self.endpoint
    }
}

impl Drop for MidiVirtualDestinationStream {
    fn drop(&mut self) {
        self.context.deactivate();
        let _ = unsafe { ffi::MIDIEndpointDispose(self.endpoint) };
    }
}

#[derive(Debug)]
/// Mirrors the CoreMIDI MIDI client notification stream payload.
pub struct MidiClientNotificationStream {
    bridged_client: *mut c_void,
    stream: BoundedAsyncStream<Notification>,
    context: StreamContext<Notification>,
}

impl MidiClientNotificationStream {
    /// Wraps `MIDIClientCreateWithBlock`.
    pub fn subscribe(name: &str, capacity: usize) -> MidiResult<Self> {
        let (stream, context) = new_stream_pair(capacity)?;
        let bridged_client = notification_client(name, notification_stream_callback, &context)?;
        Ok(Self {
            bridged_client,
            stream,
            context,
        })
    }
}

impl Drop for MidiClientNotificationStream {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { private::release_swift_object(self.bridged_client) };
    }
}

unsafe impl Send for MidiClientNotificationStream {}
unsafe impl Sync for MidiClientNotificationStream {}

#[derive(Debug)]
/// Mirrors the CoreMIDI MIDI CI discovery stream payload.
pub struct MidiCIDiscoveryStream {
    handle: *mut c_void,
    stream: BoundedAsyncStream<Vec<CiDeviceInfo>>,
    context: StreamContext<Vec<CiDeviceInfo>>,
}

impl MidiCIDiscoveryStream {
    #[must_use]
    /// Wraps `MIDICIDeviceManager` discovery notifications.
    pub fn subscribe(capacity: usize) -> Option<Self> {
        let (stream, context) = new_stream_pair(capacity).ok()?;
        let handle = unsafe {
            cmr_ci_discovery_subscribe(
                Some(ci_discovery_stream_callback),
                context.as_ptr(),
                Some(StreamContext::<Vec<CiDeviceInfo>>::RETAIN),
                Some(StreamContext::<Vec<CiDeviceInfo>>::RELEASE),
            )
        };
        if handle.is_null() {
            return None;
        }
        Some(Self {
            handle,
            stream,
            context,
        })
    }
}

impl Drop for MidiCIDiscoveryStream {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { cmr_ci_discovery_unsubscribe(self.handle) };
    }
}

unsafe impl Send for MidiCIDiscoveryStream {}
unsafe impl Sync for MidiCIDiscoveryStream {}

#[derive(Debug)]
/// Mirrors the CoreMIDI MIDI thru connection stream payload.
pub struct MidiThruConnectionStream {
    bridged_client: *mut c_void,
    stream: BoundedAsyncStream<()>,
    context: StreamContext<()>,
}

impl MidiThruConnectionStream {
    /// Wraps `MIDIClientCreateWithBlock` notifications for `kMIDIMsgThruConnectionsChanged`.
    pub fn subscribe(name: &str, capacity: usize) -> MidiResult<Self> {
        let (stream, context) = new_stream_pair(capacity)?;
        let bridged_client = notification_client(name, thru_connection_stream_callback, &context)?;
        Ok(Self {
            bridged_client,
            stream,
            context,
        })
    }
}

impl Drop for MidiThruConnectionStream {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { private::release_swift_object(self.bridged_client) };
    }
}

unsafe impl Send for MidiThruConnectionStream {}
unsafe impl Sync for MidiThruConnectionStream {}

macro_rules! impl_stream_accessors {
    ($name:ident, $item:ty) => {
        impl $name {
            #[must_use]
            /// Wraps the next-item accessor for the CoreMIDI async stream.
            pub const fn next(&self) -> NextItem<'_, $item> {
                self.stream.next()
            }

            #[must_use]
            /// Wraps the nonblocking next-item accessor for the CoreMIDI async stream.
            pub fn try_next(&self) -> Option<$item> {
                self.stream.try_next()
            }

            #[must_use]
            /// Wraps the buffered-item count for the CoreMIDI async stream.
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

fn new_stream_pair<T: Send + 'static>(
    capacity: usize,
) -> MidiResult<(BoundedAsyncStream<T>, StreamContext<T>)> {
    if capacity == 0 {
        return Err(MidiError::InvalidArgument(
            "async stream capacity must be > 0".into(),
        ));
    }
    let (stream, sender) = BoundedAsyncStream::new(capacity);
    Ok((stream, CallbackContext::new(sender)))
}

fn notification_client<T: Send + 'static>(
    name: &str,
    callback: unsafe extern "C" fn(*mut c_void, *const c_char),
    context: &StreamContext<T>,
) -> MidiResult<*mut c_void> {
    let name = private::to_cstring(name)?;
    let mut bridged_client = ptr::null_mut();
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_client_new_with_notifications(
                name.as_ptr(),
                Some(callback),
                context.as_ptr(),
                Some(StreamContext::<T>::RETAIN),
                Some(StreamContext::<T>::RELEASE),
                &raw mut bridged_client,
                &raw mut error,
            ),
            error,
        )
    }?;
    Ok(bridged_client)
}

unsafe extern "C" fn event_list_stream_callback(
    context: *mut c_void,
    event_list: *const ffi::MIDIEventList,
    _src_conn_ref_con: *mut c_void,
) {
    let _ = unsafe {
        StreamContext::<OwnedEventList>::with(context, "coremidi event list stream", |sender| {
            if let Some(event_list) = OwnedEventList::copy_from(event_list) {
                sender.push(event_list);
            }
        })
    };
}

unsafe extern "C" fn notification_stream_callback(
    context: *mut c_void,
    payload_json: *const c_char,
) {
    let _ = unsafe {
        StreamContext::<Notification>::with(context, "coremidi notification stream", |sender| {
            if let Some(notification) = Notification::from_bridge_payload(payload_json) {
                sender.push(notification);
            }
        })
    };
}

unsafe extern "C" fn thru_connection_stream_callback(
    context: *mut c_void,
    payload_json: *const c_char,
) {
    let _ = unsafe {
        StreamContext::<()>::with(context, "coremidi thru connection stream", |sender| {
            if matches!(
                Notification::from_bridge_payload(payload_json),
                Some(Notification::ThruConnectionsChanged)
            ) {
                sender.push(());
            }
        })
    };
}

unsafe extern "C" fn ci_discovery_stream_callback(context: *mut c_void) {
    let _ = unsafe {
        StreamContext::<Vec<CiDeviceInfo>>::with(
            context,
            "coremidi CI discovery stream",
            |sender| {
                if let Ok(devices) = discovered_ci_devices() {
                    sender.push(devices);
                }
            },
        )
    };
}
