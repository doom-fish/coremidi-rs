use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use core::ptr;
use std::slice;

use crate::error::{MidiError, MidiResult};
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
/// Wraps `MIDIProtocolID` values.
pub enum MidiProtocol {
    /// Wraps `kMIDIProtocol_1_0`.
    Midi1 = ffi::kMIDIProtocol_1_0,
    /// Wraps `kMIDIProtocol_2_0`.
    Midi2 = ffi::kMIDIProtocol_2_0,
}

impl MidiProtocol {
    #[must_use]
    /// Returns the wrapped `MIDIProtocolID`.
    pub const fn as_raw(self) -> ffi::MIDIProtocolID {
        self as ffi::MIDIProtocolID
    }

    #[must_use]
    /// Wraps an existing `MIDIProtocolID`.
    pub const fn from_raw(raw: ffi::MIDIProtocolID) -> Option<Self> {
        match raw {
            ffi::kMIDIProtocol_1_0 => Some(Self::Midi1),
            ffi::kMIDIProtocol_2_0 => Some(Self::Midi2),
            _ => None,
        }
    }
}

macro_rules! midi_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident : $repr:ty { $($variant:ident = $value:expr),+ $(,)? }) => {
        $(#[$meta])*
        /// Wraps matching CoreMIDI enum values.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr($repr)]
        $vis enum $name {
            $(#[doc = "Wraps the matching CoreMIDI case."] $variant = $value,)+
        }

        impl $name {
            /// Returns the raw CoreMIDI value for this enum.
            #[must_use]
            pub const fn as_raw(self) -> $repr {
                self as $repr
            }

            /// Converts a raw CoreMIDI value into this enum when it is known.
            #[must_use]
            pub const fn from_raw(raw: $repr) -> Option<Self> {
                match raw {
                    $($value => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

macro_rules! midi_bitflags {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($repr:ty) { $($flag:ident = $value:expr),+ $(,)? }) => {
        $(#[$meta])*
        /// Wraps matching CoreMIDI option-bit values.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        $vis struct $name($repr);

        impl $name {
            $(#[doc = "Wraps the matching CoreMIDI option flag."] pub const $flag: Self = Self($value);)+

            /// Returns an empty set of CoreMIDI option flags.
            #[must_use]
            pub const fn empty() -> Self {
                Self(0)
            }

            /// Returns the raw CoreMIDI option bits.
            #[must_use]
            pub const fn bits(self) -> $repr {
                self.0
            }

            /// Wraps a raw CoreMIDI option bitset without clearing unknown bits.
            #[must_use]
            pub const fn from_bits_retain(bits: $repr) -> Self {
                Self(bits)
            }

            /// Tests whether the CoreMIDI option set contains another flag set.
            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// Tests whether no CoreMIDI option flags are set.
            #[must_use]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }
        }

        impl From<$repr> for $name {
            fn from(bits: $repr) -> Self {
                Self::from_bits_retain(bits)
            }
        }

        impl From<$name> for $repr {
            fn from(options: $name) -> Self {
                options.bits()
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    };
}

midi_enum!(
    pub enum MidiMessageType: u32 {
        Utility = 0x0,
        System = 0x1,
        ChannelVoice1 = 0x2,
        SysEx = 0x3,
        ChannelVoice2 = 0x4,
        Data128 = 0x5,
        FlexData = 0xD,
        Stream = 0xF,
        Invalid = 0xFF,
    }
);

impl MidiMessageType {
    #[must_use]
    /// Wraps the CoreMIDI from up word operation for `MidiMessageType`.
    pub const fn from_up_word(word: u32) -> Option<Self> {
        Self::from_raw((word >> 28) & 0xF)
    }
}

midi_enum!(
    pub enum MidiCvStatus: u32 {
        NoteOff = 0x8,
        NoteOn = 0x9,
        PolyPressure = 0xA,
        ControlChange = 0xB,
        ProgramChange = 0xC,
        ChannelPressure = 0xD,
        PitchBend = 0xE,
        RegisteredPnc = 0x0,
        AssignablePnc = 0x1,
        RegisteredControl = 0x2,
        AssignableControl = 0x3,
        RelRegisteredControl = 0x4,
        RelAssignableControl = 0x5,
        PerNotePitchBend = 0x6,
        PerNoteMgmt = 0xF,
    }
);

midi_enum!(
    pub enum MidiSystemStatus: u32 {
        StartOfExclusive = 0xF0,
        EndOfExclusive = 0xF7,
        Mtc = 0xF1,
        SongPosPointer = 0xF2,
        SongSelect = 0xF3,
        TuneRequest = 0xF6,
        TimingClock = 0xF8,
        Start = 0xFA,
        Continue = 0xFB,
        Stop = 0xFC,
        ActiveSensing = 0xFE,
        SystemReset = 0xFF,
    }
);

impl MidiSystemStatus {
    /// Aliases `ActiveSensing` for the matching CoreMIDI status value.
    pub const ACTIVE_SENDING: Self = Self::ActiveSensing;
}

midi_enum!(
    pub enum MidiSysExStatus: u32 {
        Complete = 0x0,
        Start = 0x1,
        Continue = 0x2,
        End = 0x3,
        MixedDataSetHeader = 0x8,
        MixedDataSetPayload = 0x9,
    }
);

midi_enum!(
    pub enum MidiUtilityStatus: u32 {
        Noop = 0x0,
        JitterReductionClock = 0x1,
        JitterReductionTimestamp = 0x2,
        DeltaClockstampTicksPerQuarterNote = 0x3,
        TicksSinceLastEvent = 0x4,
    }
);

midi_enum!(
    pub enum UmpStreamMessageStatus: u32 {
        EndpointDiscovery = 0x00,
        EndpointInfoNotification = 0x01,
        DeviceIdentityNotification = 0x02,
        EndpointNameNotification = 0x03,
        ProductInstanceIdNotification = 0x04,
        StreamConfigurationRequest = 0x05,
        StreamConfigurationNotification = 0x06,
        FunctionBlockDiscovery = 0x10,
        FunctionBlockInfoNotification = 0x11,
        FunctionBlockNameNotification = 0x12,
        StartOfClip = 0x20,
        EndOfClip = 0x21,
    }
);

midi_enum!(
    pub enum UmpStreamMessageFormat: u8 {
        Complete = 0x00,
        Start = 0x01,
        Continuing = 0x02,
        End = 0x03,
    }
);

midi_enum!(
    pub enum MidiNoteAttribute: u8 {
        None = 0x0,
        ManufacturerSpecific = 0x1,
        ProfileSpecific = 0x2,
        Pitch = 0x3,
    }
);

midi_bitflags!(
    pub struct MidiProgramChangeOptions(u8) {
        BANK_VALID = 0x1,
    }
);

midi_bitflags!(
    pub struct MidiPerNoteManagementOptions(u8) {
        RESET = 0x1,
        DETACH = 0x2,
    }
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIMessage_64`.
pub struct MidiMessage64(ffi::MIDIMessage_64);

impl MidiMessage64 {
    #[must_use]
    /// Wraps the CoreMIDI new operation for `MidiMessage64`.
    pub const fn new(word0: u32, word1: u32) -> Self {
        Self(ffi::MIDIMessage_64 { word0, word1 })
    }

    #[must_use]
    /// Returns the wrapped `MIDIMessage_64`.
    pub const fn as_raw(self) -> ffi::MIDIMessage_64 {
        self.0
    }

    #[must_use]
    /// Wraps an existing `MIDIMessage_64`.
    pub const fn from_raw(raw: ffi::MIDIMessage_64) -> Self {
        Self(raw)
    }

    #[must_use]
    /// Wraps the CoreMIDI words operation for `MidiMessage64`.
    pub const fn words(self) -> [u32; 2] {
        [self.0.word0, self.0.word1]
    }

    #[must_use]
    /// Wraps the CoreMIDI message type operation for `MidiMessage64`.
    pub fn message_type(self) -> Option<MidiMessageType> {
        MidiMessageType::from_up_word(self.0.word0)
    }
}

impl From<ffi::MIDIMessage_64> for MidiMessage64 {
    fn from(raw: ffi::MIDIMessage_64) -> Self {
        Self::from_raw(raw)
    }
}

impl From<MidiMessage64> for ffi::MIDIMessage_64 {
    fn from(message: MidiMessage64) -> Self {
        message.as_raw()
    }
}

impl From<[u32; 2]> for MidiMessage64 {
    fn from(words: [u32; 2]) -> Self {
        Self::new(words[0], words[1])
    }
}

impl From<MidiMessage64> for [u32; 2] {
    fn from(message: MidiMessage64) -> Self {
        message.words()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIMessage_96`.
pub struct MidiMessage96(ffi::MIDIMessage_96);

impl MidiMessage96 {
    #[must_use]
    /// Wraps the CoreMIDI new operation for `MidiMessage96`.
    pub const fn new(word0: u32, word1: u32, word2: u32) -> Self {
        Self(ffi::MIDIMessage_96 {
            word0,
            word1,
            word2,
        })
    }

    #[must_use]
    /// Returns the wrapped `MIDIMessage_96`.
    pub const fn as_raw(self) -> ffi::MIDIMessage_96 {
        self.0
    }

    #[must_use]
    /// Wraps an existing `MIDIMessage_96`.
    pub const fn from_raw(raw: ffi::MIDIMessage_96) -> Self {
        Self(raw)
    }

    #[must_use]
    /// Wraps the CoreMIDI words operation for `MidiMessage96`.
    pub const fn words(self) -> [u32; 3] {
        [self.0.word0, self.0.word1, self.0.word2]
    }

    #[must_use]
    /// Wraps the CoreMIDI message type operation for `MidiMessage96`.
    pub fn message_type(self) -> Option<MidiMessageType> {
        MidiMessageType::from_up_word(self.0.word0)
    }
}

impl From<ffi::MIDIMessage_96> for MidiMessage96 {
    fn from(raw: ffi::MIDIMessage_96) -> Self {
        Self::from_raw(raw)
    }
}

impl From<MidiMessage96> for ffi::MIDIMessage_96 {
    fn from(message: MidiMessage96) -> Self {
        message.as_raw()
    }
}

impl From<[u32; 3]> for MidiMessage96 {
    fn from(words: [u32; 3]) -> Self {
        Self::new(words[0], words[1], words[2])
    }
}

impl From<MidiMessage96> for [u32; 3] {
    fn from(message: MidiMessage96) -> Self {
        message.words()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `MIDIMessage_128`.
pub struct MidiMessage128(ffi::MIDIMessage_128);

impl MidiMessage128 {
    #[must_use]
    /// Wraps the CoreMIDI new operation for `MidiMessage128`.
    pub const fn new(word0: u32, word1: u32, word2: u32, word3: u32) -> Self {
        Self(ffi::MIDIMessage_128 {
            word0,
            word1,
            word2,
            word3,
        })
    }

    #[must_use]
    /// Returns the wrapped `MIDIMessage_128`.
    pub const fn as_raw(self) -> ffi::MIDIMessage_128 {
        self.0
    }

    #[must_use]
    /// Wraps an existing `MIDIMessage_128`.
    pub const fn from_raw(raw: ffi::MIDIMessage_128) -> Self {
        Self(raw)
    }

    #[must_use]
    /// Wraps the CoreMIDI words operation for `MidiMessage128`.
    pub const fn words(self) -> [u32; 4] {
        [self.0.word0, self.0.word1, self.0.word2, self.0.word3]
    }

    #[must_use]
    /// Wraps the CoreMIDI message type operation for `MidiMessage128`.
    pub fn message_type(self) -> Option<MidiMessageType> {
        MidiMessageType::from_up_word(self.0.word0)
    }
}

impl From<ffi::MIDIMessage_128> for MidiMessage128 {
    fn from(raw: ffi::MIDIMessage_128) -> Self {
        Self::from_raw(raw)
    }
}

impl From<MidiMessage128> for ffi::MIDIMessage_128 {
    fn from(message: MidiMessage128) -> Self {
        message.as_raw()
    }
}

impl From<[u32; 4]> for MidiMessage128 {
    fn from(words: [u32; 4]) -> Self {
        Self::new(words[0], words[1], words[2], words[3])
    }
}

impl From<MidiMessage128> for [u32; 4] {
    fn from(message: MidiMessage128) -> Self {
        message.words()
    }
}

#[derive(Debug, Clone)]
/// Wraps `MIDIPacketList`.
pub struct PacketListBuffer {
    storage: Vec<u64>,
    current_packet: *mut ffi::MIDIPacket,
}

impl PacketListBuffer {
    #[must_use]
    /// Wraps `MIDIPacketListInit`.
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

    /// Wraps `MIDIPacketListInit`.
    pub fn clear(&mut self) {
        self.current_packet = unsafe { ffi::MIDIPacketListInit(self.storage.as_mut_ptr().cast()) };
    }

    #[must_use]
    /// Wraps the CoreMIDI capacity bytes operation for `PacketListBuffer`.
    pub fn capacity_bytes(&self) -> usize {
        self.storage.len() * size_of::<u64>()
    }

    /// Wraps `MIDIPacketListAdd`.
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
    /// Wraps the CoreMIDI as ptr operation for `PacketListBuffer`.
    pub fn as_ptr(&self) -> *const ffi::MIDIPacketList {
        self.storage.as_ptr().cast()
    }

    #[must_use]
    /// Wraps the CoreMIDI as packet list operation for `PacketListBuffer`.
    pub fn as_packet_list(&self) -> PacketListRef<'_> {
        unsafe { PacketListRef::from_ptr(self.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIPacketList`.
pub struct PacketListRef<'a> {
    ptr: *const ffi::MIDIPacketList,
    _marker: PhantomData<&'a ffi::MIDIPacketList>,
}

impl<'a> PacketListRef<'a> {
    #[must_use]
    /// Wraps the CoreMIDI from ptr operation for `PacketListRef`.
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIPacketList) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI packet count operation for `PacketListRef`.
    pub fn packet_count(self) -> u32 {
        unsafe { ptr::addr_of!((*self.ptr).numPackets).read_unaligned() }
    }

    #[must_use]
    /// Wraps the CoreMIDI iter operation for `PacketListRef`.
    pub fn iter(self) -> PacketIter<'a> {
        PacketIter {
            next_packet: unsafe { ptr::addr_of!((*self.ptr).packet).cast() },
            remaining: self.packet_count(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI as ptr operation for `PacketListRef`.
    pub const fn as_ptr(self) -> *const ffi::MIDIPacketList {
        self.ptr
    }
}

#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIPacket`.
pub struct MidiPacketRef<'a> {
    ptr: *const ffi::MIDIPacket,
    _marker: PhantomData<&'a ffi::MIDIPacket>,
}

impl<'a> MidiPacketRef<'a> {
    #[must_use]
    /// Wraps the CoreMIDI from ptr operation for `MidiPacketRef`.
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIPacket) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI timestamp operation for `MidiPacketRef`.
    pub fn timestamp(self) -> ffi::MIDITimeStamp {
        unsafe { ptr::addr_of!((*self.ptr).timeStamp).read_unaligned() }
    }

    #[must_use]
    /// Wraps the CoreMIDI len operation for `MidiPacketRef`.
    pub fn len(self) -> usize {
        usize::from(unsafe { ptr::addr_of!((*self.ptr).length).read_unaligned() })
    }

    #[must_use]
    /// Wraps the CoreMIDI is empty operation for `MidiPacketRef`.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[must_use]
    /// Wraps the CoreMIDI bytes operation for `MidiPacketRef`.
    pub fn bytes(self) -> &'a [u8] {
        let len = self.len();
        let data_ptr = unsafe { ptr::addr_of!((*self.ptr).data).cast::<u8>() };
        unsafe { slice::from_raw_parts(data_ptr, len) }
    }
}

#[derive(Debug, Clone)]
/// Iterates CoreMIDI packet values.
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
/// Wraps `MIDIEventList`.
pub struct EventListBuffer {
    storage: Vec<u64>,
    current_packet: *mut ffi::MIDIEventPacket,
    protocol: MidiProtocol,
}

impl EventListBuffer {
    #[must_use]
    /// Wraps `MIDIEventListInit`.
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

    /// Wraps `MIDIEventListInit`.
    pub fn clear(&mut self) {
        self.current_packet = unsafe {
            ffi::MIDIEventListInit(self.storage.as_mut_ptr().cast(), self.protocol.as_raw())
        };
    }

    #[must_use]
    /// Wraps the CoreMIDI protocol operation for `EventListBuffer`.
    pub const fn protocol(&self) -> MidiProtocol {
        self.protocol
    }

    #[must_use]
    /// Wraps the CoreMIDI capacity bytes operation for `EventListBuffer`.
    pub fn capacity_bytes(&self) -> usize {
        self.storage.len() * size_of::<u64>()
    }

    /// Wraps `MIDIEventListAdd`.
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
    /// Wraps the CoreMIDI as ptr operation for `EventListBuffer`.
    pub fn as_ptr(&self) -> *const ffi::MIDIEventList {
        self.storage.as_ptr().cast()
    }

    #[must_use]
    /// Wraps the CoreMIDI as event list operation for `EventListBuffer`.
    pub fn as_event_list(&self) -> EventListRef<'_> {
        unsafe { EventListRef::from_ptr(self.as_ptr()) }
    }
}

#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIEventList`.
pub struct EventListRef<'a> {
    ptr: *const ffi::MIDIEventList,
    _marker: PhantomData<&'a ffi::MIDIEventList>,
}

impl<'a> EventListRef<'a> {
    #[must_use]
    /// Wraps the CoreMIDI from ptr operation for `EventListRef`.
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIEventList) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI packet count operation for `EventListRef`.
    pub fn packet_count(self) -> u32 {
        unsafe { ptr::addr_of!((*self.ptr).numPackets).read_unaligned() }
    }

    #[must_use]
    /// Wraps the CoreMIDI protocol operation for `EventListRef`.
    pub fn protocol(self) -> Option<MidiProtocol> {
        let raw = unsafe { ptr::addr_of!((*self.ptr).protocol).read_unaligned() };
        MidiProtocol::from_raw(raw)
    }

    #[must_use]
    /// Wraps the CoreMIDI iter operation for `EventListRef`.
    pub fn iter(self) -> EventIter<'a> {
        EventIter {
            next_packet: unsafe { ptr::addr_of!((*self.ptr).packet).cast() },
            remaining: self.packet_count(),
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI as ptr operation for `EventListRef`.
    pub const fn as_ptr(self) -> *const ffi::MIDIEventList {
        self.ptr
    }
}

#[derive(Debug, Clone, Copy)]
/// Wraps `MIDIEventPacket`.
pub struct MidiEventPacketRef<'a> {
    ptr: *const ffi::MIDIEventPacket,
    _marker: PhantomData<&'a ffi::MIDIEventPacket>,
}

impl<'a> MidiEventPacketRef<'a> {
    #[must_use]
    /// Wraps the CoreMIDI from ptr operation for `MidiEventPacketRef`.
    pub const unsafe fn from_ptr(ptr: *const ffi::MIDIEventPacket) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Wraps the CoreMIDI timestamp operation for `MidiEventPacketRef`.
    pub fn timestamp(self) -> ffi::MIDITimeStamp {
        unsafe { ptr::addr_of!((*self.ptr).timeStamp).read_unaligned() }
    }

    #[must_use]
    /// Wraps the CoreMIDI word count operation for `MidiEventPacketRef`.
    pub fn word_count(self) -> usize {
        usize::try_from(unsafe { ptr::addr_of!((*self.ptr).wordCount).read_unaligned() })
            .unwrap_or(0)
    }

    #[must_use]
    /// Wraps the CoreMIDI words operation for `MidiEventPacketRef`.
    pub fn words(self) -> &'a [u32] {
        let count = self.word_count();
        let words_ptr = unsafe { ptr::addr_of!((*self.ptr).words).cast::<u32>() };
        unsafe { slice::from_raw_parts(words_ptr, count) }
    }

    #[must_use]
    /// Wraps the CoreMIDI message type operation for `MidiEventPacketRef`.
    pub fn message_type(self) -> Option<MidiMessageType> {
        self.words()
            .first()
            .and_then(|word| MidiMessageType::from_up_word(*word))
    }
}

#[derive(Debug, Clone)]
/// Iterates CoreMIDI event values.
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
