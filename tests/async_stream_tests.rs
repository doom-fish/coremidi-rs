#![cfg(feature = "async")]

mod common;

use std::thread;
use std::time::{Duration, Instant};

use coremidi::prelude::*;
use coremidi::{
    MidiCIDiscoveryStream, MidiClientNotificationStream, MidiEventStream, MidiThruConnectionStream,
    MidiVirtualDestinationStream, OwnedEventList,
};

fn wait_for<T>(mut next: impl FnMut() -> Option<T>) -> Option<T> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(item) = next() {
            return Some(item);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(Duration::from_millis(2));
    }
}

#[test]
fn test_midi_client_notification_stream_subscribe() {
    common::connect_midi_server();
    if let Ok(stream) = MidiClientNotificationStream::subscribe("async notify subscribe", 16) {
        assert_eq!(stream.buffered_count(), 0);
    }
}

#[test]
fn test_midi_client_notification_stream_drop_closes() {
    common::connect_midi_server();
    if let Ok(stream) = MidiClientNotificationStream::subscribe("async notify drop", 16) {
        assert_eq!(stream.buffered_count(), 0);
        drop(stream);
    }
}

#[test]
fn test_owned_event_list_from_null() {
    common::connect_midi_server();
    let owned = unsafe { OwnedEventList::copy_from(std::ptr::null()) };
    assert!(owned.is_none());
}

#[test]
fn test_midi_ci_discovery_stream_subscribe() {
    common::connect_midi_server();
    if let Some(stream) = MidiCIDiscoveryStream::subscribe(16) {
        assert!(stream.try_next().is_some());
    }
    assert!(MidiCIDiscoveryStream::subscribe(0).is_none());
}

#[test]
fn test_stream_capacity_is_validated() {
    common::connect_midi_server();
    assert!(matches!(
        MidiClientNotificationStream::subscribe("async zero capacity", 0),
        Err(MidiError::InvalidArgument(_))
    ));
    assert!(matches!(
        MidiThruConnectionStream::subscribe("async zero capacity", 0),
        Err(MidiError::InvalidArgument(_))
    ));
}

#[test]
fn test_midi_event_stream_copies_event_lists_from_a_source() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("async event stream")?;
    let source =
        client.virtual_source_with_protocol("async event stream source", MidiProtocol::Midi2)?;
    let stream = MidiEventStream::subscribe(client.raw(), source.raw(), MidiProtocol::Midi2, 8)?;
    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events.add_packet_words(3, &[0x4090_3C00, 0xFFFF_0000])?;

    source.received_event_list(&events)?;

    let list = wait_for(|| stream.try_next()).expect("event list");
    assert_eq!(list.protocol, MidiProtocol::Midi2);
    assert_eq!(list.packets.len(), 1);
    let packet = list.packets[0];
    let (timestamp, word_count, words) = (packet.timeStamp, packet.wordCount, packet.words);
    assert_eq!(timestamp, 3);
    assert_eq!(word_count, 2);
    assert_eq!(&words[..2], &[0x4090_3C00, 0xFFFF_0000]);
    drop(stream);
    source.received_event_list(&events)?;
    Ok(())
}

#[test]
fn test_midi_virtual_destination_stream_copies_sent_event_lists() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("async destination stream")?;
    let stream = MidiVirtualDestinationStream::create(
        client.raw(),
        "async destination stream",
        MidiProtocol::Midi1,
        8,
    )?;
    let output = client.output_port("async destination stream output")?;
    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi1, 256);
    events.add_packet_words(4, &[0x2090_3C7F])?;

    output.send_event_list(
        unsafe { MidiEndpoint::from_raw(stream.endpoint()) },
        &events,
    )?;

    let list = wait_for(|| stream.try_next()).expect("event list");
    assert_eq!(list.protocol, MidiProtocol::Midi1);
    assert_eq!(list.packets.len(), 1);
    let words = list.packets[0].words;
    assert_eq!(words[0], 0x2090_3C7F);
    Ok(())
}

#[test]
fn test_midi_thru_connection_stream_subscribe() {
    common::connect_midi_server();
    if let Ok(stream) = MidiThruConnectionStream::subscribe("async thru subscribe", 16) {
        assert_eq!(stream.buffered_count(), 0);
    }
}

#[test]
fn test_streams_are_send_and_sync() {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MidiEventStream>();
    assert_send_sync::<MidiVirtualDestinationStream>();
    assert_send_sync::<MidiClientNotificationStream>();
    assert_send_sync::<MidiCIDiscoveryStream>();
    assert_send_sync::<MidiThruConnectionStream>();
}
