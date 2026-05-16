use std::error::Error;
use std::ffi::c_void;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use coremidi::ffi;
use coremidi::packet::PacketListRef;
use coremidi::prelude::*;

#[derive(Clone, Copy)]
struct Route {
    output_port: ffi::MIDIPortRef,
    destination: ffi::MIDIEndpointRef,
}

static ROUTE: Mutex<Option<Route>> = Mutex::new(None);
static RECEIVED_PACKET: Mutex<Option<Vec<u8>>> = Mutex::new(None);

unsafe extern "C" fn source_input_callback(
    packet_list: *const ffi::MIDIPacketList,
    _read_proc_ref_con: *mut c_void,
    _src_conn_ref_con: *mut c_void,
) {
    let route = *ROUTE.lock().expect("route mutex poisoned");
    if let Some(route) = route {
        let _ = ffi::MIDISend(route.output_port, route.destination, packet_list);
    }
}

unsafe extern "C" fn destination_callback(
    packet_list: *const ffi::MIDIPacketList,
    _read_proc_ref_con: *mut c_void,
    _src_conn_ref_con: *mut c_void,
) {
    let packet_list = PacketListRef::from_ptr(packet_list);
    if let Some(packet) = packet_list.iter().next() {
        *RECEIVED_PACKET.lock().expect("received mutex poisoned") = Some(packet.bytes().to_vec());
    }
}

extern "C" {
    fn mach_absolute_time() -> u64;
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = MidiClient::new("coremidi smoke client")?;
    let source = client.virtual_source("coremidi smoke source")?;
    let destination = unsafe {
        client.virtual_destination(
            "coremidi smoke destination",
            Some(destination_callback),
            std::ptr::null_mut(),
        )?
    };
    let output = client.output_port("coremidi smoke output")?;
    let input = unsafe {
        client.input_port(
            "coremidi smoke input",
            Some(source_input_callback),
            std::ptr::null_mut(),
        )?
    };

    *ROUTE.lock().expect("route mutex poisoned") = Some(Route {
        output_port: output.raw(),
        destination: destination.raw(),
    });

    unsafe {
        input.connect_source(source.endpoint(), std::ptr::null_mut())?;
    }

    let now = unsafe { mach_absolute_time() };
    let mut packets = PacketListBuffer::with_capacity(1024);
    packets.add_packet(now, &[0x90, 60, 100])?;

    source.received(&packets)?;

    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let maybe_bytes = RECEIVED_PACKET
            .lock()
            .expect("received mutex poisoned")
            .clone();
        if let Some(bytes) = maybe_bytes {
            if bytes == [0x90, 60, 100] {
                println!("✅ coremidi loopback OK");
                return Ok(());
            }
            return Err(format!("unexpected packet bytes: {bytes:?}").into());
        }

        if Instant::now() >= deadline {
            return Err("timed out waiting for virtual destination callback".into());
        }

        thread::sleep(Duration::from_millis(10));
    }
}
