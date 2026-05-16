use coremidi::driver::{driver_interface_ids, driver_io_run_loop_available};

#[test]
fn driver_area_reports_interface_ids() {
    let ids = driver_interface_ids().expect("driver interface ids available");
    assert!(!ids.driver_type_id.is_empty());
    assert!(!ids.driver_interface_id.is_empty());
    assert!(!ids.driver_interface2_id.is_empty());
    assert!(!ids.driver_interface3_id.is_empty());
    let _ = driver_io_run_loop_available();
}
