use core::ffi::c_void;
use core::fmt;
use core::future::Future;
use core::ptr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::spsc::{SpscConsumer, SpscProducer, SpscRing};

use crate::endpoint::MidiEndpoint;
use crate::error::{MidiError, MidiResult};
use crate::ffi;
use crate::packet::{MidiMessageType, MidiProtocol};

pub const MAX_EVENT_WORDS: usize = 64;
pub const MAX_RECEIVER_CAPACITY: usize = 65_536;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiEventRecord {
    timestamp: ffi::MIDITimeStamp,
    protocol: ffi::MIDIProtocolID,
    source: ffi::MIDIEndpointRef,
    word_count: u8,
    words: [u32; MAX_EVENT_WORDS],
}

impl MidiEventRecord {
    #[must_use]
    pub const fn timestamp(&self) -> ffi::MIDITimeStamp {
        self.timestamp
    }

    #[must_use]
    pub const fn protocol(&self) -> Option<MidiProtocol> {
        MidiProtocol::from_raw(self.protocol)
    }

    #[must_use]
    pub const fn source(&self) -> Option<MidiEndpoint> {
        if self.source == 0 {
            None
        } else {
            Some(unsafe { MidiEndpoint::from_raw(self.source) })
        }
    }

    #[must_use]
    pub fn words(&self) -> &[u32] {
        &self.words[..usize::from(self.word_count)]
    }

    #[must_use]
    pub fn message_type(&self) -> Option<MidiMessageType> {
        self.words()
            .first()
            .and_then(|word| MidiMessageType::from_up_word(*word))
    }

    unsafe fn read(
        packet: *const ffi::MIDIEventPacket,
        protocol: ffi::MIDIProtocolID,
        source: ffi::MIDIEndpointRef,
    ) -> Self {
        let word_count = unsafe { ptr::addr_of!((*packet).wordCount).read_unaligned() };
        let valid = usize::try_from(word_count)
            .unwrap_or(MAX_EVENT_WORDS)
            .min(MAX_EVENT_WORDS);
        let source_words = unsafe { ptr::addr_of!((*packet).words).cast::<u32>() };
        let mut words = [0_u32; MAX_EVENT_WORDS];
        for (index, word) in words[..valid].iter_mut().enumerate() {
            *word = unsafe { source_words.add(index).read_unaligned() };
        }
        Self {
            timestamp: unsafe { ptr::addr_of!((*packet).timeStamp).read_unaligned() },
            protocol,
            source,
            word_count: u8::try_from(valid).unwrap_or(u8::MAX),
            words,
        }
    }
}

impl fmt::Debug for MidiEventRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MidiEventRecord")
            .field("timestamp", &self.timestamp)
            .field("protocol", &self.protocol())
            .field("source", &self.source)
            .field("words", &self.words())
            .finish_non_exhaustive()
    }
}

pub(crate) struct EventSink {
    producer: SpscProducer<MidiEventRecord, MAX_RECEIVER_CAPACITY>,
    dropped: Arc<AtomicU64>,
}

impl EventSink {
    unsafe fn push_event_list(
        &self,
        event_list: *const ffi::MIDIEventList,
        source: ffi::MIDIEndpointRef,
    ) {
        if event_list.is_null() {
            return;
        }
        let protocol = unsafe { ptr::addr_of!((*event_list).protocol).read_unaligned() };
        let packet_count = unsafe { ptr::addr_of!((*event_list).numPackets).read_unaligned() };
        let mut packet =
            unsafe { ptr::addr_of!((*event_list).packet).cast::<ffi::MIDIEventPacket>() };
        for _ in 0..packet_count {
            let record = unsafe { MidiEventRecord::read(packet, protocol, source) };
            if self.producer.push_overwrite(record).is_some() {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
            packet = unsafe { ffi::MIDIEventPacketNext(packet) };
        }
    }
}

#[derive(Debug)]
pub struct MidiEventReceiver {
    consumer: SpscConsumer<MidiEventRecord, MAX_RECEIVER_CAPACITY>,
    dropped: Arc<AtomicU64>,
}

impl MidiEventReceiver {
    #[must_use]
    pub fn try_next(&self) -> Option<MidiEventRecord> {
        self.consumer.pop()
    }

    pub fn next(&self) -> impl Future<Output = Option<MidiEventRecord>> + '_ {
        self.consumer.pop_async()
    }

    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.consumer.buffered_count()
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.consumer.capacity()
    }

    #[must_use]
    pub fn dropped_count(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }

    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.consumer.is_closed()
    }
}

pub(crate) fn event_channel(
    capacity: usize,
) -> MidiResult<(CallbackContext<EventSink>, MidiEventReceiver)> {
    if capacity == 0 || capacity > MAX_RECEIVER_CAPACITY {
        return Err(MidiError::InvalidArgument(format!(
            "receiver capacity must be between 1 and {MAX_RECEIVER_CAPACITY} records"
        )));
    }
    let (producer, consumer) = SpscRing::with_capacity(capacity);
    let dropped = Arc::new(AtomicU64::new(0));
    let sink = EventSink {
        producer,
        dropped: Arc::clone(&dropped),
    };
    Ok((
        CallbackContext::new(sink),
        MidiEventReceiver { consumer, dropped },
    ))
}

pub(crate) unsafe extern "C" fn port_receiver_trampoline(
    context: *mut c_void,
    event_list: *const ffi::MIDIEventList,
    src_conn_ref_con: *mut c_void,
) {
    let source = u32::try_from(src_conn_ref_con as usize).unwrap_or(0);
    let _ = unsafe {
        CallbackContext::<EventSink>::with(context, "MidiEventReceiver input port", |sink| {
            sink.push_event_list(event_list, source);
        })
    };
}

pub(crate) unsafe extern "C" fn destination_receiver_trampoline(
    context: *mut c_void,
    event_list: *const ffi::MIDIEventList,
    _src_conn_ref_con: *mut c_void,
) {
    let _ = unsafe {
        CallbackContext::<EventSink>::with(
            context,
            "MidiEventReceiver virtual destination",
            |sink| {
                sink.push_event_list(event_list, 0);
            },
        )
    };
}
