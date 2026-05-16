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

private func cmrBytes<T>(of value: T) -> [UInt8] {
    withUnsafeBytes(of: value) { Array($0) }
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
        let payload = CMRLegacyProfilePayload(name: profile.name, profileID: [UInt8](profile.profileID))
        return cmrString(cmrJSONString(payload))
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return nil
    }
}
