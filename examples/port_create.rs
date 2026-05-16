#![allow(clippy::missing_const_for_fn)]

use std::ffi::c_void;

use coremidi::prelude::*;

unsafe extern "C" fn noop_read(
    _packet_list: *const coremidi::ffi::MIDIPacketList,
    _read_proc_ref_con: *mut c_void,
    _src_conn_ref_con: *mut c_void,
) {
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MidiClient::new("coremidi port create")?;
    let output = client.output_port("coremidi port create output")?;
    let input = unsafe {
        client.input_port(
            "coremidi port create input",
            Some(noop_read),
            std::ptr::null_mut(),
        )?
    };
    println!("input_port={} output_port={}", input.raw(), output.raw());
    Ok(())
}
