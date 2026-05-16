use coremidi::setup::{current_setup_xml, serial_port_drivers};

fn main() {
    match current_setup_xml() {
        Ok(bytes) => println!("current_setup_xml_bytes={}", bytes.len()),
        Err(error) => println!("current_setup_xml unavailable: {error}"),
    }
    match serial_port_drivers() {
        Ok(drivers) => println!("serial_port_drivers={}", drivers.len()),
        Err(error) => println!("serial_port_drivers unavailable: {error}"),
    }
}
