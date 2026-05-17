use coremidi::thru_connection::{MidiTransform, MidiTransformKind, ThruConnectionParams};

#[test]
fn thru_connection_area_roundtrips_params() {
    let params = ThruConnectionParams::default();
    let bytes = params.to_bytes().expect("serialize thru params");
    let decoded = ThruConnectionParams::from_bytes(&bytes).expect("deserialize thru params");
    assert_eq!(params, decoded);

    let transform = MidiTransform {
        kind: MidiTransformKind::Add,
        param: 12,
    };
    assert_eq!(
        MidiTransform::from_raw(transform.into_raw()).expect("decode raw transform"),
        transform
    );
}
