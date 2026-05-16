import CoreMIDI
import Foundation

private struct CMRMIDI2DeviceInfoPayload: Codable {
    var manufacturerID: [UInt8]
    var family: UInt16
    var modelNumber: UInt16
    var revisionLevel: [UInt8]
}

private struct CMRCIProfilePayload: Codable {
    var name: String
    var profileID: [UInt8]
    var profileType: UInt8
    var groupOffset: UInt8
    var firstChannel: UInt8
    var enabledChannelCount: UInt16
    var totalChannelCount: UInt16
    var isEnabled: Bool
}

private struct CMRCIDevicePayload: Codable {
    var deviceInfo: CMRMIDI2DeviceInfoPayload
    var muid: UInt32
    var supportsProtocolNegotiation: Bool
    var supportsProfileConfiguration: Bool
    var supportsPropertyExchange: Bool
    var supportsProcessInquiry: Bool
    var maxSysExSize: UInt
    var maxPropertyExchangeRequests: UInt
    var deviceType: UInt8
    var profiles: [CMRCIProfilePayload]
}

private struct CMRLegacyProfilePayload: Codable {
    var name: String
    var profileID: [UInt8]
}

private struct CMRCIProfileStatePayload: Codable {
    var midiChannel: UInt8
    var enabledProfiles: [CMRLegacyProfilePayload]
    var disabledProfiles: [CMRLegacyProfilePayload]
}

private func cmrBytes<T>(of value: T) -> [UInt8] {
    withUnsafeBytes(of: value) { Array($0) }
}

private func cmrLegacyProfilePayload(_ profile: MIDICIProfile) -> CMRLegacyProfilePayload {
    CMRLegacyProfilePayload(name: profile.name, profileID: [UInt8](profile.profileID))
}

private func cmrLegacyProfile(_ payload: CMRLegacyProfilePayload) throws -> MIDICIProfile {
    guard payload.profileID.count == 5 else {
        throw cmrError("MIDI-CI profiles require exactly 5 profile-ID bytes")
    }
    return MIDICIProfile(data: Data(payload.profileID), name: payload.name)
}

private func cmrCIProfileStatePayload(_ state: MIDICIProfileState) -> CMRCIProfileStatePayload {
    CMRCIProfileStatePayload(
        midiChannel: state.midiChannel,
        enabledProfiles: state.enabledProfiles.map(cmrLegacyProfilePayload),
        disabledProfiles: state.disabledProfiles.map(cmrLegacyProfilePayload)
    )
}

@available(macOS 15.0, *)
private func cmrDeviceInfoPayload(_ info: MIDI2DeviceInfo) -> CMRMIDI2DeviceInfoPayload {
    CMRMIDI2DeviceInfoPayload(
        manufacturerID: cmrBytes(of: info.manufacturerID.sysExIDByte),
        family: info.family,
        modelNumber: info.modelNumber,
        revisionLevel: cmrBytes(of: info.revisionLevel.revisionLevel)
    )
}

@available(macOS 15.0, *)
private func cmrCIProfilePayload(_ profile: MIDIUMPCIProfile) -> CMRCIProfilePayload {
    CMRCIProfilePayload(
        name: profile.name,
        profileID: cmrBytes(of: profile.profileID),
        profileType: profile.profileType.rawValue,
        groupOffset: profile.groupOffset,
        firstChannel: profile.firstChannel,
        enabledChannelCount: profile.enabledChannelCount,
        totalChannelCount: profile.totalChannelCount,
        isEnabled: profile.isEnabled
    )
}

@available(macOS 15.0, *)
private func cmrCIDevicePayload(_ device: MIDICIDevice) -> CMRCIDevicePayload {
    let muid = (device.value(forKey: "MUID") as? NSNumber)?.uint32Value ?? 0
    let deviceType = device.deviceType.rawValue
    return CMRCIDevicePayload(
        deviceInfo: cmrDeviceInfoPayload(device.deviceInfo),
        muid: muid,
        supportsProtocolNegotiation: device.supportsProtocolNegotiation,
        supportsProfileConfiguration: device.supportsProfileConfiguration,
        supportsPropertyExchange: device.supportsPropertyExchange,
        supportsProcessInquiry: device.supportsProcessInquiry,
        maxSysExSize: UInt(device.maxSysExSize),
        maxPropertyExchangeRequests: UInt(device.maxPropertyExchangeRequests),
        deviceType: deviceType,
        profiles: device.profiles.map(cmrCIProfilePayload)
    )
}

@_cdecl("cmr_ci_device_manager_constants_json")
public func cmr_ci_device_manager_constants_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        let unavailable: [String: String] = [
            "device_added_notification": "",
            "device_removed_notification": "",
            "profile_updated_notification": "",
            "profile_removed_notification": "",
            "device_object_key": "",
            "profile_object_key": "",
        ]
        return cmrString(cmrJSONString(unavailable))
    }
    let payload: [String: String] = [
        "device_added_notification": MIDICIDeviceManager.deviceWasAddedNotification.rawValue,
        "device_removed_notification": MIDICIDeviceManager.deviceWasRemovedNotification.rawValue,
        "profile_updated_notification": MIDICIDeviceManager.profileWasUpdatedNotification.rawValue,
        "profile_removed_notification": MIDICIDeviceManager.profileWasRemovedNotification.rawValue,
        "device_object_key": MIDICIDeviceManager.DictionaryKey.deviceObject.rawValue,
        "profile_object_key": MIDICIDeviceManager.DictionaryKey.profileObject.rawValue,
    ]
    return cmrString(cmrJSONString(payload))
}

@_cdecl("cmr_ci_devices_json")
public func cmr_ci_devices_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        return cmrString("[]")
    }
    let payload = MIDICIDeviceManager.shared.discoveredCIDevices.map(cmrCIDevicePayload)
    return cmrString(cmrJSONString(payload))
}

@_cdecl("cmr_legacy_ci_profile_json")
public func cmr_legacy_ci_profile_json(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let data = cmrMakeData(bytes, length)
        let profile: MIDICIProfile
        if let name {
            profile = MIDICIProfile(data: data, name: String(cString: name))
        } else if #available(macOS 11.0, *) {
            profile = MIDICIProfile(data: data)
        } else {
            throw cmrError("legacy MIDICIProfile(data:) requires macOS 11 or newer when no name is supplied")
        }
        return cmrString(cmrJSONString(cmrLegacyProfilePayload(profile)))
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return nil
    }
}

@_cdecl("cmr_ci_profile_state_new")
public func cmr_ci_profile_state_new(
    _ midiChannel: UInt8,
    _ useMidiChannel: Bool,
    _ enabledJSON: UnsafePointer<CChar>?,
    _ disabledJSON: UnsafePointer<CChar>?,
    _ outState: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outState.pointee = nil

    do {
        if useMidiChannel, midiChannel > 0x0F {
            throw cmrError("MIDI-CI profile-state channel must be in the range 0...15")
        }
        let enabledPayloads = try cmrDecodeJSON(enabledJSON, as: [CMRLegacyProfilePayload].self)
        let disabledPayloads = try cmrDecodeJSON(disabledJSON, as: [CMRLegacyProfilePayload].self)
        let enabledProfiles = try enabledPayloads.map(cmrLegacyProfile)
        let disabledProfiles = try disabledPayloads.map(cmrLegacyProfile)

        let state: MIDICIProfileState
        if useMidiChannel {
            state = MIDICIProfileState(
                channel: midiChannel,
                enabledProfiles: enabledProfiles,
                disabledProfiles: disabledProfiles
            )
        } else if #available(macOS 15.0, *) {
            state = MIDICIProfileState(enabledProfiles: enabledProfiles, disabledProfiles: disabledProfiles)
        } else {
            state = MIDICIProfileState(
                channel: 0,
                enabledProfiles: enabledProfiles,
                disabledProfiles: disabledProfiles
            )
        }

        outState.pointee = cmrRetain(state)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_ci_profile_state_json")
public func cmr_ci_profile_state_json(_ statePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let state: MIDICIProfileState = cmrBorrow(statePtr) else {
        return cmrString("null")
    }
    return cmrString(cmrJSONString(cmrCIProfileStatePayload(state)))
}
