mod common;

use coremidi::driver::{driver_interface_ids, driver_io_run_loop_available};

#[test]
fn driver_area_reports_interface_ids() {
    common::connect_midi_server();
    let ids = driver_interface_ids().expect("driver interface ids available");
    assert_ne!(ids.driver_type_id, "");
    assert_ne!(ids.driver_interface_id, "");
    assert_ne!(ids.driver_interface2_id, "");
    assert_ne!(ids.driver_interface3_id, "");
    let _ = driver_io_run_loop_available();
}
