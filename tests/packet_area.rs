use coremidi::packet::{
    MidiCvStatus, MidiMessage128, MidiMessage64, MidiMessage96, MidiMessageType, MidiNoteAttribute,
    MidiPerNoteManagementOptions, MidiProgramChangeOptions, MidiSysExStatus, MidiSystemStatus,
    MidiUtilityStatus, UmpStreamMessageFormat, UmpStreamMessageStatus,
};
use coremidi::prelude::*;

#[test]
fn packet_and_event_buffers_roundtrip() -> MidiResult<()> {
    let mut packets = PacketListBuffer::with_capacity(256);
    packets.add_packet(10, &[0x90, 60, 100])?;
    let packet = packets
        .as_packet_list()
        .iter()
        .next()
        .expect("packet present");
    assert_eq!(packet.timestamp(), 10);
    assert_eq!(packet.bytes(), &[0x90, 60, 100]);

    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events.add_packet_words(20, &[0x4123_4567, 0x89AB_CDEF])?;
    let event = events.as_event_list().iter().next().expect("event present");
    assert_eq!(event.timestamp(), 20);
    assert_eq!(event.words(), &[0x4123_4567, 0x89AB_CDEF]);
    assert_eq!(event.message_type(), Some(MidiMessageType::ChannelVoice2));
    assert_eq!(events.as_event_list().protocol(), Some(MidiProtocol::Midi2));

    assert_eq!(
        MidiMessageType::from_up_word(0xF123_4567),
        Some(MidiMessageType::Stream)
    );
    assert_eq!(MidiCvStatus::from_raw(0x9), Some(MidiCvStatus::NoteOn));
    assert_eq!(
        MidiSystemStatus::from_raw(0xFE),
        Some(MidiSystemStatus::ActiveSensing)
    );
    assert_eq!(
        MidiSysExStatus::from_raw(0x9),
        Some(MidiSysExStatus::MixedDataSetPayload)
    );
    assert_eq!(
        MidiUtilityStatus::from_raw(0x4),
        Some(MidiUtilityStatus::TicksSinceLastEvent)
    );
    assert_eq!(
        UmpStreamMessageFormat::from_raw(0x02),
        Some(UmpStreamMessageFormat::Continuing)
    );
    assert_eq!(
        UmpStreamMessageStatus::from_raw(0x10),
        Some(UmpStreamMessageStatus::FunctionBlockDiscovery)
    );
    assert_eq!(
        MidiNoteAttribute::from_raw(0x3),
        Some(MidiNoteAttribute::Pitch)
    );
    assert_eq!(MidiProgramChangeOptions::BANK_VALID.bits(), 0x1);
    assert_eq!(
        (MidiPerNoteManagementOptions::RESET | MidiPerNoteManagementOptions::DETACH).bits(),
        0x3
    );

    let message64 = MidiMessage64::new(0x4123_4567, 0x89AB_CDEF);
    let message96 = MidiMessage96::from([0xD123_4567, 0x89AB_CDEF, 0x0123_4567]);
    let message128 = MidiMessage128::new(0xF123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF);
    assert_eq!(message64.words(), [0x4123_4567, 0x89AB_CDEF]);
    assert_eq!(
        message64.message_type(),
        Some(MidiMessageType::ChannelVoice2)
    );
    assert_eq!(message96.words(), [0xD123_4567, 0x89AB_CDEF, 0x0123_4567]);
    assert_eq!(message96.message_type(), Some(MidiMessageType::FlexData));
    assert_eq!(
        message128.words(),
        [0xF123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF]
    );
    assert_eq!(message128.message_type(), Some(MidiMessageType::Stream));
    Ok(())
}

fn packet_contents(buffer: &PacketListBuffer) -> Vec<(u64, Vec<u8>)> {
    buffer
        .as_packet_list()
        .iter()
        .map(|packet| (packet.timestamp(), packet.bytes().to_vec()))
        .collect()
}

fn event_contents(buffer: &EventListBuffer) -> Vec<(u64, Vec<u32>)> {
    buffer
        .as_event_list()
        .iter()
        .map(|packet| (packet.timestamp(), packet.words().to_vec()))
        .collect()
}

#[test]
fn cloned_packet_list_buffer_keeps_writing_after_the_original_is_dropped() -> MidiResult<()> {
    let mut original = PacketListBuffer::with_capacity(256);
    original.add_packet(1, &[0x90, 60, 100])?;
    let mut clone = original.clone();
    drop(original);

    clone.add_packet(2, &[0x80, 60, 0])?;
    clone.add_packet(3, &[0xB0, 7, 64])?;

    assert_eq!(
        packet_contents(&clone),
        vec![
            (1, vec![0x90, 60, 100]),
            (2, vec![0x80, 60, 0]),
            (3, vec![0xB0, 7, 64]),
        ]
    );
    Ok(())
}

#[test]
fn packet_list_buffer_clones_write_independently() -> MidiResult<()> {
    let mut original = PacketListBuffer::with_capacity(256);
    original.add_packet(1, &[0x90, 60, 100])?;
    let mut clone = original.clone();

    clone.add_packet(2, &[0x80, 60, 0])?;
    original.add_packet(3, &[0xB0, 7, 64])?;

    assert_eq!(
        packet_contents(&original),
        vec![(1, vec![0x90, 60, 100]), (3, vec![0xB0, 7, 64])]
    );
    assert_eq!(
        packet_contents(&clone),
        vec![(1, vec![0x90, 60, 100]), (2, vec![0x80, 60, 0])]
    );

    clone.clear();
    clone.add_packet(4, &[0xC0, 5])?;
    assert_eq!(packet_contents(&clone), vec![(4, vec![0xC0, 5])]);
    assert_eq!(original.as_packet_list().packet_count(), 2);
    Ok(())
}

#[test]
fn cloned_event_list_buffer_keeps_writing_after_the_original_is_dropped() -> MidiResult<()> {
    let mut original = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    original.add_packet_words(1, &[0x4090_3C00, 0xFFFF_0000])?;
    let mut clone = original.clone();
    drop(original);

    clone.add_packet_words(2, &[0x4080_3C00, 0x0000_0000])?;
    clone.add_packet_words(3, &[0x40B0_0700, 0x8000_0000])?;

    assert_eq!(
        event_contents(&clone),
        vec![
            (1, vec![0x4090_3C00, 0xFFFF_0000]),
            (2, vec![0x4080_3C00, 0x0000_0000]),
            (3, vec![0x40B0_0700, 0x8000_0000]),
        ]
    );
    assert_eq!(clone.as_event_list().protocol(), Some(MidiProtocol::Midi2));
    Ok(())
}

#[test]
fn event_list_buffer_clones_write_independently() -> MidiResult<()> {
    let mut original = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    original.add_packet_words(1, &[0x4090_3C00, 0xFFFF_0000])?;
    let mut clone = original.clone();

    clone.add_packet_words(2, &[0x4080_3C00, 0x0000_0000])?;
    original.add_packet_words(3, &[0x40B0_0700, 0x8000_0000])?;

    assert_eq!(
        event_contents(&original),
        vec![
            (1, vec![0x4090_3C00, 0xFFFF_0000]),
            (3, vec![0x40B0_0700, 0x8000_0000]),
        ]
    );
    assert_eq!(
        event_contents(&clone),
        vec![
            (1, vec![0x4090_3C00, 0xFFFF_0000]),
            (2, vec![0x4080_3C00, 0x0000_0000]),
        ]
    );
    Ok(())
}

#[test]
fn packet_list_buffer_reports_a_full_buffer() {
    let mut buffer = PacketListBuffer::with_capacity(0);
    let capacity = buffer.capacity_bytes();
    let payload = vec![0xF0_u8; capacity];
    assert_eq!(
        buffer.add_packet(1, &payload),
        Err(MidiError::BufferTooSmall {
            requested: capacity,
            available: capacity,
        })
    );
    assert_eq!(buffer.as_packet_list().packet_count(), 0);
}
