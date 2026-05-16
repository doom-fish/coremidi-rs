use coremidi::prelude::*;

#[test]
fn endpoint_area_counts_are_iterable() {
    assert_eq!(devices().count(), device_count());
    assert_eq!(sources().count(), source_count());
    assert_eq!(destinations().count(), destination_count());
    assert_eq!(external_devices().count(), external_device_count());

    if let Some(device) = devices().next() {
        assert_eq!(device.entities().count(), device.entity_count());
    }
}
