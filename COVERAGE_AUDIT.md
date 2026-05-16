# coremidi-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 235
VERIFIED: 176
GAPS: 30
EXEMPT: 29
COVERAGE_PCT: 85.44%

- Counts cover top-level public CoreMIDI declarations (types, exported constants, Obj-C interfaces/protocols, and non-inline functions), not per-method Obj-C coverage.
- `ffi::<symbol>` rows indicate exact raw C bindings in `coremidi::ffi`; raw extern functions/statics are feature-gated behind `raw-ffi`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MIDIBluetoothDriverActivateAllConnections` | function | `MIDIBluetoothConnection.h` | `ffi::MIDIBluetoothDriverActivateAllConnections` |
| `MIDIBluetoothDriverDisconnect` | function | `MIDIBluetoothConnection.h` | `ffi::MIDIBluetoothDriverDisconnect` |
| `MIDICIDevice` | interface | `MIDICIDevice.h` | `capability::discovered_ci_devices / capability::CiDeviceInfo` |
| `MIDICIDeviceManager` | interface | `MIDICIDeviceManager.h` | `capability::discovered_ci_devices` |
| `MIDICIProfile` | interface | `MIDICapabilityInquiry.h` | `capability::legacy_ci_profile / capability::LegacyCiProfileInfo` |
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
| `MIDIEventListForEachEvent` | function | `MIDIMessages.h` | `packet::EventListRef::iter / EventIter` |
| `MIDIUMPFunctionBlockDirection` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockDirection` |
| `MIDIUMPFunctionBlockMIDI1Info` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockMIDI1Info` |
| `MIDIUMPFunctionBlockUIHint` | enum | `MIDIMessages.h` | `ffi::MIDIUMPFunctionBlockUIHint` |
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
| `MIDICIProfileID` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileID` |
| `MIDICIProfileIDManufacturerSpecific` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileIDManufacturerSpecific` |
| `MIDICIProfileIDStandard` | struct | `MIDIUMPCI.h` | `ffi::MIDICIProfileIDStandard` |
| `MIDICIProfileType` | enum | `MIDIUMPCI.h` | `ffi::MIDICIProfileType` |
| `MIDIUMPCIObjectBackingType` | enum | `MIDIUMPCI.h` | `ffi::MIDIUMPCIObjectBackingType` |
| `MIDIUMPCIProfile` | interface | `MIDIUMPCIProfile.h` | `capability::CiProfileInfo` |
| `MIDI2DeviceInfo` | interface | `MIDIUMPEndpoint.h` | `endpoint::Midi2DeviceInfo / endpoint::Midi2DeviceInfoHandle` |
| `MIDIUMPEndpoint` | interface | `MIDIUMPEndpoint.h` | `endpoint::UmpEndpointInfo / endpoint::UmpEndpointManager` |
| `MIDIUMPProtocolOptions` | enum | `MIDIUMPEndpoint.h` | `ffi::MIDIUMPProtocolOptions` |
| `MIDIUMPEndpointManager` | interface | `MIDIUMPEndpointManager.h` | `endpoint::UmpEndpointManager` |
| `MIDIUMPFunctionBlock` | interface | `MIDIUMPFunctionBlock.h` | `endpoint::UmpFunctionBlockInfo` |
| `MIDIUMPMutableEndpoint` | interface | `MIDIUMPMutableEndpoint.h` | `endpoint::MutableUmpEndpoint` |
| `MIDIUMPMutableFunctionBlock` | interface | `MIDIUMPMutableFunctionBlock.h` | `endpoint::MutableUmpFunctionBlock` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `MIDICIDeviceObjectKey` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager userInfo dictionary keys are not surfaced. |
| `MIDICIDeviceWasAddedNotification` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager notifications are not surfaced. |
| `MIDICIDeviceWasRemovedNotification` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager notifications are not surfaced. |
| `MIDICIProfileObjectKey` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager userInfo dictionary keys are not surfaced. |
| `MIDICIProfileWasRemovedNotification` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager notifications are not surfaced. |
| `MIDICIProfileWasUpdatedNotification` | const | `MIDICIDeviceManager.h` | MIDICIDeviceManager notifications are not surfaced. |
| `MIDICIProfileState` | interface | `MIDICapabilityInquiry.h` | No public wrapper for per-channel enabled/disabled profile-state snapshots. |
| `MIDICVStatus` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDIMessageType` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDIMessage_128` | struct | `MIDIMessages.h` | Fixed-width MIDIMessages.h UMP helper structs are not exposed. |
| `MIDIMessage_64` | struct | `MIDIMessages.h` | Fixed-width MIDIMessages.h UMP helper structs are not exposed. |
| `MIDIMessage_96` | struct | `MIDIMessages.h` | Fixed-width MIDIMessages.h UMP helper structs are not exposed. |
| `MIDINoteAttribute` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDIPerNoteManagementOptions` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDIProgramChangeOptions` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDISysExStatus` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDISystemStatus` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDIUtilityStatus` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `UMPStreamMessageFormat` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `UMPStreamMessageStatus` | enum | `MIDIMessages.h` | MIDIMessages.h typed UMP status/attribute enums are not exposed. |
| `MIDICIManagementMessageType` | enum | `MIDIUMPCI.h` | Typed MIDI-CI management sub-ID enums are not exposed. |
| `MIDICIProcessInquiryMessageType` | enum | `MIDIUMPCI.h` | Typed MIDI-CI process-inquiry sub-ID enums are not exposed. |
| `MIDICIProfileMessageType` | enum | `MIDIUMPCI.h` | Typed MIDI-CI profile message sub-ID enums are not exposed. |
| `MIDICIPropertyExchangeMessageType` | enum | `MIDIUMPCI.h` | Typed MIDI-CI property-exchange sub-ID enums are not exposed. |
| `MIDIUMPEndpointObjectKey` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager userInfo dictionary keys are not surfaced. |
| `MIDIUMPEndpointWasAddedNotification` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager notifications are not surfaced. |
| `MIDIUMPEndpointWasRemovedNotification` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager notifications are not surfaced. |
| `MIDIUMPEndpointWasUpdatedNotification` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager notifications are not surfaced. |
| `MIDIUMPFunctionBlockObjectKey` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager userInfo dictionary keys are not surfaced. |
| `MIDIUMPFunctionBlockWasUpdatedNotification` | const | `MIDIUMPEndpointManager.h` | MIDIUMPEndpointManager notifications are not surfaced. |

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
| `MIDIDestinationCreate` | function | `MIDIServices.h` | legacy packet-list virtual destination constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIDestinationCreateWithProtocol", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDIDestinationCreateWithBlock` | function | `MIDIServices.h` | legacy packet-list virtual destination constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIDestinationCreateWithProtocol", macos(10.11, API_TO_BE_DEPRECATED))` |
| `MIDIInputPortCreate` | function | `MIDIServices.h` | legacy packet-list input-port constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIInputPortCreateWithProtocol", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDIInputPortCreateWithBlock` | function | `MIDIServices.h` | legacy packet-list input-port constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIInputPortCreateWithProtocol", macos(10.11, API_TO_BE_DEPRECATED))` |
| `MIDIPacketListAdd` | function | `MIDIServices.h` | legacy packet-list builder intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIEventListAdd", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDIPacketListInit` | function | `MIDIServices.h` | legacy packet-list builder intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIEventListInit", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDIReceived` | function | `MIDIServices.h` | legacy packet-list receive API intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIReceivedEventList", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDISend` | function | `MIDIServices.h` | legacy packet-list send API intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDISendEventList", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDISourceCreate` | function | `MIDIServices.h` | legacy packet-list virtual source constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDISourceCreateWithProtocol", macos(10.0, API_TO_BE_DEPRECATED))` |
| `kMIDIPropertyFactoryPatchNameFile` | const | `MIDIServices.h` | 10.x-deprecated property constant intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("kMIDIPropertyNameConfiguration", macos(10.1, 10.2))` |
| `kMIDIPropertyNameConfiguration` | const | `MIDIServices.h` | 10.x-deprecated property constant intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("kMIDIPropertyNameConfigurationDictionary", macos(10.2, 10.15))` |
| `kMIDIPropertyUserPatchNameFile` | const | `MIDIServices.h` | 10.x-deprecated property constant intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("kMIDIPropertyNameConfiguration", macos(10.1, 10.2))` |
| `MIDIDeviceAddEntity` | function | `MIDISetup.h` | deprecated pre-protocol entity constructor intentionally skipped | `API_DEPRECATED_WITH_REPLACEMENT("MIDIDeviceNewEntity", macos(10.0, API_TO_BE_DEPRECATED))` |
| `MIDIGetSerialPortDrivers` | function | `MIDISetup.h` | obsolete serial-port ownership API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.1, 10.6))` |
| `MIDIGetSerialPortOwner` | function | `MIDISetup.h` | obsolete serial-port ownership API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.1, 10.6))` |
| `MIDISetSerialPortOwner` | function | `MIDISetup.h` | obsolete serial-port ownership API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.1, 10.6))` |
| `MIDISetupCreate` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupDispose` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupFromData` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupGetCurrent` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupInstall` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
| `MIDISetupToData` | function | `MIDISetup.h` | obsolete setup-blob API intentionally skipped | `API_DEPRECATED("No longer supported", macos(10.0, 10.6))` |
