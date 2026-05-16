use coremidi::notification::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notification = Notification::from_json_str(
        r#"{"message_id":4,"message_size":32,"object":1,"object_type":2,"property_name":"displayName"}"#,
    )?;
    println!("{notification:?}");
    Ok(())
}
