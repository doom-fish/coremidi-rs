mod common;

use coremidi::setup::{current_setup_xml, serial_port_drivers};

#[test]
fn setup_area_read_only_calls_do_not_panic() {
    common::connect_midi_server();
    let _ = current_setup_xml();
    let _ = serial_port_drivers();
}
