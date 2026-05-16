use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr;
use std::slice;

use crate::error::{MidiError, MidiResult};
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MidiProtocol {
    Midi1 = ffi::kMIDIProtocol_1_0,
    Midi2 = ffi::kMIDIProtocol_2_0,
}

impl MidiProtocol {
    #[must_use]
    pub const fn as_raw(self) -> ffi::MIDIProtocolID {
        self as ffi::MIDIProtocolID
    }

    #[must_use]
    pub const fn from_raw(raw: ffi::MIDIProtocolID) -> Option<Self> {
        match raw {
            ffi::kMIDIProtocol_1_0 => Some(Self::Midi1),
            ffi::kMIDIProtocol_2_0 => Some(Self::Midi2),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketListBuffer {
    storage: Vec<u64>,
    current_packet: *mut ffi::MIDIPacket,
}

impl PacketListBuffer {
    #[must_use]
    pub fn with_capacity(capacity_bytes: usize) -> Self {
        let capacity_bytes = capacity_bytes.max(size_of::<ffi::MIDIPacketList>());
        let words = capacity_bytes.div_ceil(size_of::<u64>()).max(1);
        let mut storage = vec![0_u64; words];
        let current_packet = unsafe { ffi::MIDIPacketListInit(storage.as_mut_ptr().cast()) };
        Self {
            storage,
            current_packet,
        }
    }

    pub fn clear(&mut self) {
        self.current_packet = unsafe { ffi::MIDIPacketListInit(self.storage.as_mut_ptr().cast()) };
    }

    #[must_use]
    pub fn capacity_bytes(&self) -> usize {
        self.storage.len() * size_of::<u64>()
    }

    pub fn add_packet(&mut self, timestamp: ffi::MIDITimeStamp, data: &[u8]) -> MidiResult<()> {
        if data.len() > usize::from(u16::MAX) {
            return Err(MidiError::InvalidArgument(
                "MIDIPacket payloads larger than 65535 bytes must be split across packets".into(),
            ));
        }

        let packet = unsafe {
            ffi::MIDIPacketListAdd(
                self.storage.as_mut_ptr().cast(),
                self.capacity_bytes(),
                self.current_packet,
                timestamp,
                data.len(),
                data.as_ptr(),
            )
        };
        if packet.is_null() {
            Err(MidiError::BufferTooSmall {
                requested: data.len(),
                available: self.capacity_bytes(),
            })
        } else {
            self.current_packet = packet;
            Ok(())
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const ffi::MIDIPacketList {
        self.storage.as_ptr().cast()
    }

    #[must_use]
    pub fn as_packet_list(&self) -> PacketListRef<'_> {
        unsafe { PacketListRef::from_ptr(self.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PacketListRef<'a> {
    ptr: *const ffi::MIDIPacketList,
    _marker: PhantomData<&'a ffi::MIDIPacketList>,
}

impl<'a> PacketListRef<'a> {
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIPacketList) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn packet_count(self) -> u32 {
        unsafe { ptr::addr_of!((*self.ptr).numPackets).read_unaligned() }
    }

    #[must_use]
    pub fn iter(self) -> PacketIter<'a> {
        PacketIter {
            next_packet: unsafe { ptr::addr_of!((*self.ptr).packet).cast() },
            remaining: self.packet_count(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_ptr(self) -> *const ffi::MIDIPacketList {
        self.ptr
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MidiPacketRef<'a> {
    ptr: *const ffi::MIDIPacket,
    _marker: PhantomData<&'a ffi::MIDIPacket>,
}

impl<'a> MidiPacketRef<'a> {
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIPacket) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn timestamp(self) -> ffi::MIDITimeStamp {
        unsafe { ptr::addr_of!((*self.ptr).timeStamp).read_unaligned() }
    }

    #[must_use]
    pub fn len(self) -> usize {
        usize::from(unsafe { ptr::addr_of!((*self.ptr).length).read_unaligned() })
    }

    #[must_use]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn bytes(self) -> &'a [u8] {
        let len = self.len();
        let data_ptr = unsafe { ptr::addr_of!((*self.ptr).data).cast::<u8>() };
        unsafe { slice::from_raw_parts(data_ptr, len) }
    }
}

#[derive(Debug, Clone)]
pub struct PacketIter<'a> {
    next_packet: *const ffi::MIDIPacket,
    remaining: u32,
    _marker: PhantomData<&'a ffi::MIDIPacket>,
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = MidiPacketRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.next_packet.is_null() {
            return None;
        }

        let current = unsafe { MidiPacketRef::from_ptr(self.next_packet) };
        self.next_packet = unsafe { midi_packet_next(self.next_packet) };
        self.remaining -= 1;
        Some(current)
    }
}

#[derive(Debug, Clone)]
pub struct EventListBuffer {
    storage: Vec<u64>,
    current_packet: *mut ffi::MIDIEventPacket,
    protocol: MidiProtocol,
}

impl EventListBuffer {
    #[must_use]
    pub fn with_capacity(protocol: MidiProtocol, capacity_bytes: usize) -> Self {
        let capacity_bytes = capacity_bytes.max(size_of::<ffi::MIDIEventList>());
        let words = capacity_bytes.div_ceil(size_of::<u64>()).max(1);
        let mut storage = vec![0_u64; words];
        let current_packet =
            unsafe { ffi::MIDIEventListInit(storage.as_mut_ptr().cast(), protocol.as_raw()) };
        Self {
            storage,
            current_packet,
            protocol,
        }
    }

    pub fn clear(&mut self) {
        self.current_packet = unsafe {
            ffi::MIDIEventListInit(self.storage.as_mut_ptr().cast(), self.protocol.as_raw())
        };
    }

    #[must_use]
    pub const fn protocol(&self) -> MidiProtocol {
        self.protocol
    }

    #[must_use]
    pub fn capacity_bytes(&self) -> usize {
        self.storage.len() * size_of::<u64>()
    }

    pub fn add_packet_words(
        &mut self,
        timestamp: ffi::MIDITimeStamp,
        words: &[u32],
    ) -> MidiResult<()> {
        let packet = unsafe {
            ffi::MIDIEventListAdd(
                self.storage.as_mut_ptr().cast(),
                self.capacity_bytes(),
                self.current_packet,
                timestamp,
                words.len(),
                words.as_ptr(),
            )
        };
        if packet.is_null() {
            Err(MidiError::BufferTooSmall {
                requested: core::mem::size_of_val(words),
                available: self.capacity_bytes(),
            })
        } else {
            self.current_packet = packet;
            Ok(())
        }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const ffi::MIDIEventList {
        self.storage.as_ptr().cast()
    }

    #[must_use]
    pub fn as_event_list(&self) -> EventListRef<'_> {
        unsafe { EventListRef::from_ptr(self.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventListRef<'a> {
    ptr: *const ffi::MIDIEventList,
    _marker: PhantomData<&'a ffi::MIDIEventList>,
}

impl<'a> EventListRef<'a> {
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIEventList) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn packet_count(self) -> u32 {
        unsafe { ptr::addr_of!((*self.ptr).numPackets).read_unaligned() }
    }

    #[must_use]
    pub fn protocol(self) -> Option<MidiProtocol> {
        let raw = unsafe { ptr::addr_of!((*self.ptr).protocol).read_unaligned() };
        MidiProtocol::from_raw(raw)
    }

    #[must_use]
    pub fn iter(self) -> EventIter<'a> {
        EventIter {
            next_packet: unsafe { ptr::addr_of!((*self.ptr).packet).cast() },
            remaining: self.packet_count(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_ptr(self) -> *const ffi::MIDIEventList {
        self.ptr
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MidiEventPacketRef<'a> {
    ptr: *const ffi::MIDIEventPacket,
    _marker: PhantomData<&'a ffi::MIDIEventPacket>,
}

impl<'a> MidiEventPacketRef<'a> {
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIEventPacket) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub fn timestamp(self) -> ffi::MIDITimeStamp {
        unsafe { ptr::addr_of!((*self.ptr).timeStamp).read_unaligned() }
    }

    #[must_use]
    pub fn word_count(self) -> usize {
        usize::try_from(unsafe { ptr::addr_of!((*self.ptr).wordCount).read_unaligned() })
            .unwrap_or(0)
    }

    #[must_use]
    pub fn words(self) -> &'a [u32] {
        let count = self.word_count();
        let words_ptr = unsafe { ptr::addr_of!((*self.ptr).words).cast::<u32>() };
        unsafe { slice::from_raw_parts(words_ptr, count) }
    }
}

#[derive(Debug, Clone)]
pub struct EventIter<'a> {
    next_packet: *const ffi::MIDIEventPacket,
    remaining: u32,
    _marker: PhantomData<&'a ffi::MIDIEventPacket>,
}

impl<'a> Iterator for EventIter<'a> {
    type Item = MidiEventPacketRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 || self.next_packet.is_null() {
            return None;
        }

        let current = unsafe { MidiEventPacketRef::from_ptr(self.next_packet) };
        self.next_packet = unsafe { midi_event_packet_next(self.next_packet) };
        self.remaining -= 1;
        Some(current)
    }
}

unsafe fn midi_packet_next(packet: *const ffi::MIDIPacket) -> *const ffi::MIDIPacket {
    let len = usize::from(ptr::addr_of!((*packet).length).read_unaligned());
    let data_end = ptr::addr_of!((*packet).data).cast::<u8>().add(len) as usize;

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let next = (data_end + 3) & !3;
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let next = data_end;

    next as *const ffi::MIDIPacket
}

unsafe fn midi_event_packet_next(
    packet: *const ffi::MIDIEventPacket,
) -> *const ffi::MIDIEventPacket {
    let word_count =
        usize::try_from(ptr::addr_of!((*packet).wordCount).read_unaligned()).unwrap_or(0);
    ptr::addr_of!((*packet).words)
        .cast::<u32>()
        .add(word_count)
        .cast()
}
