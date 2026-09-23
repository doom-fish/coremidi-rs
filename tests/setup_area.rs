#![allow(deprecated)]

mod common;

use coremidi::setup::{current_setup_xml, serial_port_drivers, serial_port_owner};
use coremidi::MidiError;

#[test]
fn setup_area_obsolete_calls_return_results() {
    common::connect_midi_server();
    match current_setup_xml() {
        Ok(bytes) => assert!(bytes.is_empty() || bytes.starts_with(b"<?xml")),
        Err(error) => assert!(matches!(error, MidiError::Status(_))),
    }
    match serial_port_drivers() {
        Ok(drivers) => assert!(drivers.iter().all(|driver| !driver.is_empty())),
        Err(error) => assert!(matches!(error, MidiError::Status(_))),
    }
    assert!(matches!(
        serial_port_owner("coremidi-rs missing serial port"),
        Ok(None) | Err(MidiError::Status(_))
    ));
}
