use core::ffi::c_void;
use std::ptr;
use std::sync::{Mutex, OnceLock};

use crate::cf::OwnedCFString;
use crate::error::{result_from_status, MidiError, MidiResult};
use crate::ffi;
use crate::object::{MidiEndpoint, MidiObject};
use crate::packet::{EventListBuffer, MidiProtocol, PacketListBuffer};

pub type MidiProtocolReadProc = unsafe extern "C" fn(*const ffi::MIDIEventList, *mut c_void);

#[derive(Debug)]
pub struct MidiClient {
    raw: ffi::MIDIClientRef,
}

impl MidiClient {
    pub fn new(name: &str) -> MidiResult<Self> {
        unsafe { Self::with_notify(name, None, ptr::null_mut()) }
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
        Ok(Self { raw })
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
        let _ = unsafe { ffi::MIDIClientDispose(self.raw) };
    }
}

impl MidiObject for MidiClient {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
pub struct MidiInputPort {
    raw: ffi::MIDIPortRef,
    protocol_mode: bool,
    protocol_contexts: Mutex<Vec<*mut ProtocolConnectionContext>>,
}

impl MidiInputPort {
    unsafe fn new_legacy(
        client: ffi::MIDIClientRef,
        name: &str,
        read_proc: ffi::MIDIReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(ffi::MIDIInputPortCreate(
            client,
            name.as_raw(),
            read_proc,
            ref_con,
            &mut raw,
        ))?;
        Ok(Self {
            raw,
            protocol_mode: false,
            protocol_contexts: Mutex::new(Vec::new()),
        })
    }

    fn new_with_protocol(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe {
            ffi::MIDIInputPortCreateWithProtocol(
                client,
                name.as_raw(),
                protocol.as_raw(),
                &mut raw,
                protocol_receive_block(),
            )
        })?;
        Ok(Self {
            raw,
            protocol_mode: true,
            protocol_contexts: Mutex::new(Vec::new()),
        })
    }

    /// Connect a source to this port using a raw CoreMIDI `connRefCon`.
    ///
    /// # Safety
    ///
    /// `conn_ref_con` must remain valid until the source is disconnected or the
    /// port is dropped.
    pub unsafe fn connect_source(
        &self,
        source: MidiEndpoint,
        conn_ref_con: *mut c_void,
    ) -> MidiResult<()> {
        result_from_status(ffi::MIDIPortConnectSource(
            self.raw,
            source.raw(),
            conn_ref_con,
        ))
    }

    /// Connect a source to a protocol-aware input port using a C callback and
    /// user refcon.
    ///
    /// # Safety
    ///
    /// `callback` and `ref_con` must remain valid until the port is dropped.
    pub unsafe fn connect_source_with_protocol_callback(
        &self,
        source: MidiEndpoint,
        callback: MidiProtocolReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<()> {
        if !self.protocol_mode {
            return Err(MidiError::Unsupported(
                "connect_source_with_protocol_callback requires a protocol input port".into(),
            ));
        }

        let context = Box::into_raw(Box::new(ProtocolConnectionContext { callback, ref_con }));
        let result = result_from_status(ffi::MIDIPortConnectSource(
            self.raw,
            source.raw(),
            context.cast(),
        ));
        match result {
            Ok(()) => {
                if let Ok(mut contexts) = self.protocol_contexts.lock() {
                    contexts.push(context);
                }
                Ok(())
            }
            Err(error) => {
                drop(Box::from_raw(context));
                Err(error)
            }
        }
    }

    pub fn disconnect_source(&self, source: MidiEndpoint) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDIPortDisconnectSource(self.raw, source.raw()) })
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIPortRef {
        self.raw
    }
}

impl Drop for MidiInputPort {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIPortDispose(self.raw) };
        if let Ok(mut contexts) = self.protocol_contexts.lock() {
            for context in contexts.drain(..) {
                unsafe {
                    drop(Box::from_raw(context));
                }
            }
        }
    }
}

impl MidiObject for MidiInputPort {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
pub struct MidiOutputPort {
    raw: ffi::MIDIPortRef,
}

impl MidiOutputPort {
    fn new(client: ffi::MIDIClientRef, name: &str) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe { ffi::MIDIOutputPortCreate(client, name.as_raw(), &mut raw) })?;
        Ok(Self { raw })
    }

    pub fn send(&self, dest: MidiEndpoint, packets: &PacketListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDISend(self.raw, dest.raw(), packets.as_ptr()) })
    }

    pub fn send_event_list(&self, dest: MidiEndpoint, events: &EventListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDISendEventList(self.raw, dest.raw(), events.as_ptr()) })
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIPortRef {
        self.raw
    }
}

impl Drop for MidiOutputPort {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIPortDispose(self.raw) };
    }
}

impl MidiObject for MidiOutputPort {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
pub struct VirtualSource {
    raw: ffi::MIDIEndpointRef,
}

impl VirtualSource {
    fn new(client: ffi::MIDIClientRef, name: &str) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe { ffi::MIDISourceCreate(client, name.as_raw(), &mut raw) })?;
        Ok(Self { raw })
    }

    pub fn received(&self, packets: &PacketListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDIReceived(self.raw, packets.as_ptr()) })
    }

    pub fn received_event_list(&self, events: &EventListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDIReceivedEventList(self.raw, events.as_ptr()) })
    }

    #[must_use]
    pub const fn endpoint(&self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(self.raw) }
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIEndpointRef {
        self.raw
    }
}

impl Drop for VirtualSource {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIEndpointDispose(self.raw) };
    }
}

impl MidiObject for VirtualSource {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
pub struct VirtualDestination {
    raw: ffi::MIDIEndpointRef,
}

impl VirtualDestination {
    unsafe fn new(
        client: ffi::MIDIClientRef,
        name: &str,
        read_proc: ffi::MIDIReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(ffi::MIDIDestinationCreate(
            client,
            name.as_raw(),
            read_proc,
            ref_con,
            &mut raw,
        ))?;
        Ok(Self { raw })
    }

    #[must_use]
    pub const fn endpoint(&self) -> MidiEndpoint {
        unsafe { MidiEndpoint::from_raw(self.raw) }
    }

    #[must_use]
    pub const fn raw(&self) -> ffi::MIDIEndpointRef {
        self.raw
    }
}

impl Drop for VirtualDestination {
    fn drop(&mut self) {
        let _ = unsafe { ffi::MIDIEndpointDispose(self.raw) };
    }
}

impl MidiObject for VirtualDestination {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

struct ProtocolConnectionContext {
    callback: MidiProtocolReadProc,
    ref_con: *mut c_void,
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

static RECEIVE_BLOCK_DESCRIPTOR: BlockDescriptor = BlockDescriptor {
    reserved: 0,
    size: core::mem::size_of::<GlobalReceiveBlock>(),
};

extern "C" {
    static _NSConcreteGlobalBlock: c_void;
}

extern "C" fn protocol_receive_block_invoke(
    _block: *const c_void,
    evtlist: *const ffi::MIDIEventList,
    src_conn_ref_con: *mut c_void,
) {
    if src_conn_ref_con.is_null() {
        return;
    }

    let context = unsafe { &*src_conn_ref_con.cast::<ProtocolConnectionContext>() };
    unsafe { (context.callback)(evtlist, context.ref_con) };
}

fn protocol_receive_block() -> *const c_void {
    static BLOCK: OnceLock<GlobalReceiveBlock> = OnceLock::new();
    std::ptr::from_ref(BLOCK.get_or_init(|| GlobalReceiveBlock {
        isa: ptr::addr_of!(_NSConcreteGlobalBlock).cast(),
        flags: BLOCK_IS_GLOBAL,
        reserved: 0,
        invoke: protocol_receive_block_invoke,
        descriptor: &RECEIVE_BLOCK_DESCRIPTOR,
    }))
    .cast::<c_void>()
}
