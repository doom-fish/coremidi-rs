use coremidi::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut packets = PacketListBuffer::with_capacity(256);
    packets.add_packet(1, &[0x90, 60, 100])?;
    packets.add_packet(2, &[0x80, 60, 0])?;

    let mut events = EventListBuffer::with_capacity(MidiProtocol::Midi2, 256);
    events.add_packet_words(3, &[0x4123_4567, 0x89AB_CDEF])?;

    println!(
        "packet_count={} event_count={} protocol={:?}",
        packets.as_packet_list().packet_count(),
        events.as_event_list().packet_count(),
        events.as_event_list().protocol(),
    );
    Ok(())
}
