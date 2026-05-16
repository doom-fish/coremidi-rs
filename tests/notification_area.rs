use coremidi::notification::Notification;

#[test]
fn notification_area_decodes_json_payloads() {
    let notification = Notification::from_json_str(
        r#"{"message_id":4,"message_size":32,"object":1,"object_type":2,"property_name":"displayName"}"#,
    )
    .expect("notification must decode");
    match notification {
        Notification::PropertyChanged {
            object,
            property_name,
            ..
        } => {
            assert_eq!(object, 1);
            assert_eq!(property_name, "displayName");
        }
        other => panic!("unexpected notification: {other:?}"),
    }
}
