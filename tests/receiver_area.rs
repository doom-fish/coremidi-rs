mod common;

use std::thread;
use std::time::{Duration, Instant};

use coremidi::packet::MidiMessageType;
use coremidi::prelude::*;
use coremidi::receiver::MAX_RECEIVER_CAPACITY;

fn wait_until(mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(2));
    }
    true
}

fn note_events(first_timestamp: u64, count: u32) -> EventListBuffer {
    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 4096);
    for index in 0..count {
        events
            .add_packet_words(
                first_timestamp + u64::from(index),
                &[0x4090_3C00 | index, 0xFFFF_0000],
            )
            .expect("event list has room");
    }
    events
}

#[test]
fn receiver_port_records_words_timestamp_and_source() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area port")?;
    let source =
        client.virtual_source_with_protocol("receiver area port source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let (port, receiver) =
        client.input_port_with_receiver("receiver area port input", MidiProtocol::Midi2, 16)?;
    port.connect(source.endpoint())?;

    source.received_event_list(&note_events(11, 2))?;
    assert!(wait_until(|| receiver.buffered_count() == 2));

    let first = receiver.try_next().expect("first record");
    assert_eq!(first.timestamp(), 11);
    assert_eq!(first.words(), &[0x4090_3C00, 0xFFFF_0000]);
    assert_eq!(first.protocol(), Some(MidiProtocol::Midi2));
    assert_eq!(first.source(), Some(source.endpoint()));
    assert_eq!(first.message_type(), Some(MidiMessageType::ChannelVoice2));

    let second = receiver.try_next().expect("second record");
    assert_eq!(second.timestamp(), 12);
    assert_eq!(second.words(), &[0x4090_3C01, 0xFFFF_0000]);

    assert_eq!(receiver.try_next(), None);
    assert_eq!(receiver.dropped_count(), 0);

    port.disconnect_source(source.endpoint())?;
    source.received_event_list(&note_events(20, 1))?;
    thread::sleep(Duration::from_millis(100));
    assert_eq!(receiver.try_next(), None);
    Ok(())
}

#[test]
fn receiver_keeps_the_newest_records_when_full() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area overflow")?;
    let source = client
        .virtual_source_with_protocol("receiver area overflow source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let (port, receiver) =
        client.input_port_with_receiver("receiver area overflow input", MidiProtocol::Midi2, 4)?;
    port.connect(source.endpoint())?;
    assert_eq!(receiver.capacity(), 4);

    source.received_event_list(&note_events(100, 10))?;
    assert!(wait_until(|| receiver.dropped_count() == 6));

    let kept: Vec<u64> = std::iter::from_fn(|| receiver.try_next())
        .map(|record| record.timestamp())
        .collect();
    assert_eq!(kept, vec![106, 107, 108, 109]);
    Ok(())
}

#[test]
fn receiver_next_resolves_with_a_record() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area async")?;
    let source =
        client.virtual_source_with_protocol("receiver area async source", MidiProtocol::Midi2)?;
    source.set_integer_property(MidiProperty::private(), 1)?;
    let (port, receiver) =
        client.input_port_with_receiver("receiver area async input", MidiProtocol::Midi2, 8)?;
    port.connect(source.endpoint())?;

    source.received_event_list(&note_events(7, 1))?;
    let record = pollster::block_on(receiver.next()).expect("record before close");
    assert_eq!(record.timestamp(), 7);
    Ok(())
}

#[test]
fn receiver_closes_once_the_port_is_dropped() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area port close")?;
    let (port, receiver) = client.input_port_with_receiver(
        "receiver area port close input",
        MidiProtocol::Midi1,
        8,
    )?;
    assert!(!receiver.is_closed());

    drop(port);

    assert!(wait_until(|| receiver.is_closed()));
    assert_eq!(pollster::block_on(receiver.next()), None);
    Ok(())
}

#[test]
fn receiver_destination_collects_sent_events_and_closes_on_drop() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area destination")?;
    let (destination, receiver) = client.virtual_destination_with_receiver(
        "receiver area destination",
        MidiProtocol::Midi1,
        8,
    )?;
    destination.set_integer_property(MidiProperty::private(), 1)?;
    let output = client.output_port("receiver area destination output")?;
    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi1, 256);
    events.add_packet_words(5, &[0x2090_3C7F])?;

    output.send_event_list(destination.endpoint(), &events)?;

    assert!(wait_until(|| receiver.buffered_count() == 1));
    let record = receiver.try_next().expect("record");
    assert_eq!(record.words(), &[0x2090_3C7F]);
    assert_eq!(record.protocol(), Some(MidiProtocol::Midi1));
    assert_eq!(record.source(), None);

    drop(destination);
    assert!(wait_until(|| receiver.is_closed()));
    Ok(())
}

#[test]
fn receiver_capacity_is_validated() -> MidiResult<()> {
    common::connect_midi_server();
    let client = MidiClient::new("receiver area capacity")?;
    assert!(matches!(
        client.input_port_with_receiver("receiver area zero", MidiProtocol::Midi1, 0),
        Err(MidiError::InvalidArgument(_))
    ));
    assert!(matches!(
        client.input_port_with_receiver(
            "receiver area huge",
            MidiProtocol::Midi1,
            MAX_RECEIVER_CAPACITY + 1
        ),
        Err(MidiError::InvalidArgument(_))
    ));
    assert!(matches!(
        client.virtual_destination_with_receiver("receiver area zero", MidiProtocol::Midi1, 0),
        Err(MidiError::InvalidArgument(_))
    ));
    let (_port, receiver) =
        client.input_port_with_receiver("receiver area max", MidiProtocol::Midi1, 1)?;
    assert_eq!(receiver.capacity(), 1);
    Ok(())
}

#[test]
fn receiver_types_are_send_and_sync() {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MidiEventReceiver>();
    assert_send_sync::<MidiEventRecord>();
    assert_send_sync::<MidiInputPort>();
    assert_send_sync::<VirtualDestination>();
    assert_send_sync::<PacketListBuffer>();
    assert_send_sync::<EventListBuffer>();
}
