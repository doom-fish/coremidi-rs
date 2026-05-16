use coremidi::prelude::*;

fn main() {
    println!(
        "devices={} sources={} destinations={} external_devices={}",
        device_count(),
        source_count(),
        destination_count(),
        external_device_count(),
    );

    if let Some(source) = sources().next() {
        println!("first source name={:?}", source.name().ok());
    }
    if let Some(device) = devices().next() {
        println!("first device entity_count={}", device.entity_count());
    }
}
