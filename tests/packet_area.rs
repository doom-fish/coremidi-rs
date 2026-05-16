use coremidi::prelude::*;

#[test]
fn packet_and_event_buffers_roundtrip() -> MidiResult<()> {
    let mut packets = PacketListBuffer::with_capacity(256);
    packets.add_packet(10, &[0x90, 60, 100])?;
    let packet = packets.as_packet_list().iter().next().expect("packet present");
    assert_eq!(packet.timestamp(), 10);
    assert_eq!(packet.bytes(), &[0x90, 60, 100]);

    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events.add_packet_words(20, &[0x4123_4567, 0x89AB_CDEF])?;
    let event = events.as_event_list().iter().next().expect("event present");
    assert_eq!(event.timestamp(), 20);
    assert_eq!(event.words(), &[0x4123_4567, 0x89AB_CDEF]);
    assert_eq!(events.as_event_list().protocol(), Some(MidiProtocol::Midi2));
    Ok(())
}
