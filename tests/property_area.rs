use coremidi::prelude::*;
use coremidi::property::MidiObjectType;

#[test]
fn property_area_exposes_constants_and_types() {
    assert_eq!(
        MidiObjectType::from_raw(MidiObjectType::Device.raw()),
        MidiObjectType::Device
    );
    assert!(MidiObjectType::ExternalSource.is_external());
    assert!(format!("{:?}", MidiProperty::name()).contains("MidiProperty"));

    if let Some(endpoint) = sources().next().or_else(|| destinations().next()) {
        let _ = endpoint.string_property(MidiProperty::display_name());
        let _ = endpoint.unique_id();
    }
}
