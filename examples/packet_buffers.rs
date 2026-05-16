use coremidi::packet::{
    MidiMessage128, MidiMessage64, MidiNoteAttribute, MidiProgramChangeOptions,
    UmpStreamMessageStatus,
};
use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut packets = PacketListBuffer::with_capacity(256);
    packets.add_packet(1, &[0x90, 60, 100])?;
    packets.add_packet(2, &[0x80, 60, 0])?;

    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events.add_packet_words(3, &[0x4123_4567, 0x89AB_CDEF])?;
    let event = events.as_event_list().iter().next().expect("event present");

    let message64 = MidiMessage64::new(0x4123_4567, 0x89AB_CDEF);
    let message128 = MidiMessage128::new(0xF123_4567, 0x89AB_CDEF, 0x0123_4567, 0x89AB_CDEF);
    println!(
        "packet_count={} event_count={} protocol={:?} event_type={:?} helper64={:?} helper128={:?} note_attr={:#x} bank_valid={:#x} stream_status={:#x}",
        packets.as_packet_list().packet_count(),
        events.as_event_list().packet_count(),
        events.as_event_list().protocol(),
        event.message_type(),
        message64.message_type(),
        message128.message_type(),
        MidiNoteAttribute::Pitch.as_raw(),
        MidiProgramChangeOptions::BANK_VALID.bits(),
        UmpStreamMessageStatus::FunctionBlockDiscovery.as_raw(),
    );
    Ok(())
}
