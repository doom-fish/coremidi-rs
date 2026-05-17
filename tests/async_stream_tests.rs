#![cfg(feature = "async")]

use coremidi::{
    MidiCIDiscoveryStream, MidiClientNotificationStream, MidiThruConnectionStream, OwnedEventList,
};

#[test]
fn test_midi_client_notification_stream_subscribe() {
    if let Ok(stream) = MidiClientNotificationStream::subscribe("async notify subscribe", 16) {
        assert_eq!(stream.buffered_count(), 0);
    }
}

#[test]
fn test_midi_client_notification_stream_drop_closes() {
    if let Ok(stream) = MidiClientNotificationStream::subscribe("async notify drop", 16) {
        assert_eq!(stream.buffered_count(), 0);
        drop(stream);
    }
}

#[test]
fn test_owned_event_list_from_null() {
    let owned = unsafe { OwnedEventList::copy_from(std::ptr::null()) };
    assert!(owned.is_none());
}

#[test]
fn test_midi_ci_discovery_stream_subscribe() {
    if let Some(stream) = MidiCIDiscoveryStream::subscribe(16) {
        assert!(stream.buffered_count() <= 16);
    }
}

#[test]
fn test_midi_thru_connection_stream_subscribe() {
    if let Ok(stream) = MidiThruConnectionStream::subscribe("async thru subscribe", 16) {
        assert_eq!(stream.buffered_count(), 0);
    }
}
