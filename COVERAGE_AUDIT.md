# coremidi-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 235
VERIFIED: 220
GAPS: 1
EXEMPT: 14
COVERAGE_PCT: 99.57%

- Generated against MacOSX26.2.sdk and not regenerated against the installed 26.5 SDK.
- Counts cover top-level public CoreMIDI declarations (types, exported constants, Obj-C interfaces/protocols, and non-inline functions), not per-method Obj-C coverage. Obj-C interfaces count as VERIFIED when the crate has a snapshot or handle type for them, even if it wraps only part of their methods.
- VERIFIED means the symbol is declared in `coremidi::ffi` or used by a Rust wrapper. 154 of the 220 VERIFIED rows name only a raw `ffi::<symbol>` declaration (public only with the `raw-ffi` feature); many of those back safe wrappers, but the row does not claim one.
- COVERAGE_PCT is (VERIFIED + EXEMPT) / SDK_PUBLIC_SYMBOLS. EXEMPT rows are symbols the crate does not wrap safely.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MIDIBluetoothDriverActivateAllConnections` | function | `MIDIBluetoothConnection.h` | `ffi::MIDIBluetoothDriverActivateAllConnections` |
| `MIDIBluetoothDriverDisconnect` | function | `MIDIBluetoothConnection.h` | `ffi::MIDIBluetoothDriverDisconnect` |
| `MIDICIDevice` | interface | `MIDICIDevice.h` | `capability::discovered_ci_devices / capability::CiDeviceInfo` |
| `MIDICIDeviceManager` | interface | `MIDICIDeviceManager.h` | `capability::discovered_ci_devices / capability::ci_device_manager_constants` |
| `MIDICIDeviceObjectKey` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::device_object_key` |
| `MIDICIDeviceWasAddedNotification` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::device_added_notification` |
| `MIDICIDeviceWasRemovedNotification` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::device_removed_notification` |
| `MIDICIProfile` | interface | `MIDICapabilityInquiry.h` | `capability::legacy_ci_profile / capability::LegacyCiProfileInfo` |
| `MIDICIProfileObjectKey` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::profile_object_key` |
| `MIDICIProfileState` | interface | `MIDICapabilityInquiry.h` | `capability::CiProfileState / capability::CiProfileStateInfo` |
| `MIDICIProfileWasRemovedNotification` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::profile_removed_notification` |
| `MIDICIProfileWasUpdatedNotification` | const | `MIDICIDeviceManager.h` | `capability::ci_device_manager_constants / capability::CiDeviceManagerConstants::profile_updated_notification` |
| `MIDIDeviceCreate` | function | `MIDIDriver.h` | `ffi::MIDIDeviceCreate` |
| `MIDIDeviceDispose` | function | `MIDIDriver.h` | `ffi::MIDIDeviceDispose` |
| `MIDIDeviceListAddDevice` | function | `MIDIDriver.h` | `ffi::MIDIDeviceListAddDevice` |
| `MIDIDeviceListDispose` | function | `MIDIDriver.h` | `ffi::MIDIDeviceListDispose` |
| `MIDIDeviceListGetDevice` | function | `MIDIDriver.h` | `ffi::MIDIDeviceListGetDevice` |
| `MIDIDeviceListGetNumberOfDevices` | function | `MIDIDriver.h` | `ffi::MIDIDeviceListGetNumberOfDevices` |
| `MIDIDriverEnableMonitoring` | function | `MIDIDriver.h` | `ffi::MIDIDriverEnableMonitoring` |
| `MIDIDriverInterface` | struct | `MIDIDriver.h` | `ffi::MIDIDriverInterface` |
| `MIDIEndpointGetRefCons` | function | `MIDIDriver.h` | `ffi::MIDIEndpointGetRefCons` |
| `MIDIEndpointSetRefCons` | function | `MIDIDriver.h` | `ffi::MIDIEndpointSetRefCons` |
| `MIDIGetDriverDeviceList` | function | `MIDIDriver.h` | `ffi::MIDIGetDriverDeviceList` |
| `MIDIGetDriverIORunLoop` | function | `MIDIDriver.h` | `ffi::MIDIGetDriverIORunLoop` |
| `kMIDIDriverPropertyUsesSerial` | const | `MIDIDriver.h` | `ffi::kMIDIDriverPropertyUsesSerial` |
| `MIDICVStatus` | enum | `MIDIMessages.h` | `packet::MidiCvStatus` |
| `MIDIMessageType` | enum | `MIDIMessages.h` | `packet::MidiMessageType` |
| `MIDIMessage_128` | struct | `MIDIMessages.h` | `packet::MidiMessage128 / ffi::MIDIMessage_128` |
| `MIDIMessage_64` | struct | `MIDIMessages.h` | `packet::MidiMessage64 / ffi::MIDIMessage_64` |
| `MIDIMessage_96` | struct | `MIDIMessages.h` | `packet::MidiMessage96 / ffi::MIDIMessage_96` |
| `MIDINoteAttribute` | enum | `MIDIMessages.h` | `packet::MidiNoteAttribute` |
| `MIDIPerNoteManagementOptions` | enum | `MIDIMessages.h` | `packet::MidiPerNoteManagementOptions` |
| `MIDIProgramChangeOptions` | enum | `MIDIMessages.h` | `packet::MidiProgramChangeOptions` |
| `MIDISysExStatus` | enum | `MIDIMessages.h` | `packet::MidiSysExStatus` |
| `MIDISystemStatus` | enum | `MIDIMessages.h` | `packet::MidiSystemStatus` |
| `MIDIUMPFunctionBlockDirection` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockDirection` |
| `MIDIUMPFunctionBlockMIDI1Info` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockMIDI1Info` |
| `MIDIUMPFunctionBlockUIHint` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockUIHint` |
| `MIDIUtilityStatus` | enum | `MIDIMessages.h` | `packet::MidiUtilityStatus` |
| `UMPStreamMessageFormat` | enum | `MIDIMessages.h` | `packet::UmpStreamMessageFormat` |
| `UMPStreamMessageStatus` | enum | `MIDIMessages.h` | `packet::UmpStreamMessageStatus` |
| `MIDINetworkBonjourServiceType` | const | `MIDINetworkSession.h` | `network::NetworkConstants::bonjour_service_type` |
| `MIDINetworkConnection` | interface | `MIDINetworkSession.h` | `network::NetworkConnection` |
| `MIDINetworkConnectionPolicy` | enum | `MIDINetworkSession.h` | `network::NetworkConnectionPolicy` |
| `MIDINetworkHost` | interface | `MIDINetworkSession.h` | `network::NetworkHost` |
| `MIDINetworkNotificationContactsDidChange` | const | `MIDINetworkSession.h` | `network::NetworkConstants::contacts_changed_notification` |
| `MIDINetworkNotificationSessionDidChange` | const | `MIDINetworkSession.h` | `network::NetworkConstants::session_changed_notification` |
| `MIDINetworkSession` | interface | `MIDINetworkSession.h` | `network::NetworkSession` |
| `MIDIClientCreate` | function | `MIDIServices.h` | `ffi::MIDIClientCreate` |
| `MIDIClientCreateWithBlock` | function | `MIDIServices.h` | `ffi::MIDIClientCreateWithBlock` |
| `MIDIClientDispose` | function | `MIDIServices.h` | `ffi::MIDIClientDispose` |
| `MIDIDestinationCreateWithProtocol` | function | `MIDIServices.h` | `ffi::MIDIDestinationCreateWithProtocol` |
| `MIDIDeviceGetEntity` | function | `MIDIServices.h` | `ffi::MIDIDeviceGetEntity` |
| `MIDIDeviceGetNumberOfEntities` | function | `MIDIServices.h` | `ffi::MIDIDeviceGetNumberOfEntities` |
| `MIDIEndpointDispose` | function | `MIDIServices.h` | `ffi::MIDIEndpointDispose` |
| `MIDIEndpointGetEntity` | function | `MIDIServices.h` | `ffi::MIDIEndpointGetEntity` |
| `MIDIEntityGetDestination` | function | `MIDIServices.h` | `ffi::MIDIEntityGetDestination` |
| `MIDIEntityGetDevice` | function | `MIDIServices.h` | `ffi::MIDIEntityGetDevice` |
| `MIDIEntityGetNumberOfDestinations` | function | `MIDIServices.h` | `ffi::MIDIEntityGetNumberOfDestinations` |
| `MIDIEntityGetNumberOfSources` | function | `MIDIServices.h` | `ffi::MIDIEntityGetNumberOfSources` |
| `MIDIEntityGetSource` | function | `MIDIServices.h` | `ffi::MIDIEntityGetSource` |
| `MIDIEventList` | struct | `MIDIServices.h` | `ffi::MIDIEventList` |
| `MIDIEventListAdd` | function | `MIDIServices.h` | `ffi::MIDIEventListAdd` |
| `MIDIEventListInit` | function | `MIDIServices.h` | `ffi::MIDIEventListInit` |
| `MIDIEventPacket` | struct | `MIDIServices.h` | `ffi::MIDIEventPacket` |
| `MIDIEventPacketSysexBytesForGroup` | function | `MIDIServices.h` | `ffi::MIDIEventPacketSysexBytesForGroup` |
| `MIDIFlushOutput` | function | `MIDIServices.h` | `ffi::MIDIFlushOutput` |
| `MIDIGetDestination` | function | `MIDIServices.h` | `ffi::MIDIGetDestination` |
| `MIDIGetDevice` | function | `MIDIServices.h` | `ffi::MIDIGetDevice` |
| `MIDIGetExternalDevice` | function | `MIDIServices.h` | `ffi::MIDIGetExternalDevice` |
| `MIDIGetNumberOfDestinations` | function | `MIDIServices.h` | `ffi::MIDIGetNumberOfDestinations` |
| `MIDIGetNumberOfDevices` | function | `MIDIServices.h` | `ffi::MIDIGetNumberOfDevices` |
| `MIDIGetNumberOfExternalDevices` | function | `MIDIServices.h` | `ffi::MIDIGetNumberOfExternalDevices` |
| `MIDIGetNumberOfSources` | function | `MIDIServices.h` | `ffi::MIDIGetNumberOfSources` |
| `MIDIGetSource` | function | `MIDIServices.h` | `ffi::MIDIGetSource` |
| `MIDIIOErrorNotification` | struct | `MIDIServices.h` | `ffi::MIDIIOErrorNotification` |
| `MIDIInputPortCreateWithProtocol` | function | `MIDIServices.h` | `ffi::MIDIInputPortCreateWithProtocol` |
| `MIDINotification` | struct | `MIDIServices.h` | `ffi::MIDINotification` |
| `MIDINotificationMessageID` | enum | `MIDIServices.h` | `notification::NotificationMessageId` |
| `MIDIObjectAddRemoveNotification` | struct | `MIDIServices.h` | `ffi::MIDIObjectAddRemoveNotification` |
| `MIDIObjectFindByUniqueID` | function | `MIDIServices.h` | `ffi::MIDIObjectFindByUniqueID` |
| `MIDIObjectGetDataProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectGetDataProperty` |
| `MIDIObjectGetDictionaryProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectGetDictionaryProperty` |
| `MIDIObjectGetIntegerProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectGetIntegerProperty` |
| `MIDIObjectGetProperties` | function | `MIDIServices.h` | `ffi::MIDIObjectGetProperties` |
| `MIDIObjectGetStringProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectGetStringProperty` |
| `MIDIObjectPropertyChangeNotification` | struct | `MIDIServices.h` | `ffi::MIDIObjectPropertyChangeNotification` |
| `MIDIObjectRemoveProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectRemoveProperty` |
| `MIDIObjectSetDataProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectSetDataProperty` |
| `MIDIObjectSetDictionaryProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectSetDictionaryProperty` |
| `MIDIObjectSetIntegerProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectSetIntegerProperty` |
| `MIDIObjectSetStringProperty` | function | `MIDIServices.h` | `ffi::MIDIObjectSetStringProperty` |
| `MIDIObjectType` | enum | `MIDIServices.h` | `ffi::MIDIObjectType` |
| `MIDIOutputPortCreate` | function | `MIDIServices.h` | `ffi::MIDIOutputPortCreate` |
| `MIDIPacket` | struct | `MIDIServices.h` | `ffi::MIDIPacket` |
| `MIDIPacketList` | struct | `MIDIServices.h` | `ffi::MIDIPacketList` |
| `MIDIPortConnectSource` | function | `MIDIServices.h` | `ffi::MIDIPortConnectSource` |
| `MIDIPortDisconnectSource` | function | `MIDIServices.h` | `ffi::MIDIPortDisconnectSource` |
| `MIDIPortDispose` | function | `MIDIServices.h` | `ffi::MIDIPortDispose` |
| `MIDIProtocolID` | enum | `MIDIServices.h` | `ffi::MIDIProtocolID` |
| `MIDIReceivedEventList` | function | `MIDIServices.h` | `ffi::MIDIReceivedEventList` |
| `MIDIRestart` | function | `MIDIServices.h` | `ffi::MIDIRestart` |
| `MIDISendEventList` | function | `MIDIServices.h` | `ffi::MIDISendEventList` |
| `MIDISendSysex` | function | `MIDIServices.h` | `ffi::MIDISendSysex` |
| `MIDISendUMPSysex` | function | `MIDIServices.h` | `ffi::MIDISendUMPSysex` |
| `MIDISendUMPSysex8` | function | `MIDIServices.h` | `ffi::MIDISendUMPSysex8` |
| `MIDISourceCreateWithProtocol` | function | `MIDIServices.h` | `ffi::MIDISourceCreateWithProtocol` |
| `MIDISysexSendRequest` | struct | `MIDIServices.h` | `ffi::MIDISysexSendRequest` |
| `MIDISysexSendRequestUMP` | struct | `MIDIServices.h` | `ffi::MIDISysexSendRequestUMP` |
| `kMIDIPropertyAdvanceScheduleTimeMuSec` | const | `MIDIServices.h` | `ffi::kMIDIPropertyAdvanceScheduleTimeMuSec` |
| `kMIDIPropertyAssociatedEndpoint` | const | `MIDIServices.h` | `ffi::kMIDIPropertyAssociatedEndpoint` |
| `kMIDIPropertyCanRoute` | const | `MIDIServices.h` | `ffi::kMIDIPropertyCanRoute` |
| `kMIDIPropertyConnectionUniqueID` | const | `MIDIServices.h` | `ffi::kMIDIPropertyConnectionUniqueID` |
| `kMIDIPropertyDeviceID` | const | `MIDIServices.h` | `ffi::kMIDIPropertyDeviceID` |
| `kMIDIPropertyDisplayName` | const | `MIDIServices.h` | `ffi::kMIDIPropertyDisplayName` |
| `kMIDIPropertyDriverDeviceEditorApp` | const | `MIDIServices.h` | `ffi::kMIDIPropertyDriverDeviceEditorApp` |
| `kMIDIPropertyDriverOwner` | const | `MIDIServices.h` | `ffi::kMIDIPropertyDriverOwner` |
| `kMIDIPropertyDriverVersion` | const | `MIDIServices.h` | `ffi::kMIDIPropertyDriverVersion` |
| `kMIDIPropertyImage` | const | `MIDIServices.h` | `ffi::kMIDIPropertyImage` |
| `kMIDIPropertyIsBroadcast` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsBroadcast` |
| `kMIDIPropertyIsDrumMachine` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsDrumMachine` |
| `kMIDIPropertyIsEffectUnit` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsEffectUnit` |
| `kMIDIPropertyIsEmbeddedEntity` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsEmbeddedEntity` |
| `kMIDIPropertyIsMixer` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsMixer` |
| `kMIDIPropertyIsSampler` | const | `MIDIServices.h` | `ffi::kMIDIPropertyIsSampler` |
| `kMIDIPropertyManufacturer` | const | `MIDIServices.h` | `ffi::kMIDIPropertyManufacturer` |
| `kMIDIPropertyMaxReceiveChannels` | const | `MIDIServices.h` | `ffi::kMIDIPropertyMaxReceiveChannels` |
| `kMIDIPropertyMaxSysExSpeed` | const | `MIDIServices.h` | `ffi::kMIDIPropertyMaxSysExSpeed` |
| `kMIDIPropertyMaxTransmitChannels` | const | `MIDIServices.h` | `ffi::kMIDIPropertyMaxTransmitChannels` |
| `kMIDIPropertyModel` | const | `MIDIServices.h` | `ffi::kMIDIPropertyModel` |
| `kMIDIPropertyName` | const | `MIDIServices.h` | `ffi::kMIDIPropertyName` |
| `kMIDIPropertyNameConfigurationDictionary` | const | `MIDIServices.h` | `ffi::kMIDIPropertyNameConfigurationDictionary` |
| `kMIDIPropertyOffline` | const | `MIDIServices.h` | `ffi::kMIDIPropertyOffline` |
| `kMIDIPropertyPanDisruptsStereo` | const | `MIDIServices.h` | `ffi::kMIDIPropertyPanDisruptsStereo` |
| `kMIDIPropertyPrivate` | const | `MIDIServices.h` | `ffi::kMIDIPropertyPrivate` |
| `kMIDIPropertyProtocolID` | const | `MIDIServices.h` | `ffi::kMIDIPropertyProtocolID` |
| `kMIDIPropertyReceiveChannels` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceiveChannels` |
| `kMIDIPropertyReceivesBankSelectLSB` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesBankSelectLSB` |
| `kMIDIPropertyReceivesBankSelectMSB` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesBankSelectMSB` |
| `kMIDIPropertyReceivesClock` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesClock` |
| `kMIDIPropertyReceivesMTC` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesMTC` |
| `kMIDIPropertyReceivesNotes` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesNotes` |
| `kMIDIPropertyReceivesProgramChanges` | const | `MIDIServices.h` | `ffi::kMIDIPropertyReceivesProgramChanges` |
| `kMIDIPropertySingleRealtimeEntity` | const | `MIDIServices.h` | `ffi::kMIDIPropertySingleRealtimeEntity` |
| `kMIDIPropertySupportsGeneralMIDI` | const | `MIDIServices.h` | `ffi::kMIDIPropertySupportsGeneralMIDI` |
| `kMIDIPropertySupportsMMC` | const | `MIDIServices.h` | `ffi::kMIDIPropertySupportsMMC` |
| `kMIDIPropertySupportsShowControl` | const | `MIDIServices.h` | `ffi::kMIDIPropertySupportsShowControl` |
| `kMIDIPropertyTransmitChannels` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitChannels` |
| `kMIDIPropertyTransmitsBankSelectLSB` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsBankSelectLSB` |
| `kMIDIPropertyTransmitsBankSelectMSB` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsBankSelectMSB` |
| `kMIDIPropertyTransmitsClock` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsClock` |
| `kMIDIPropertyTransmitsMTC` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsMTC` |
| `kMIDIPropertyTransmitsNotes` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsNotes` |
| `kMIDIPropertyTransmitsProgramChanges` | const | `MIDIServices.h` | `ffi::kMIDIPropertyTransmitsProgramChanges` |
| `kMIDIPropertyUMPActiveGroupBitmap` | const | `MIDIServices.h` | `ffi::kMIDIPropertyUMPActiveGroupBitmap` |
| `kMIDIPropertyUMPCanTransmitGroupless` | const | `MIDIServices.h` | `ffi::kMIDIPropertyUMPCanTransmitGroupless` |
| `kMIDIPropertyUniqueID` | const | `MIDIServices.h` | `ffi::kMIDIPropertyUniqueID` |
| `MIDIDeviceNewEntity` | function | `MIDISetup.h` | `ffi::MIDIDeviceNewEntity` |
| `MIDIDeviceRemoveEntity` | function | `MIDISetup.h` | `ffi::MIDIDeviceRemoveEntity` |
| `MIDIEntityAddOrRemoveEndpoints` | function | `MIDISetup.h` | `ffi::MIDIEntityAddOrRemoveEndpoints` |
| `MIDIExternalDeviceCreate` | function | `MIDISetup.h` | `ffi::MIDIExternalDeviceCreate` |
| `MIDISetupAddDevice` | function | `MIDISetup.h` | `ffi::MIDISetupAddDevice` |
| `MIDISetupAddExternalDevice` | function | `MIDISetup.h` | `ffi::MIDISetupAddExternalDevice` |
| `MIDISetupRemoveDevice` | function | `MIDISetup.h` | `ffi::MIDISetupRemoveDevice` |
| `MIDISetupRemoveExternalDevice` | function | `MIDISetup.h` | `ffi::MIDISetupRemoveExternalDevice` |
| `MIDIControlTransform` | struct | `MIDIThruConnection.h` | `ffi::MIDIControlTransform` |
| `MIDIThruConnectionCreate` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionCreate` |
| `MIDIThruConnectionDispose` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionDispose` |
| `MIDIThruConnectionEndpoint` | struct | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionEndpoint` |
| `MIDIThruConnectionFind` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionFind` |
| `MIDIThruConnectionGetParams` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionGetParams` |
| `MIDIThruConnectionParams` | struct | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionParams` |
| `MIDIThruConnectionParamsInitialize` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionParamsInitialize` |
| `MIDIThruConnectionSetParams` | function | `MIDIThruConnection.h` | `ffi::MIDIThruConnectionSetParams` |
| `MIDITransform` | struct | `MIDIThruConnection.h` | `ffi::MIDITransform` |
| `MIDITransformControlType` | enum | `MIDIThruConnection.h` | `thru_connection::MidiControlType` |
| `MIDITransformType` | enum | `MIDIThruConnection.h` | `thru_connection::MidiTransformKind` |
| `MIDIValueMap` | struct | `MIDIThruConnection.h` | `ffi::MIDIValueMap` |
| `MIDI2DeviceManufacturer` | struct | `MIDIUMPCI.h` | `ffi::MIDI2DeviceManufacturer` |
| `MIDI2DeviceRevisionLevel` | struct | `MIDIUMPCI.h` | `ffi::MIDI2DeviceRevisionLevel` |
| `MIDICICategoryOptions` | enum | `MIDIUMPCI.h` | `capability::CiDeviceInfo::{supports_protocol_negotiation, supports_profile_configuration, supports_property_exchange, supports_process_inquiry}` |
| `MIDICIDeviceType` | enum | `MIDIUMPCI.h` | `ffi::MIDICIDeviceType` |
| `MIDICIManagementMessageType` | enum | `MIDIUMPCI.h` | `capability::CiManagementMessageType` |
| `MIDICIProcessInquiryMessageType` | enum | `MIDIUMPCI.h` | `capability::CiProcessInquiryMessageType` |
| `MIDICIProfileID` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileID` |
| `MIDICIProfileIDManufacturerSpecific` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileIDManufacturerSpecific` |
| `MIDICIProfileIDStandard` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileIDStandard` |
| `MIDICIProfileMessageType` | enum | `MIDIUMPCI.h` | `capability::CiProfileMessageType` |
| `MIDICIProfileType` | enum | `MIDIUMPCI.h` | `ffi::MIDICIProfileType` |
| `MIDICIPropertyExchangeMessageType` | enum | `MIDIUMPCI.h` | `capability::CiPropertyExchangeMessageType` |
| `MIDIUMPCIObjectBackingType` | enum | `MIDIUMPCI.h` | `ffi::MIDIUMPCIObjectBackingType` |
| `MIDIUMPCIProfile` | interface | `MIDIUMPCIProfile.h` | `capability::CiProfileInfo` |
| `MIDI2DeviceInfo` | interface | `MIDIUMPEndpoint.h` | `endpoint::Midi2DeviceInfo / endpoint::Midi2DeviceInfoHandle` |
| `MIDIUMPEndpoint` | interface | `MIDIUMPEndpoint.h` | `endpoint::UmpEndpointInfo / endpoint::UmpEndpointManager` |
| `MIDIUMPProtocolOptions` | enum | `MIDIUMPEndpoint.h` | `ffi::MIDIUMPProtocolOptions` |
| `MIDIUMPEndpointManager` | interface | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::{endpoints, constants}` |
| `MIDIUMPEndpointObjectKey` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::endpoint_object_key` |
| `MIDIUMPEndpointWasAddedNotification` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::endpoint_added_notification` |
| `MIDIUMPEndpointWasRemovedNotification` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::endpoint_removed_notification` |
| `MIDIUMPEndpointWasUpdatedNotification` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::endpoint_updated_notification` |
| `MIDIUMPFunctionBlock` | interface | `MIDIUMPFunctionBlock.h` | `endpoint::UmpFunctionBlockInfo` |
| `MIDIUMPFunctionBlockObjectKey` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::function_block_object_key` |
| `MIDIUMPFunctionBlockWasUpdatedNotification` | const | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager::constants / endpoint::UmpEndpointManagerConstants::function_block_updated_notification` |
| `MIDIUMPMutableEndpoint` | interface | `MIDIUMPMutableEndpoint.h` | `endpoint::MutableUmpEndpoint` |
| `MIDIUMPMutableFunctionBlock` | interface | `MIDIUMPMutableFunctionBlock.h` | `endpoint::MutableUmpFunctionBlock` |
| `MIDIDestinationCreate` | function | `MIDIServices.h` | `client::MidiClient::virtual_destination` (unsafe, raw `MIDIReadProc`) |
| `MIDIInputPortCreate` | function | `MIDIServices.h` | `client::MidiClient::input_port` (unsafe, raw `MIDIReadProc`) |
| `MIDIPacketListAdd` | function | `MIDIServices.h` | `packet::PacketListBuffer::add_packet` |
| `MIDIPacketListInit` | function | `MIDIServices.h` | `packet::PacketListBuffer::with_capacity / clear` |
| `MIDIReceived` | function | `MIDIServices.h` | `endpoint::VirtualSource::received` |
| `MIDISend` | function | `MIDIServices.h` | `port::MidiOutputPort::send` |
| `MIDISourceCreate` | function | `MIDIServices.h` | `client::MidiClient::virtual_source` |
| `kMIDIPropertyFactoryPatchNameFile` | const | `MIDIServices.h` | `property::MidiProperty::factory_patch_name_file` |
| `kMIDIPropertyNameConfiguration` | const | `MIDIServices.h` | `property::MidiProperty::name_configuration` |
| `kMIDIPropertyUserPatchNameFile` | const | `MIDIServices.h` | `property::MidiProperty::user_patch_name_file` |
| `MIDIDeviceAddEntity` | function | `MIDISetup.h` | `setup::device_add_entity_deprecated` (`#[deprecated]`) |
| `MIDIGetSerialPortDrivers` | function | `MIDISetup.h` | `setup::serial_port_drivers` (`#[deprecated]`; returns -4 on current macOS) |
| `MIDIGetSerialPortOwner` | function | `MIDISetup.h` | `setup::serial_port_owner` (`#[deprecated]`; returns -4 on current macOS) |
| `MIDISetupGetCurrent` | function | `MIDISetup.h` | `setup::current_setup_xml` (`#[deprecated]`; returns -4 on current macOS) |
| `MIDISetupToData` | function | `MIDISetup.h` | `setup::current_setup_xml` (`#[deprecated]`; returns -4 on current macOS) |

## 🔴 GAPS
| Symbol | Kind | Header | Status |
| --- | --- | --- | --- |
| `MIDIEventListForEachEvent` | function | `MIDIMessages.h` | not wrapped: `packet::EventIter` walks packets and exposes raw UMP words, but nothing decodes them into `MIDIUniversalMessage` |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MIDICIDeviceIdentification` | struct | `MIDICapabilityInquiry.h` | legacy MIDI-CI 1.0 identification struct in deprecated header section | `MIDICapabilityInquiry.h legacy/deprecated MIDI-CI header` |
| `MIDICIDeviceInfo` | interface | `MIDICapabilityInquiry.h` | legacy MIDI-CI 1.0 class intentionally skipped | `MIDICI1_0_DEPRECATED` |
| `MIDICIDiscoveredNode` | interface | `MIDICapabilityInquiry.h` | legacy MIDI-CI discovery class intentionally skipped | `MIDICI1_1_DEPRECATED` |
| `MIDICIDiscoveryManager` | interface | `MIDICapabilityInquiry.h` | legacy MIDI-CI discovery singleton intentionally skipped | `MIDICI1_1_DEPRECATED` |
| `MIDICIProfileResponderDelegate` | protocol | `MIDICapabilityInquiry.h` | legacy MIDI-CI responder protocol intentionally skipped | `MIDICI1_1_DEPRECATED` |
| `MIDICIResponder` | interface | `MIDICapabilityInquiry.h` | legacy MIDI-CI responder class intentionally skipped | `MIDICI1_1_DEPRECATED` |
| `MIDICISession` | interface | `MIDICapabilityInquiry.h` | legacy MIDI-CI session class intentionally skipped | `MIDICI1_0_DEPRECATED` |
| `MIDIDestinationCreateWithBlock` | function | `MIDIServices.h` | legacy packet-list virtual destination constructor intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED_WITH_REPLACEMENT("MIDIDestinationCreateWithProtocol", macos(10.11, API_TO_BE_DEPRECATED))` |
| `MIDIInputPortCreateWithBlock` | function | `MIDIServices.h` | legacy packet-list input-port constructor intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED_WITH_REPLACEMENT("MIDIInputPortCreateWithProtocol", macos(10.11, API_TO_BE_DEPRECATED))` |
| `MIDISetSerialPortOwner` | function | `MIDISetup.h` | obsolete serial-port ownership API intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED("No longer supported", macos(10.1, 10.6))` |
| `MIDISetupCreate` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupDispose` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupFromData` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupInstall` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped (raw declaration in `coremidi::ffi` only) | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
