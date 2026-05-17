use coremidi::endpoint::UmpEndpointManager;
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
    if let Ok(constants) = UmpEndpointManager::constants() {
        println!(
            "ump_endpoint_added_notification={} function_block_key={}",
            constants.endpoint_added_notification, constants.function_block_object_key,
        );
    }
    if let Ok(endpoints) = UmpEndpointManager::endpoints() {
        println!("ump_endpoints={}", endpoints.len());
    }
}
