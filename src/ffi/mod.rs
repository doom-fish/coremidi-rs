#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type OSStatus = i32;
pub const noErr: OSStatus = 0;

pub type Byte = u8;
pub type ByteCount = usize;
pub type ItemCount = usize;
pub type MIDITimeStamp = u64;
pub type MIDIObjectRef = u32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDIProtocolID = i32;
pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFIndex = isize;

pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

pub const kMIDIInvalidClient: OSStatus = -10_830;
pub const kMIDIInvalidPort: OSStatus = -10_831;
pub const kMIDIWrongEndpointType: OSStatus = -10_832;
pub const kMIDINoConnection: OSStatus = -10_833;
pub const kMIDIUnknownEndpoint: OSStatus = -10_834;
pub const kMIDIUnknownProperty: OSStatus = -10_835;
pub const kMIDIWrongPropertyType: OSStatus = -10_836;
pub const kMIDINoCurrentSetup: OSStatus = -10_837;
pub const kMIDIMessageSendErr: OSStatus = -10_838;
pub const kMIDIServerStartErr: OSStatus = -10_839;
pub const kMIDISetupFormatErr: OSStatus = -10_840;
pub const kMIDIWrongThread: OSStatus = -10_841;
pub const kMIDIObjectNotFound: OSStatus = -10_842;
pub const kMIDIIDNotUnique: OSStatus = -10_843;
pub const kMIDINotPermitted: OSStatus = -10_844;
pub const kMIDIUnknownError: OSStatus = -10_845;

pub const kMIDIProtocol_1_0: MIDIProtocolID = 1;
pub const kMIDIProtocol_2_0: MIDIProtocolID = 2;

pub type MIDINotifyProc = Option<unsafe extern "C" fn(*const MIDINotification, *mut c_void)>;
pub type MIDIReadProc =
    Option<unsafe extern "C" fn(*const MIDIPacketList, *mut c_void, *mut c_void)>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDINotification {
    pub messageID: i32,
    pub messageSize: u32,
}

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MIDIEventPacket {
    pub timeStamp: MIDITimeStamp,
    pub wordCount: u32,
    pub words: [u32; 64],
}

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MIDIEventList {
    pub protocol: MIDIProtocolID,
    pub numPackets: u32,
    pub packet: [MIDIEventPacket; 1],
}

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MIDIPacket {
    pub timeStamp: MIDITimeStamp,
    pub length: u16,
    pub data: [Byte; 256],
}

#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct MIDIPacketList {
    pub numPackets: u32,
    pub packet: [MIDIPacket; 1],
}

/// Opaque 24-byte representation of CoreMIDI's `MIDIUniversalMessage` struct.
///
/// The public v0.1 surface uses `MIDIEventList` / `MIDIEventPacket` for MIDI 2.0
/// transport; this type is re-exported so downstream callers can store or pass
/// parsed UMP payloads without a Swift bridge.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIUniversalMessage {
    pub raw_words: [u32; 6],
}

#[link(name = "CoreMIDI", kind = "framework")]
extern "C" {
    pub static kMIDIPropertyName: CFStringRef;
    pub static kMIDIPropertyManufacturer: CFStringRef;
    pub static kMIDIPropertyModel: CFStringRef;
    pub static kMIDIPropertyUniqueID: CFStringRef;

    pub fn MIDIClientCreate(
        name: CFStringRef,
        notifyProc: MIDINotifyProc,
        notifyRefCon: *mut c_void,
        outClient: *mut MIDIClientRef,
    ) -> OSStatus;
    pub fn MIDIClientDispose(client: MIDIClientRef) -> OSStatus;

    pub fn MIDIInputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut c_void,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
    pub fn MIDIInputPortCreateWithProtocol(
        client: MIDIClientRef,
        portName: CFStringRef,
        protocol: MIDIProtocolID,
        outPort: *mut MIDIPortRef,
        receiveBlock: *const c_void,
    ) -> OSStatus;
    pub fn MIDIOutputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
    pub fn MIDIPortDispose(port: MIDIPortRef) -> OSStatus;
    pub fn MIDIPortConnectSource(
        port: MIDIPortRef,
        source: MIDIEndpointRef,
        connRefCon: *mut c_void,
    ) -> OSStatus;
    pub fn MIDIPortDisconnectSource(port: MIDIPortRef, source: MIDIEndpointRef) -> OSStatus;

    pub fn MIDIGetNumberOfDevices() -> ItemCount;
    pub fn MIDIGetDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
    pub fn MIDIDeviceGetNumberOfEntities(device: MIDIDeviceRef) -> ItemCount;
    pub fn MIDIDeviceGetEntity(device: MIDIDeviceRef, entityIndex0: ItemCount) -> MIDIEntityRef;
    pub fn MIDIEntityGetNumberOfSources(entity: MIDIEntityRef) -> ItemCount;
    pub fn MIDIEntityGetSource(entity: MIDIEntityRef, sourceIndex0: ItemCount) -> MIDIEndpointRef;
    pub fn MIDIEntityGetNumberOfDestinations(entity: MIDIEntityRef) -> ItemCount;
    pub fn MIDIEntityGetDestination(
        entity: MIDIEntityRef,
        destIndex0: ItemCount,
    ) -> MIDIEndpointRef;

    pub fn MIDIDestinationCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut c_void,
        outDest: *mut MIDIEndpointRef,
    ) -> OSStatus;
    pub fn MIDISourceCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        outSrc: *mut MIDIEndpointRef,
    ) -> OSStatus;
    pub fn MIDIEndpointDispose(endpt: MIDIEndpointRef) -> OSStatus;

    pub fn MIDIObjectGetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outValue: *mut i32,
    ) -> OSStatus;
    pub fn MIDIObjectGetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outValue: *mut CFStringRef,
    ) -> OSStatus;

    pub fn MIDISend(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        pktlist: *const MIDIPacketList,
    ) -> OSStatus;
    pub fn MIDISendEventList(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        evtlist: *const MIDIEventList,
    ) -> OSStatus;
    pub fn MIDIReceived(src: MIDIEndpointRef, pktlist: *const MIDIPacketList) -> OSStatus;
    pub fn MIDIReceivedEventList(src: MIDIEndpointRef, evtlist: *const MIDIEventList) -> OSStatus;

    pub fn MIDIPacketListInit(pktlist: *mut MIDIPacketList) -> *mut MIDIPacket;
    pub fn MIDIPacketListAdd(
        pktlist: *mut MIDIPacketList,
        listSize: ByteCount,
        curPacket: *mut MIDIPacket,
        time: MIDITimeStamp,
        nData: ByteCount,
        data: *const Byte,
    ) -> *mut MIDIPacket;

    pub fn MIDIEventListInit(
        evtlist: *mut MIDIEventList,
        protocol: MIDIProtocolID,
    ) -> *mut MIDIEventPacket;
    pub fn MIDIEventListAdd(
        evtlist: *mut MIDIEventList,
        listSize: ByteCount,
        curPacket: *mut MIDIEventPacket,
        time: MIDITimeStamp,
        wordCount: ByteCount,
        words: *const u32,
    ) -> *mut MIDIEventPacket;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetCString(
        string: CFStringRef,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: u32,
    ) -> bool;
    pub fn CFStringGetLength(string: CFStringRef) -> CFIndex;
    pub fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: u32) -> CFIndex;
}
