#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type OSStatus = i32;
pub const noErr: OSStatus = 0;

pub type Byte = u8;
pub type Boolean = u8;
pub type ByteCount = usize;
pub type ItemCount = usize;
pub type MIDITimeStamp = u64;
pub type MIDIObjectRef = u32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDISetupRef = MIDIObjectRef;
pub type MIDIDeviceListRef = MIDIObjectRef;
pub type MIDIThruConnectionRef = MIDIObjectRef;
pub type MIDIProtocolID = i32;
pub type MIDIObjectType = i32;
pub type MIDIUniqueID = i32;
pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFPropertyListRef = *const c_void;
pub type CFRunLoopRef = *const c_void;
pub type CFUUIDRef = *const c_void;
pub type CFIndex = isize;

pub type MIDIUInteger2 = u8;
pub type MIDIUInteger4 = u8;
pub type MIDIUInteger7 = u8;
pub type MIDIUInteger14 = u16;
pub type MIDIUInteger28 = u32;
pub type MIDIUMPGroupNumber = MIDIUInteger4;
pub type MIDIChannelNumber = MIDIUInteger4;
pub type MIDICIDeviceID = MIDIUInteger7;
pub type MIDICIMUID = MIDIUInteger28;
pub type MIDIUMPFunctionBlockID = MIDIUInteger7;
pub type MIDIUMPProtocolOptions = MIDIUInteger4;
pub type MIDIUMPFunctionBlockMIDI1Info = i32;
pub type MIDIUMPFunctionBlockUIHint = i32;
pub type MIDIUMPFunctionBlockDirection = i32;
pub type MIDICIDeviceType = u8;
pub type MIDICIProfileType = u8;
pub type MIDIUMPCIObjectBackingType = u8;

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

pub const kMIDIObjectType_Other: MIDIObjectType = -1;
pub const kMIDIObjectType_Device: MIDIObjectType = 0;
pub const kMIDIObjectType_Entity: MIDIObjectType = 1;
pub const kMIDIObjectType_Source: MIDIObjectType = 2;
pub const kMIDIObjectType_Destination: MIDIObjectType = 3;
pub const kMIDIObjectType_ExternalMask: MIDIObjectType = 0x10;
pub const kMIDIObjectType_ExternalDevice: MIDIObjectType = 0x10 | kMIDIObjectType_Device;
pub const kMIDIObjectType_ExternalEntity: MIDIObjectType = 0x10 | kMIDIObjectType_Entity;
pub const kMIDIObjectType_ExternalSource: MIDIObjectType = 0x10 | kMIDIObjectType_Source;
pub const kMIDIObjectType_ExternalDestination: MIDIObjectType = 0x10 | kMIDIObjectType_Destination;
pub const kMIDIInvalidUniqueID: MIDIUniqueID = 0;

pub const kMIDIProtocol_1_0: MIDIProtocolID = 1;
pub const kMIDIProtocol_2_0: MIDIProtocolID = 2;

pub const kMIDIMsgSetupChanged: i32 = 1;
pub const kMIDIMsgObjectAdded: i32 = 2;
pub const kMIDIMsgObjectRemoved: i32 = 3;
pub const kMIDIMsgPropertyChanged: i32 = 4;
pub const kMIDIMsgThruConnectionsChanged: i32 = 5;
pub const kMIDIMsgSerialPortOwnerChanged: i32 = 6;
pub const kMIDIMsgIOError: i32 = 7;
pub const kMIDIMsgInternalStart: i32 = 0x1000;

pub const kMIDITransform_None: u16 = 0;
pub const kMIDITransform_FilterOut: u16 = 1;
pub const kMIDITransform_MapControl: u16 = 2;
pub const kMIDITransform_Add: u16 = 8;
pub const kMIDITransform_Scale: u16 = 9;
pub const kMIDITransform_MinValue: u16 = 10;
pub const kMIDITransform_MaxValue: u16 = 11;
pub const kMIDITransform_MapValue: u16 = 12;

pub const kMIDIThruConnection_MaxEndpoints: usize = 8;
pub const kMIDIControlType_7Bit: u8 = 0;
pub const kMIDIControlType_14Bit: u8 = 1;
pub const kMIDIControlType_7BitRPN: u8 = 2;
pub const kMIDIControlType_14BitRPN: u8 = 3;
pub const kMIDIControlType_7BitNRPN: u8 = 4;
pub const kMIDIControlType_14BitNRPN: u8 = 5;

pub const kMIDIUMPFunctionBlockMIDI1InfoNotMIDI1: MIDIUMPFunctionBlockMIDI1Info = 0;
pub const kMIDIUMPFunctionBlockMIDI1InfoUnrestrictedBandwidth: MIDIUMPFunctionBlockMIDI1Info = 1;
pub const kMIDIUMPFunctionBlockMIDI1InfoRestrictedBandwidth: MIDIUMPFunctionBlockMIDI1Info = 2;
pub const kMIDIUMPFunctionBlockUIHintUnknown: MIDIUMPFunctionBlockUIHint = 0;
pub const kMIDIUMPFunctionBlockUIHintReceiver: MIDIUMPFunctionBlockUIHint = 1;
pub const kMIDIUMPFunctionBlockUIHintSender: MIDIUMPFunctionBlockUIHint = 2;
pub const kMIDIUMPFunctionBlockUIHintSenderReceiver: MIDIUMPFunctionBlockUIHint = 3;
pub const kMIDIUMPFunctionBlockDirectionUnknown: MIDIUMPFunctionBlockDirection = 0;
pub const kMIDIUMPFunctionBlockDirectionInput: MIDIUMPFunctionBlockDirection = 1;
pub const kMIDIUMPFunctionBlockDirectionOutput: MIDIUMPFunctionBlockDirection = 2;
pub const kMIDIUMPFunctionBlockDirectionBidirectional: MIDIUMPFunctionBlockDirection = 3;
pub const kMIDIUMPProtocolOptionsMIDI1: MIDIUMPProtocolOptions = 1;
pub const kMIDIUMPProtocolOptionsMIDI2: MIDIUMPProtocolOptions = 1 << 1;
pub const kMIDICIDeviceTypeUnknown: MIDICIDeviceType = 0;
pub const kMIDICIDeviceTypeLegacyMIDI1: MIDICIDeviceType = 1;
pub const kMIDICIDeviceTypeVirtual: MIDICIDeviceType = 2;
pub const kMIDICIDeviceTypeUSBMIDI: MIDICIDeviceType = 3;
pub const kMIDICIProfileTypeSingleChannel: MIDICIProfileType = 1;
pub const kMIDICIProfileTypeGroup: MIDICIProfileType = 2;
pub const kMIDICIProfileTypeFunctionBlock: MIDICIProfileType = 3;
pub const kMIDICIProfileTypeMultichannel: MIDICIProfileType = 4;
pub const kMIDIUMPCIObjectBackingTypeUnknown: MIDIUMPCIObjectBackingType = 0;
pub const kMIDIUMPCIObjectBackingTypeVirtual: MIDIUMPCIObjectBackingType = 1;
pub const kMIDIUMPCIObjectBackingTypeDriverDevice: MIDIUMPCIObjectBackingType = 2;
pub const kMIDIUMPCIObjectBackingTypeUSBMIDI: MIDIUMPCIObjectBackingType = 3;
pub const kMIDIDeviceIDUMPGroup: MIDICIDeviceID = 0x7e;
pub const kMIDIDeviceIDFunctionBlock: MIDICIDeviceID = 0x7f;

pub type MIDINotifyProc = Option<unsafe extern "C" fn(*const MIDINotification, *mut c_void)>;
pub type MIDIReadProc =
    Option<unsafe extern "C" fn(*const MIDIPacketList, *mut c_void, *mut c_void)>;
pub type MIDICompletionProc = Option<unsafe extern "C" fn(*mut MIDISysexSendRequest)>;
pub type MIDICompletionProcUMP = Option<unsafe extern "C" fn(*mut MIDISysexSendRequestUMP)>;
pub type MIDINotifyBlock = *const c_void;
pub type MIDIReadBlock = *const c_void;
pub type MIDIReceiveBlock = *const c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDINotification {
    pub messageID: i32,
    pub messageSize: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDIObjectAddRemoveNotification {
    pub messageID: i32,
    pub messageSize: u32,
    pub parent: MIDIObjectRef,
    pub parentType: MIDIObjectType,
    pub child: MIDIObjectRef,
    pub childType: MIDIObjectType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDIObjectPropertyChangeNotification {
    pub messageID: i32,
    pub messageSize: u32,
    pub object: MIDIObjectRef,
    pub objectType: MIDIObjectType,
    pub propertyName: CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDIIOErrorNotification {
    pub messageID: i32,
    pub messageSize: u32,
    pub driverDevice: MIDIDeviceRef,
    pub errorCode: OSStatus,
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIUniversalMessage {
    pub raw_words: [u32; 6],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIMessage_64 {
    pub word0: u32,
    pub word1: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIMessage_96 {
    pub word0: u32,
    pub word1: u32,
    pub word2: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIMessage_128 {
    pub word0: u32,
    pub word1: u32,
    pub word2: u32,
    pub word3: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDISysexSendRequest {
    pub destination: MIDIEndpointRef,
    pub data: *const Byte,
    pub bytesToSend: u32,
    pub complete: Boolean,
    pub reserved: [Byte; 3],
    pub completionProc: MIDICompletionProc,
    pub completionRefCon: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MIDISysexSendRequestUMP {
    pub destination: MIDIEndpointRef,
    pub words: *mut u32,
    pub wordsToSend: u32,
    pub complete: Boolean,
    pub completionProc: MIDICompletionProcUMP,
    pub completionRefCon: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIValueMap {
    pub value: [u8; 128],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDITransform {
    pub transform: u16,
    pub param: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIControlTransform {
    pub controlType: u8,
    pub remappedControlType: u8,
    pub controlNumber: u16,
    pub transform: u16,
    pub param: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIThruConnectionEndpoint {
    pub endpointRef: MIDIEndpointRef,
    pub uniqueID: MIDIUniqueID,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDIThruConnectionParams {
    pub version: u32,
    pub numSources: u32,
    pub sources: [MIDIThruConnectionEndpoint; kMIDIThruConnection_MaxEndpoints],
    pub numDestinations: u32,
    pub destinations: [MIDIThruConnectionEndpoint; kMIDIThruConnection_MaxEndpoints],
    pub channelMap: [u8; 16],
    pub lowVelocity: u8,
    pub highVelocity: u8,
    pub lowNote: u8,
    pub highNote: u8,
    pub noteNumber: MIDITransform,
    pub velocity: MIDITransform,
    pub keyPressure: MIDITransform,
    pub channelPressure: MIDITransform,
    pub programChange: MIDITransform,
    pub pitchBend: MIDITransform,
    pub filterOutSysEx: u8,
    pub filterOutMTC: u8,
    pub filterOutBeatClock: u8,
    pub filterOutTuneRequest: u8,
    pub reserved2: [u8; 3],
    pub filterOutAllControls: u8,
    pub numControlTransforms: u16,
    pub numMaps: u16,
    pub reserved3: [u16; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDI2DeviceManufacturer {
    pub sysExIDByte: [u8; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDI2DeviceRevisionLevel {
    pub revisionLevel: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDICIProfileIDStandard {
    pub profileIDByte1: MIDIUInteger7,
    pub profileBank: MIDIUInteger7,
    pub profileNumber: MIDIUInteger7,
    pub profileVersion: MIDIUInteger7,
    pub profileLevel: MIDIUInteger7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MIDICIProfileIDManufacturerSpecific {
    pub sysExID1: MIDIUInteger7,
    pub sysExID2: MIDIUInteger7,
    pub sysExID3: MIDIUInteger7,
    pub info1: MIDIUInteger7,
    pub info2: MIDIUInteger7,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MIDICIProfileID {
    pub standard: MIDICIProfileIDStandard,
    pub manufacturerSpecific: MIDICIProfileIDManufacturerSpecific,
}

#[repr(C)]
pub struct MIDIDriverInterface {
    _private: [u8; 0],
}

pub type MIDIDriverRef = *mut *mut MIDIDriverInterface;

mod raw_coremidi {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[link(name = "CoreMIDI", kind = "framework")]
    extern "C" {
        pub static kMIDIPropertyName: CFStringRef;
        pub static kMIDIPropertyManufacturer: CFStringRef;
        pub static kMIDIPropertyModel: CFStringRef;
        pub static kMIDIPropertyUniqueID: CFStringRef;
        pub static kMIDIPropertyDeviceID: CFStringRef;
        pub static kMIDIPropertyReceiveChannels: CFStringRef;
        pub static kMIDIPropertyTransmitChannels: CFStringRef;
        pub static kMIDIPropertyMaxSysExSpeed: CFStringRef;
        pub static kMIDIPropertyAdvanceScheduleTimeMuSec: CFStringRef;
        pub static kMIDIPropertyIsEmbeddedEntity: CFStringRef;
        pub static kMIDIPropertyIsBroadcast: CFStringRef;
        pub static kMIDIPropertySingleRealtimeEntity: CFStringRef;
        pub static kMIDIPropertyConnectionUniqueID: CFStringRef;
        pub static kMIDIPropertyOffline: CFStringRef;
        pub static kMIDIPropertyPrivate: CFStringRef;
        pub static kMIDIPropertyDriverOwner: CFStringRef;
        pub static kMIDIPropertyFactoryPatchNameFile: CFStringRef;
        pub static kMIDIPropertyUserPatchNameFile: CFStringRef;
        pub static kMIDIPropertyNameConfiguration: CFStringRef;
        pub static kMIDIPropertyNameConfigurationDictionary: CFStringRef;
        pub static kMIDIPropertyImage: CFStringRef;
        pub static kMIDIPropertyDriverVersion: CFStringRef;
        pub static kMIDIPropertySupportsGeneralMIDI: CFStringRef;
        pub static kMIDIPropertySupportsMMC: CFStringRef;
        pub static kMIDIPropertyCanRoute: CFStringRef;
        pub static kMIDIPropertyReceivesClock: CFStringRef;
        pub static kMIDIPropertyReceivesMTC: CFStringRef;
        pub static kMIDIPropertyReceivesNotes: CFStringRef;
        pub static kMIDIPropertyReceivesProgramChanges: CFStringRef;
        pub static kMIDIPropertyReceivesBankSelectMSB: CFStringRef;
        pub static kMIDIPropertyReceivesBankSelectLSB: CFStringRef;
        pub static kMIDIPropertyTransmitsClock: CFStringRef;
        pub static kMIDIPropertyTransmitsMTC: CFStringRef;
        pub static kMIDIPropertyTransmitsNotes: CFStringRef;
        pub static kMIDIPropertyTransmitsProgramChanges: CFStringRef;
        pub static kMIDIPropertyTransmitsBankSelectMSB: CFStringRef;
        pub static kMIDIPropertyTransmitsBankSelectLSB: CFStringRef;
        pub static kMIDIPropertyPanDisruptsStereo: CFStringRef;
        pub static kMIDIPropertyIsSampler: CFStringRef;
        pub static kMIDIPropertyIsDrumMachine: CFStringRef;
        pub static kMIDIPropertyIsMixer: CFStringRef;
        pub static kMIDIPropertyIsEffectUnit: CFStringRef;
        pub static kMIDIPropertyMaxReceiveChannels: CFStringRef;
        pub static kMIDIPropertyMaxTransmitChannels: CFStringRef;
        pub static kMIDIPropertyDriverDeviceEditorApp: CFStringRef;
        pub static kMIDIPropertySupportsShowControl: CFStringRef;
        pub static kMIDIPropertyDisplayName: CFStringRef;
        pub static kMIDIPropertyProtocolID: CFStringRef;
        pub static kMIDIPropertyUMPActiveGroupBitmap: CFStringRef;
        pub static kMIDIPropertyUMPCanTransmitGroupless: CFStringRef;
        pub static kMIDIPropertyAssociatedEndpoint: CFStringRef;
        pub static kMIDIDriverPropertyUsesSerial: CFStringRef;

        pub fn MIDIClientCreate(name: CFStringRef, notifyProc: MIDINotifyProc, notifyRefCon: *mut c_void, outClient: *mut MIDIClientRef) -> OSStatus;
        pub fn MIDIClientCreateWithBlock(name: CFStringRef, outClient: *mut MIDIClientRef, notifyBlock: MIDINotifyBlock) -> OSStatus;
        pub fn MIDIClientDispose(client: MIDIClientRef) -> OSStatus;
        pub fn MIDIInputPortCreateWithProtocol(client: MIDIClientRef, portName: CFStringRef, protocol: MIDIProtocolID, outPort: *mut MIDIPortRef, receiveBlock: MIDIReceiveBlock) -> OSStatus;
        pub fn MIDIInputPortCreate(client: MIDIClientRef, portName: CFStringRef, readProc: MIDIReadProc, refCon: *mut c_void, outPort: *mut MIDIPortRef) -> OSStatus;
        pub fn MIDIInputPortCreateWithBlock(client: MIDIClientRef, portName: CFStringRef, outPort: *mut MIDIPortRef, readBlock: MIDIReadBlock) -> OSStatus;
        pub fn MIDIOutputPortCreate(client: MIDIClientRef, portName: CFStringRef, outPort: *mut MIDIPortRef) -> OSStatus;
        pub fn MIDIPortDispose(port: MIDIPortRef) -> OSStatus;
        pub fn MIDIPortConnectSource(port: MIDIPortRef, source: MIDIEndpointRef, connRefCon: *mut c_void) -> OSStatus;
        pub fn MIDIPortDisconnectSource(port: MIDIPortRef, source: MIDIEndpointRef) -> OSStatus;
        pub fn MIDIGetNumberOfDevices() -> ItemCount;
        pub fn MIDIGetDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
        pub fn MIDIDeviceGetNumberOfEntities(device: MIDIDeviceRef) -> ItemCount;
        pub fn MIDIDeviceGetEntity(device: MIDIDeviceRef, entityIndex0: ItemCount) -> MIDIEntityRef;
        pub fn MIDIEntityGetNumberOfSources(entity: MIDIEntityRef) -> ItemCount;
        pub fn MIDIEntityGetSource(entity: MIDIEntityRef, sourceIndex0: ItemCount) -> MIDIEndpointRef;
        pub fn MIDIEntityGetNumberOfDestinations(entity: MIDIEntityRef) -> ItemCount;
        pub fn MIDIEntityGetDestination(entity: MIDIEntityRef, destIndex0: ItemCount) -> MIDIEndpointRef;
        pub fn MIDIEntityGetDevice(entity: MIDIEntityRef, outDevice: *mut MIDIDeviceRef) -> OSStatus;
        pub fn MIDIGetNumberOfSources() -> ItemCount;
        pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;
        pub fn MIDIGetNumberOfDestinations() -> ItemCount;
        pub fn MIDIGetDestination(destIndex0: ItemCount) -> MIDIEndpointRef;
        pub fn MIDIEndpointGetEntity(endpoint: MIDIEndpointRef, outEntity: *mut MIDIEntityRef) -> OSStatus;
        pub fn MIDIDestinationCreateWithProtocol(client: MIDIClientRef, name: CFStringRef, protocol: MIDIProtocolID, outDest: *mut MIDIEndpointRef, readBlock: MIDIReceiveBlock) -> OSStatus;
        pub fn MIDIDestinationCreate(client: MIDIClientRef, name: CFStringRef, readProc: MIDIReadProc, refCon: *mut c_void, outDest: *mut MIDIEndpointRef) -> OSStatus;
        pub fn MIDIDestinationCreateWithBlock(client: MIDIClientRef, name: CFStringRef, outDest: *mut MIDIEndpointRef, readBlock: MIDIReadBlock) -> OSStatus;
        pub fn MIDISourceCreateWithProtocol(client: MIDIClientRef, name: CFStringRef, protocol: MIDIProtocolID, outSrc: *mut MIDIEndpointRef) -> OSStatus;
        pub fn MIDISourceCreate(client: MIDIClientRef, name: CFStringRef, outSrc: *mut MIDIEndpointRef) -> OSStatus;
        pub fn MIDIEndpointDispose(endpt: MIDIEndpointRef) -> OSStatus;
        pub fn MIDIGetNumberOfExternalDevices() -> ItemCount;
        pub fn MIDIGetExternalDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
        pub fn MIDIObjectGetIntegerProperty(obj: MIDIObjectRef, propertyID: CFStringRef, outValue: *mut i32) -> OSStatus;
        pub fn MIDIObjectSetIntegerProperty(obj: MIDIObjectRef, propertyID: CFStringRef, value: i32) -> OSStatus;
        pub fn MIDIObjectGetStringProperty(obj: MIDIObjectRef, propertyID: CFStringRef, outValue: *mut CFStringRef) -> OSStatus;
        pub fn MIDIObjectSetStringProperty(obj: MIDIObjectRef, propertyID: CFStringRef, value: CFStringRef) -> OSStatus;
        pub fn MIDIObjectGetDataProperty(obj: MIDIObjectRef, propertyID: CFStringRef, outValue: *mut CFDataRef) -> OSStatus;
        pub fn MIDIObjectSetDataProperty(obj: MIDIObjectRef, propertyID: CFStringRef, value: CFDataRef) -> OSStatus;
        pub fn MIDIObjectGetDictionaryProperty(obj: MIDIObjectRef, propertyID: CFStringRef, outValue: *mut CFDictionaryRef) -> OSStatus;
        pub fn MIDIObjectSetDictionaryProperty(obj: MIDIObjectRef, propertyID: CFStringRef, value: CFDictionaryRef) -> OSStatus;
        pub fn MIDIObjectGetProperties(obj: MIDIObjectRef, outProperties: *mut CFPropertyListRef, deep: Boolean) -> OSStatus;
        pub fn MIDIObjectRemoveProperty(obj: MIDIObjectRef, propertyID: CFStringRef) -> OSStatus;
        pub fn MIDIObjectFindByUniqueID(inUniqueID: MIDIUniqueID, outObject: *mut MIDIObjectRef, outObjectType: *mut MIDIObjectType) -> OSStatus;
        pub fn MIDISendEventList(port: MIDIPortRef, dest: MIDIEndpointRef, evtlist: *const MIDIEventList) -> OSStatus;
        pub fn MIDISend(port: MIDIPortRef, dest: MIDIEndpointRef, pktlist: *const MIDIPacketList) -> OSStatus;
        pub fn MIDISendSysex(request: *mut MIDISysexSendRequest) -> OSStatus;
        pub fn MIDISendUMPSysex(request: *mut MIDISysexSendRequestUMP) -> OSStatus;
        pub fn MIDISendUMPSysex8(request: *mut MIDISysexSendRequestUMP) -> OSStatus;
        pub fn MIDIEventPacketSysexBytesForGroup(packet: *const MIDIEventPacket, groupIndex: u8, outData: *mut CFDataRef) -> OSStatus;
        pub fn MIDIReceivedEventList(src: MIDIEndpointRef, evtlist: *const MIDIEventList) -> OSStatus;
        pub fn MIDIReceived(src: MIDIEndpointRef, pktlist: *const MIDIPacketList) -> OSStatus;
        pub fn MIDIFlushOutput(dest: MIDIEndpointRef) -> OSStatus;
        pub fn MIDIRestart() -> OSStatus;
        pub fn MIDIEventListInit(evtlist: *mut MIDIEventList, protocol: MIDIProtocolID) -> *mut MIDIEventPacket;
        pub fn MIDIEventListAdd(evtlist: *mut MIDIEventList, listSize: ByteCount, curPacket: *mut MIDIEventPacket, time: MIDITimeStamp, wordCount: ByteCount, words: *const u32) -> *mut MIDIEventPacket;
        pub fn MIDIPacketListInit(pktlist: *mut MIDIPacketList) -> *mut MIDIPacket;
        pub fn MIDIPacketListAdd(pktlist: *mut MIDIPacketList, listSize: ByteCount, curPacket: *mut MIDIPacket, time: MIDITimeStamp, nData: ByteCount, data: *const Byte) -> *mut MIDIPacket;
        pub fn MIDIDeviceNewEntity(device: MIDIDeviceRef, name: CFStringRef, protocol: MIDIProtocolID, embedded: Boolean, numSourceEndpoints: ItemCount, numDestinationEndpoints: ItemCount, newEntity: *mut MIDIEntityRef) -> OSStatus;
        pub fn MIDIDeviceAddEntity(device: MIDIDeviceRef, name: CFStringRef, embedded: Boolean, numSourceEndpoints: ItemCount, numDestinationEndpoints: ItemCount, newEntity: *mut MIDIEntityRef) -> OSStatus;
        pub fn MIDIDeviceRemoveEntity(device: MIDIDeviceRef, entity: MIDIEntityRef) -> OSStatus;
        pub fn MIDIEntityAddOrRemoveEndpoints(entity: MIDIEntityRef, numSourceEndpoints: ItemCount, numDestinationEndpoints: ItemCount) -> OSStatus;
        pub fn MIDISetupAddDevice(device: MIDIDeviceRef) -> OSStatus;
        pub fn MIDISetupRemoveDevice(device: MIDIDeviceRef) -> OSStatus;
        pub fn MIDISetupAddExternalDevice(device: MIDIDeviceRef) -> OSStatus;
        pub fn MIDISetupRemoveExternalDevice(device: MIDIDeviceRef) -> OSStatus;
        pub fn MIDIGetSerialPortOwner(portName: CFStringRef, outDriverName: *mut CFStringRef) -> OSStatus;
        pub fn MIDISetSerialPortOwner(portName: CFStringRef, driverName: CFStringRef) -> OSStatus;
        pub fn MIDIGetSerialPortDrivers(outDriverNames: *mut CFArrayRef) -> OSStatus;
        pub fn MIDIExternalDeviceCreate(name: CFStringRef, manufacturer: CFStringRef, model: CFStringRef, outDevice: *mut MIDIDeviceRef) -> OSStatus;
        pub fn MIDISetupCreate(outSetup: *mut MIDISetupRef) -> OSStatus;
        pub fn MIDISetupDispose(setup: MIDISetupRef) -> OSStatus;
        pub fn MIDISetupInstall(setup: MIDISetupRef) -> OSStatus;
        pub fn MIDISetupGetCurrent(outSetup: *mut MIDISetupRef) -> OSStatus;
        pub fn MIDISetupToData(setup: MIDISetupRef, outData: *mut CFDataRef) -> OSStatus;
        pub fn MIDISetupFromData(data: CFDataRef, outSetup: *mut MIDISetupRef) -> OSStatus;
        pub fn MIDIBluetoothDriverActivateAllConnections() -> OSStatus;
        pub fn MIDIBluetoothDriverDisconnect(uuid: CFStringRef) -> OSStatus;
        pub fn MIDIDeviceCreate(owner: MIDIDriverRef, name: CFStringRef, manufacturer: CFStringRef, model: CFStringRef, outDevice: *mut MIDIDeviceRef) -> OSStatus;
        pub fn MIDIDeviceDispose(device: MIDIDeviceRef) -> OSStatus;
        pub fn MIDIDeviceListGetNumberOfDevices(devList: MIDIDeviceListRef) -> ItemCount;
        pub fn MIDIDeviceListGetDevice(devList: MIDIDeviceListRef, index0: ItemCount) -> MIDIDeviceRef;
        pub fn MIDIDeviceListAddDevice(devList: MIDIDeviceListRef, dev: MIDIDeviceRef) -> OSStatus;
        pub fn MIDIDeviceListDispose(devList: MIDIDeviceListRef) -> OSStatus;
        pub fn MIDIEndpointSetRefCons(endpt: MIDIEndpointRef, ref1: *mut c_void, ref2: *mut c_void) -> OSStatus;
        pub fn MIDIEndpointGetRefCons(endpt: MIDIEndpointRef, ref1: *mut *mut c_void, ref2: *mut *mut c_void) -> OSStatus;
        pub fn MIDIGetDriverIORunLoop() -> CFRunLoopRef;
        pub fn MIDIGetDriverDeviceList(driver: MIDIDriverRef) -> MIDIDeviceListRef;
        pub fn MIDIDriverEnableMonitoring(driver: MIDIDriverRef, enabled: Boolean) -> OSStatus;
        pub fn MIDIThruConnectionParamsInitialize(inConnectionParams: *mut MIDIThruConnectionParams);
        pub fn MIDIThruConnectionCreate(inPersistentOwnerID: CFStringRef, inConnectionParams: CFDataRef, outConnection: *mut MIDIThruConnectionRef) -> OSStatus;
        pub fn MIDIThruConnectionDispose(connection: MIDIThruConnectionRef) -> OSStatus;
        pub fn MIDIThruConnectionGetParams(connection: MIDIThruConnectionRef, outConnectionParams: *mut CFDataRef) -> OSStatus;
        pub fn MIDIThruConnectionSetParams(connection: MIDIThruConnectionRef, inConnectionParams: CFDataRef) -> OSStatus;
        pub fn MIDIThruConnectionFind(inPersistentOwnerID: CFStringRef, outConnectionList: *mut CFDataRef) -> OSStatus;
    }
}

#[cfg(feature = "raw-ffi")]
pub use raw_coremidi::*;
#[cfg(not(feature = "raw-ffi"))]
pub(crate) use raw_coremidi::*;

mod raw_corefoundation {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        pub static kCFAllocatorDefault: CFAllocatorRef;
        pub fn CFRelease(cf: CFTypeRef);
        pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
        pub fn CFStringCreateWithCString(alloc: CFAllocatorRef, c_str: *const c_char, encoding: u32) -> CFStringRef;
        pub fn CFStringGetCString(string: CFStringRef, buffer: *mut c_char, bufferSize: CFIndex, encoding: u32) -> bool;
        pub fn CFStringGetLength(string: CFStringRef) -> CFIndex;
        pub fn CFStringGetMaximumSizeForEncoding(length: CFIndex, encoding: u32) -> CFIndex;
        pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
        pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
        pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
        pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    }
}

#[cfg(feature = "raw-ffi")]
pub use raw_corefoundation::*;
#[cfg(not(feature = "raw-ffi"))]
pub(crate) use raw_corefoundation::*;

#[must_use]
pub unsafe fn MIDIEventPacketNext(packet: *const MIDIEventPacket) -> *const MIDIEventPacket {
    core::ptr::addr_of!((*packet).words)
        .cast::<u32>()
        .add((*packet).wordCount as usize)
        .cast()
}

#[must_use]
pub unsafe fn MIDIPacketNext(packet: *const MIDIPacket) -> *const MIDIPacket {
    let data_end = core::ptr::addr_of!((*packet).data)
        .cast::<u8>()
        .add((*packet).length as usize) as usize;
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let next = (data_end + 3) & !3;
    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    let next = data_end;
    next as *const MIDIPacket
}

#[must_use]
pub const fn MIDIThruConnectionParamsSize(params: &MIDIThruConnectionParams) -> usize {
    core::mem::size_of::<MIDIThruConnectionParams>()
        + params.numControlTransforms as usize * core::mem::size_of::<MIDIControlTransform>()
        + params.numMaps as usize * core::mem::size_of::<MIDIValueMap>()
}
