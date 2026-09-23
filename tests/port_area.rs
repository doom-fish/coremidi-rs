mod common;

use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use coremidi::ffi::MIDIEventList;
use coremidi::prelude::*;

#[test]
fn port_area_create_ports() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("port area smoke")?;
    let output = client.output_port("port area output")?;
    let input = client.input_port_with_protocol("port area input", MidiProtocol::Midi1)?;
    assert_ne!(output.raw(), 0);
    assert_ne!(input.raw(), 0);
    Ok(())
}

#[derive(Default)]
struct Probe {
    calls: AtomicUsize,
    entered: AtomicBool,
    exited: AtomicBool,
}

impl Probe {
    const fn ref_con(&self) -> *mut c_void {
        ptr::from_ref(self).cast_mut().cast()
    }
}

unsafe extern "C" fn count_call(_list: *const MIDIEventList, ref_con: *mut c_void) {
    let probe = unsafe { &*ref_con.cast::<Probe>() };
    probe.calls.fetch_add(1, Ordering::SeqCst);
}

unsafe extern "C" fn count_call_by_ten(_list: *const MIDIEventList, ref_con: *mut c_void) {
    let probe = unsafe { &*ref_con.cast::<Probe>() };
    probe.calls.fetch_add(10, Ordering::SeqCst);
}

unsafe extern "C" fn slow_call(_list: *const MIDIEventList, ref_con: *mut c_void) {
    let probe = unsafe { &*ref_con.cast::<Probe>() };
    probe.calls.fetch_add(1, Ordering::SeqCst);
    probe.entered.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(200));
    probe.exited.store(true, Ordering::SeqCst);
}

fn wait_until(mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(2));
    }
    true
}

fn one_event(timestamp: u64) -> EventListBuffer {
    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events
        .add_packet_words(timestamp, &[0x4090_3C00, 0xFFFF_0000])
        .expect("event list has room");
    events
}

#[test]
fn disconnect_source_waits_for_a_running_callback() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("port area disconnect drain")?;
    let source =
        client.virtual_source_with_protocol("port area disconnect source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let port =
        client.input_port_with_protocol("port area disconnect input", MidiProtocol::Midi2)?;
    let probe = Box::new(Probe::default());
    unsafe {
        port.connect_source_with_protocol_callback(source.endpoint(), slow_call, probe.ref_con())?;
    }

    source.received_event_list(&one_event(1))?;
    assert!(wait_until(|| probe.entered.load(Ordering::SeqCst)));
    port.disconnect_source(source.endpoint())?;
    assert!(probe.exited.load(Ordering::SeqCst));

    let calls = probe.calls.load(Ordering::SeqCst);
    source.received_event_list(&one_event(2))?;
    thread::sleep(Duration::from_millis(100));
    assert_eq!(probe.calls.load(Ordering::SeqCst), calls);
    assert_eq!(
        port.disconnect_source(source.endpoint()),
        Err(MidiError::Status(MidiStatus::NoConnection))
    );
    drop(probe);
    Ok(())
}

#[test]
fn dropping_a_protocol_port_waits_for_a_running_callback() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("port area drop drain")?;
    let source =
        client.virtual_source_with_protocol("port area drop source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let port = client.input_port_with_protocol("port area drop input", MidiProtocol::Midi2)?;
    let probe = Box::new(Probe::default());
    unsafe {
        port.connect_source_with_protocol_callback(source.endpoint(), slow_call, probe.ref_con())?;
    }

    source.received_event_list(&one_event(1))?;
    assert!(wait_until(|| probe.entered.load(Ordering::SeqCst)));
    drop(port);
    assert!(probe.exited.load(Ordering::SeqCst));
    drop(probe);
    Ok(())
}

#[test]
fn reconnecting_a_source_replaces_its_callback() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("port area reconnect")?;
    let source =
        client.virtual_source_with_protocol("port area reconnect source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let port = client.input_port_with_protocol("port area reconnect input", MidiProtocol::Midi2)?;
    let probe = Box::new(Probe::default());
    unsafe {
        port.connect_source_with_protocol_callback(source.endpoint(), count_call, probe.ref_con())?;
        port.connect_source_with_protocol_callback(
            source.endpoint(),
            count_call_by_ten,
            probe.ref_con(),
        )?;
    }

    source.received_event_list(&one_event(1))?;
    assert!(wait_until(|| probe.calls.load(Ordering::SeqCst) > 0));
    thread::sleep(Duration::from_millis(50));
    assert_eq!(probe.calls.load(Ordering::SeqCst), 10);

    port.disconnect_source(source.endpoint())?;
    drop(probe);
    Ok(())
}

#[test]
fn input_ports_reject_connections_meant_for_another_kind() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("port area kinds")?;
    let source = client.virtual_source("port area kinds source")?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let protocol_port =
        client.input_port_with_protocol("port area kinds protocol", MidiProtocol::Midi1)?;
    let (receiver_port, _receiver) =
        client.input_port_with_receiver("port area kinds receiver", MidiProtocol::Midi1, 4)?;

    assert!(matches!(
        unsafe { protocol_port.connect_source(source.endpoint(), ptr::null_mut()) },
        Err(MidiError::Unsupported(_))
    ));
    assert!(matches!(
        protocol_port.connect(source.endpoint()),
        Err(MidiError::Unsupported(_))
    ));
    assert!(matches!(
        unsafe {
            receiver_port.connect_source_with_protocol_callback(
                source.endpoint(),
                count_call,
                ptr::null_mut(),
            )
        },
        Err(MidiError::Unsupported(_))
    ));
    assert!(matches!(
        unsafe { receiver_port.connect_source(source.endpoint(), ptr::null_mut()) },
        Err(MidiError::Unsupported(_))
    ));
    Ok(())
}
