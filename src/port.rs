use core::ffi::c_void;
use std::ptr;
use std::sync::{Mutex, MutexGuard, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;

use crate::cf::OwnedCFString;
use crate::endpoint::MidiEndpoint;
use crate::error::{result_from_status, MidiError, MidiResult};
use crate::ffi;
use crate::packet::{EventListBuffer, MidiProtocol, PacketListBuffer};
use crate::private;
use crate::property::MidiObject;
use crate::receiver::{port_receiver_trampoline, EventSink};

extern "C" {
    fn cmr_flush_output(
        destination: ffi::MIDIEndpointRef,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
}

/// Callback signature matching the corresponding CoreMIDI receive proc.
pub type MidiProtocolReadProc = unsafe extern "C" fn(*const ffi::MIDIEventList, *mut c_void);

#[derive(Debug)]
/// Wraps `MIDIPortRef`.
pub struct MidiInputPort {
    raw: ffi::MIDIPortRef,
    kind: InputPortKind,
}

#[derive(Debug)]
enum InputPortKind {
    Legacy,
    Callbacks(CallbackContext<CallbackTable>),
    Receiver(CallbackContext<EventSink>),
}

impl MidiInputPort {
    pub(crate) unsafe fn new_legacy(
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
            &raw mut raw,
        ))?;
        Ok(Self {
            raw,
            kind: InputPortKind::Legacy,
        })
    }

    pub(crate) fn new_with_protocol(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
    ) -> MidiResult<Self> {
        let table = CallbackContext::new(CallbackTable::default());
        let raw = private::create_receive_object(
            private::cmr_input_port_create_with_protocol,
            client,
            name,
            protocol,
            protocol_callback_trampoline,
            &table,
        )?;
        Ok(Self {
            raw,
            kind: InputPortKind::Callbacks(table),
        })
    }

    pub(crate) fn new_with_receiver(
        client: ffi::MIDIClientRef,
        name: &str,
        protocol: MidiProtocol,
        sink: CallbackContext<EventSink>,
    ) -> MidiResult<Self> {
        let raw = private::create_receive_object(
            private::cmr_input_port_create_with_protocol,
            client,
            name,
            protocol,
            port_receiver_trampoline,
            &sink,
        )?;
        Ok(Self {
            raw,
            kind: InputPortKind::Receiver(sink),
        })
    }

    /// Wraps `MIDIPortConnectSource`.
    pub unsafe fn connect_source(
        &self,
        source: MidiEndpoint,
        conn_ref_con: *mut c_void,
    ) -> MidiResult<()> {
        if !matches!(self.kind, InputPortKind::Legacy) {
            return Err(MidiError::Unsupported(
                "connect_source requires a port created with MidiClient::input_port".into(),
            ));
        }
        result_from_status(ffi::MIDIPortConnectSource(
            self.raw,
            source.raw(),
            conn_ref_con,
        ))
    }

    /// Wraps `MIDIPortConnectSource`.
    pub unsafe fn connect_source_with_protocol_callback(
        &self,
        source: MidiEndpoint,
        callback: MidiProtocolReadProc,
        ref_con: *mut c_void,
    ) -> MidiResult<()> {
        let InputPortKind::Callbacks(table) = &self.kind else {
            return Err(MidiError::Unsupported(
                "connect_source_with_protocol_callback requires a protocol input port".into(),
            ));
        };

        let previous = table.get().replace(ProtocolConnection {
            source: source.raw(),
            callback,
            ref_con,
        });
        let result = result_from_status(ffi::MIDIPortConnectSource(
            self.raw,
            source.raw(),
            source_ref_con(source.raw()),
        ));
        if result.is_err() {
            match previous {
                Some(previous) => {
                    table.get().replace(previous);
                }
                None => {
                    table.get().remove(source.raw());
                }
            }
        }
        result
    }

    pub fn connect(&self, source: MidiEndpoint) -> MidiResult<()> {
        if !matches!(self.kind, InputPortKind::Receiver(_)) {
            return Err(MidiError::Unsupported(
                "connect requires a port created with MidiClient::input_port_with_receiver".into(),
            ));
        }
        result_from_status(unsafe {
            ffi::MIDIPortConnectSource(self.raw, source.raw(), source_ref_con(source.raw()))
        })
    }

    /// Wraps `MIDIPortDisconnectSource`.
    pub fn disconnect_source(&self, source: MidiEndpoint) -> MidiResult<()> {
        let result =
            result_from_status(unsafe { ffi::MIDIPortDisconnectSource(self.raw, source.raw()) });
        if let InputPortKind::Callbacks(table) = &self.kind {
            table.get().remove(source.raw());
        }
        result
    }

    #[must_use]
    /// Returns the wrapped `MIDIPortRef`.
    pub const fn raw(&self) -> ffi::MIDIPortRef {
        self.raw
    }
}

impl Drop for MidiInputPort {
    fn drop(&mut self) {
        match &self.kind {
            InputPortKind::Legacy => {}
            InputPortKind::Callbacks(table) => table.deactivate(),
            InputPortKind::Receiver(sink) => sink.deactivate(),
        }
        let _ = unsafe { ffi::MIDIPortDispose(self.raw) };
        if let InputPortKind::Callbacks(table) = &self.kind {
            table.get().lock().clear();
        }
    }
}

impl MidiObject for MidiInputPort {
    fn raw_object(&self) -> ffi::MIDIObjectRef {
        self.raw
    }
}

#[derive(Debug)]
/// Wraps `MIDIPortRef`.
pub struct MidiOutputPort {
    raw: ffi::MIDIPortRef,
}

impl MidiOutputPort {
    pub(crate) fn new(client: ffi::MIDIClientRef, name: &str) -> MidiResult<Self> {
        let name = OwnedCFString::new(name)?;
        let mut raw = 0;
        result_from_status(unsafe {
            ffi::MIDIOutputPortCreate(client, name.as_raw(), &raw mut raw)
        })?;
        Ok(Self { raw })
    }

    /// Wraps `MIDISend`.
    pub fn send(&self, dest: MidiEndpoint, packets: &PacketListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDISend(self.raw, dest.raw(), packets.as_ptr()) })
    }

    /// Wraps `MIDISendEventList`.
    pub fn send_event_list(&self, dest: MidiEndpoint, events: &EventListBuffer) -> MidiResult<()> {
        result_from_status(unsafe { ffi::MIDISendEventList(self.raw, dest.raw(), events.as_ptr()) })
    }

    #[must_use]
    /// Returns the wrapped `MIDIPortRef`.
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

/// Wraps the CoreMIDI flush output operation for `MidiOutputPort`.
pub fn flush_output(destination: Option<MidiEndpoint>) -> MidiResult<()> {
    let mut error = ptr::null_mut();
    unsafe {
        private::swift_result(
            cmr_flush_output(destination.map_or(0, MidiEndpoint::raw), &raw mut error),
            error,
        )
    }
}

fn source_ref_con(source: ffi::MIDIEndpointRef) -> *mut c_void {
    source as usize as *mut c_void
}

#[derive(Default)]
struct CallbackTable {
    connections: Mutex<Vec<ProtocolConnection>>,
}

impl CallbackTable {
    fn lock(&self) -> MutexGuard<'_, Vec<ProtocolConnection>> {
        self.connections
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn replace(&self, connection: ProtocolConnection) -> Option<ProtocolConnection> {
        let mut connections = self.lock();
        if let Some(existing) = connections
            .iter_mut()
            .find(|existing| existing.source == connection.source)
        {
            return Some(std::mem::replace(existing, connection));
        }
        connections.push(connection);
        None
    }

    fn remove(&self, source: ffi::MIDIEndpointRef) -> Option<ProtocolConnection> {
        let mut connections = self.lock();
        let index = connections
            .iter()
            .position(|connection| connection.source == source)?;
        Some(connections.swap_remove(index))
    }
}

struct ProtocolConnection {
    source: ffi::MIDIEndpointRef,
    callback: MidiProtocolReadProc,
    ref_con: *mut c_void,
}

unsafe impl Send for ProtocolConnection {}

unsafe extern "C" fn protocol_callback_trampoline(
    context: *mut c_void,
    event_list: *const ffi::MIDIEventList,
    src_conn_ref_con: *mut c_void,
) {
    let _ = unsafe {
        CallbackContext::<CallbackTable>::with(
            context,
            "MidiInputPort protocol callback",
            |table| {
                let connections = table.lock();
                if let Some(connection) = connections
                    .iter()
                    .find(|connection| source_ref_con(connection.source) == src_conn_ref_con)
                {
                    (connection.callback)(event_list, connection.ref_con);
                }
            },
        )
    };
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::os::unix::process::CommandExt;
    use std::process::Command;
    use std::ptr;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::OnceLock;

    use doom_fish_utils::callback_context::CallbackContext;

    use super::{CallbackTable, InputPortKind, MidiInputPort, ProtocolConnection};
    use crate::client::MidiClient;
    use crate::endpoint::MidiEndpoint;
    use crate::error::{MidiError, MidiStatus};
    use crate::ffi;

    const REEXEC_ENV: &str = "COREMIDI_RS_TEST_REEXEC";

    fn connect_midi_server() {
        static CONNECTED: OnceLock<()> = OnceLock::new();
        CONNECTED.get_or_init(|| {
            match MidiClient::new("coremidi-rs unit test server connection") {
                Ok(client) => std::mem::forget(client),
                Err(MidiError::Status(MidiStatus::OsStatus(-304) | MidiStatus::ServerStart))
                    if std::env::var_os(REEXEC_ENV).is_none() =>
                {
                    let error = Command::new(std::env::current_exe().expect("test binary path"))
                        .args(std::env::args_os().skip(1))
                        .env(REEXEC_ENV, "1")
                        .exec();
                    panic!("re-running the test binary failed: {error}");
                }
                Err(error) => panic!("CoreMIDI client creation failed: {error}"),
            }
        });
    }

    unsafe extern "C" fn ignore(_list: *const ffi::MIDIEventList, _ref_con: *mut c_void) {}

    const fn connection(source: ffi::MIDIEndpointRef) -> ProtocolConnection {
        connection_with(source, ptr::null_mut())
    }

    const fn connection_with(
        source: ffi::MIDIEndpointRef,
        ref_con: *mut c_void,
    ) -> ProtocolConnection {
        ProtocolConnection {
            source,
            callback: ignore,
            ref_con,
        }
    }

    #[test]
    fn disconnect_source_frees_the_connection_context() {
        connect_midi_server();
        let port = MidiInputPort {
            raw: 0,
            kind: InputPortKind::Callbacks(CallbackContext::new(CallbackTable::default())),
        };
        let InputPortKind::Callbacks(table) = &port.kind else {
            unreachable!("the port was built with a callback table");
        };
        let source = unsafe { MidiEndpoint::from_raw(42) };

        for _ in 0..64 {
            assert!(table.get().replace(connection(42)).is_none());
            assert!(table.get().replace(connection(43)).is_none());
            let _ = port.disconnect_source(source);
            let remaining: Vec<_> = table
                .get()
                .lock()
                .iter()
                .map(|entry| entry.source)
                .collect();
            assert_eq!(remaining, vec![43]);
            assert!(table.get().remove(43).is_some());
        }

        assert!(unsafe {
            port.connect_source_with_protocol_callback(source, ignore, ptr::null_mut())
        }
        .is_err());
        assert!(table.get().lock().is_empty());

        let mut first = 1_u8;
        let mut second = 2_u8;
        let first_ref_con = ptr::from_mut(&mut first).cast::<c_void>();
        let second_ref_con = ptr::from_mut(&mut second).cast::<c_void>();
        assert!(table
            .get()
            .replace(connection_with(42, first_ref_con))
            .is_none());
        let failed =
            unsafe { port.connect_source_with_protocol_callback(source, ignore, second_ref_con) };
        assert!(failed.is_err());
        let restored = table.get().lock();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].ref_con, first_ref_con);
    }

    #[test]
    fn dropping_a_port_deactivates_its_callback_context() {
        connect_midi_server();
        let context = CallbackContext::new(CallbackTable::default());
        assert!(context.get().replace(connection(42)).is_none());
        let port = MidiInputPort {
            raw: 0,
            kind: InputPortKind::Callbacks(context),
        };
        let InputPortKind::Callbacks(table) = &port.kind else {
            unreachable!("the port was built with a callback table");
        };
        let observer = table.retained_ptr();

        drop(port);

        let called = AtomicBool::new(false);
        let result = unsafe {
            CallbackContext::<CallbackTable>::with(observer, "test", |table| {
                called.store(true, Ordering::SeqCst);
                table.lock().len()
            })
        };
        assert_eq!(result, None);
        assert!(!called.load(Ordering::SeqCst));
        unsafe { (CallbackContext::<CallbackTable>::RELEASE)(observer) };
    }
}
