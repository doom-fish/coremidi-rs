use coremidi::driver::{driver_interface_ids, driver_io_run_loop_available};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ids = driver_interface_ids()?;
    println!(
        "driver_type_id={} interface3_id={} io_run_loop_available={}",
        ids.driver_type_id,
        ids.driver_interface3_id,
        driver_io_run_loop_available(),
    );
    Ok(())
}
