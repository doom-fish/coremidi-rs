mod common;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use coremidi::prelude::*;

#[test]
fn client_area_smoke() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("client area smoke")?;
    let output = client.output_port("client area output")?;
    assert_ne!(client.raw(), 0);
    assert_ne!(output.raw(), 0);
    Ok(())
}

#[test]
fn notification_handler_is_freed_with_the_client() -> MidiResult<()> {
    common::connect_midi_server();
    let hits = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&hits);
    let client = MidiClient::with_notification_handler("client area handler", move |_| {
        captured.fetch_add(1, Ordering::SeqCst);
    })?;
    assert_ne!(client.raw(), 0);
    assert_eq!(Arc::strong_count(&hits), 2);

    drop(client);

    let deadline = Instant::now() + Duration::from_secs(10);
    while Arc::strong_count(&hits) != 1 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(Arc::strong_count(&hits), 1);
    Ok(())
}

#[test]
fn notification_handler_is_freed_when_creation_fails() {
    common::connect_midi_server();
    let hits = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&hits);
    let result = MidiClient::with_notification_handler("client area\0handler", move |_| {
        captured.fetch_add(1, Ordering::SeqCst);
    });
    assert!(matches!(result, Err(MidiError::InvalidArgument(_))));
    assert_eq!(Arc::strong_count(&hits), 1);
}
