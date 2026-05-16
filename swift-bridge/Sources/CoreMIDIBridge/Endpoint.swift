import CoreMIDI
import Foundation

public typealias CMRReceiveBlockCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<MIDIEventList>?,
    UnsafeMutableRawPointer?
) -> Void

private struct CMRMIDI2DeviceInfoPayload: Codable {
    var manufacturerID: [UInt8]
    var family: UInt16
    var modelNumber: UInt16
    var revisionLevel: [UInt8]
}

private struct CMRUmpFunctionBlockPayload: Codable {
    var name: String
    var functionBlockID: UInt8
    var direction: Int32
    var firstGroup: UInt8
    var totalGroupsSpanned: UInt8
    var maxSysEx8Streams: UInt8
    var midi1Info: Int32
    var uiHint: Int32
    var isEnabled: Bool
    var ciDeviceMUID: UInt32?
}

private struct CMRUmpEndpointPayload: Codable {
    var name: String
    var midiProtocol: Int32
    var supportedMIDIProtocols: UInt8
    var midiDestination: UInt32
    var midiSource: UInt32
    var deviceInfo: CMRMIDI2DeviceInfoPayload
    var productInstanceID: String
    var hasStaticFunctionBlocks: Bool
    var hasJRTSReceiveCapability: Bool
    var hasJRTSTransmitCapability: Bool
    var endpointType: UInt8
    var functionBlocks: [CMRUmpFunctionBlockPayload]
}

private final class CMRReceiveBox: NSObject {
    let callback: CMRReceiveBlockCallback
    let userInfo: UnsafeMutableRawPointer?

    init(callback: @escaping CMRReceiveBlockCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
        super.init()
    }

    func block(_ eventList: UnsafePointer<MIDIEventList>?, _ sourceRefCon: UnsafeMutableRawPointer?) {
        callback(userInfo, eventList, sourceRefCon)
    }
}

@available(macOS 15.0, *)
private final class CMRMutableEndpointBox: NSObject {
    let endpoint: MIDIUMPMutableEndpoint
    let receiveBox: CMRReceiveBox?

    init(endpoint: MIDIUMPMutableEndpoint, receiveBox: CMRReceiveBox?) {
        self.endpoint = endpoint
        self.receiveBox = receiveBox
        super.init()
    }
}

@available(macOS 15.0, *)
private func cmrMutableEndpointBox(_ ptr: UnsafeMutableRawPointer?) -> CMRMutableEndpointBox? {
    cmrBorrow(ptr)
}

@available(macOS 15.0, *)
private func cmrMutableFunctionBlock(_ ptr: UnsafeMutableRawPointer?) -> MIDIUMPMutableFunctionBlock? {
    cmrBorrow(ptr)
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
private func cmrFunctionBlockPayload(_ block: MIDIUMPFunctionBlock) -> CMRUmpFunctionBlockPayload {
    let midi1Info = (block.value(forKey: "MIDI1Info") as? NSNumber)?.int32Value ?? 0
    let uiHint = (block.value(forKey: "UIHint") as? NSNumber)?.int32Value ?? 0
    let ciDevice = block.midiCIDevice
    let ciMUID = (ciDevice?.value(forKey: "MUID") as? NSNumber)?.uint32Value
    return CMRUmpFunctionBlockPayload(
        name: block.name,
        functionBlockID: block.functionBlockID,
        direction: block.direction.rawValue,
        firstGroup: block.firstGroup,
        totalGroupsSpanned: block.totalGroupsSpanned,
        maxSysEx8Streams: block.maxSysEx8Streams,
        midi1Info: midi1Info,
        uiHint: uiHint,
        isEnabled: block.isEnabled,
        ciDeviceMUID: ciMUID
    )
}

@available(macOS 15.0, *)
private func cmrEndpointPayload(_ endpoint: MIDIUMPEndpoint) -> CMRUmpEndpointPayload {
    let midiProtocol = (endpoint.value(forKey: "MIDIProtocol") as? NSNumber)?.int32Value ?? 0
    let supportedProtocols = (endpoint.value(forKey: "supportedMIDIProtocols") as? NSNumber)?.uint8Value ?? 0
    let midiDestination = (endpoint.value(forKey: "MIDIDestination") as? NSNumber)?.uint32Value ?? 0
    let midiSource = (endpoint.value(forKey: "MIDISource") as? NSNumber)?.uint32Value ?? 0
    let endpointType = (endpoint.value(forKey: "endpointType") as? NSNumber)?.uint8Value ?? 0

    return CMRUmpEndpointPayload(
        name: endpoint.name,
        midiProtocol: midiProtocol,
        supportedMIDIProtocols: supportedProtocols,
        midiDestination: midiDestination,
        midiSource: midiSource,
        deviceInfo: cmrDeviceInfoPayload(endpoint.deviceInfo),
        productInstanceID: endpoint.productInstanceID,
        hasStaticFunctionBlocks: endpoint.hasStaticFunctionBlocks,
        hasJRTSReceiveCapability: endpoint.hasJRTSReceiveCapability,
        hasJRTSTransmitCapability: endpoint.hasJRTSTransmitCapability,
        endpointType: endpointType,
        functionBlocks: endpoint.functionBlocks.map(cmrFunctionBlockPayload)
    )
}

@_cdecl("cmr_ump_endpoint_manager_constants_json")
public func cmr_ump_endpoint_manager_constants_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        let unavailable: [String: String] = [
            "endpoint_added_notification": "",
            "endpoint_removed_notification": "",
            "endpoint_updated_notification": "",
            "function_block_updated_notification": "",
            "endpoint_object_key": "",
            "function_block_object_key": "",
        ]
        return cmrString(cmrJSONString(unavailable))
    }
    let payload: [String: String] = [
        "endpoint_added_notification": MIDIUMPEndpointManager.endpointWasAddedNotification.rawValue,
        "endpoint_removed_notification": MIDIUMPEndpointManager.endpointWasRemovedNotification.rawValue,
        "endpoint_updated_notification": MIDIUMPEndpointManager.endpointWasUpdatedNotification.rawValue,
        "function_block_updated_notification": MIDIUMPEndpointManager.functionBlockWasUpdatedNotification.rawValue,
        "endpoint_object_key": MIDIUMPEndpointManager.DictionaryKey.endpointObject.rawValue,
        "function_block_object_key": MIDIUMPEndpointManager.DictionaryKey.functionBlockObject.rawValue,
    ]
    return cmrString(cmrJSONString(payload))
}

@_cdecl("cmr_ump_endpoint_manager_endpoints_json")
public func cmr_ump_endpoint_manager_endpoints_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        return cmrString("[]")
    }
    let payload = MIDIUMPEndpointManager.shared.umpEndpoints.map(cmrEndpointPayload)
    return cmrString(cmrJSONString(payload))
}

@_cdecl("cmr_ump_device_info_new")
public func cmr_ump_device_info_new(
    _ manufacturer1: UInt8,
    _ manufacturer2: UInt8,
    _ manufacturer3: UInt8,
    _ family: UInt16,
    _ modelNumber: UInt16,
    _ revision1: UInt8,
    _ revision2: UInt8,
    _ revision3: UInt8,
    _ revision4: UInt8,
    _ outInfo: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outInfo.pointee = nil
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDI2DeviceInfo requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }

    let manufacturer = MIDI2DeviceManufacturer(sysExIDByte: (manufacturer1, manufacturer2, manufacturer3))
    let revision = MIDI2DeviceRevisionLevel(revisionLevel: (revision1, revision2, revision3, revision4))
    let info = MIDI2DeviceInfo(manufacturerID: manufacturer, family: family, modelNumber: modelNumber, revisionLevel: revision)
    outInfo.pointee = cmrRetain(info)
    return CMR_OK
}

@_cdecl("cmr_ump_device_info_json")
public func cmr_ump_device_info_json(_ infoPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        return cmrString("null")
    }
    guard let info: MIDI2DeviceInfo = cmrBorrow(infoPtr) else {
        return cmrString("null")
    }
    return cmrString(cmrJSONString(cmrDeviceInfoPayload(info)))
}

@_cdecl("cmr_ump_mutable_function_block_new")
public func cmr_ump_mutable_function_block_new(
    _ name: UnsafePointer<CChar>?,
    _ direction: Int32,
    _ firstGroup: UInt8,
    _ totalGroupsSpanned: UInt8,
    _ maxSysEx8Streams: UInt8,
    _ midi1Info: Int32,
    _ uiHint: Int32,
    _ isEnabled: Bool,
    _ outBlock: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBlock.pointee = nil
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableFunctionBlock requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }

    do {
        let name = try cmrRequireString(name, "function block name")
        guard let direction = MIDIUMPFunctionBlockDirection(rawValue: direction),
              let midi1Info = MIDIUMPFunctionBlockMIDI1Info(rawValue: midi1Info),
              let uiHint = MIDIUMPFunctionBlockUIHint(rawValue: uiHint) else {
            throw cmrError("invalid function block enum value")
        }

        guard let block = MIDIUMPMutableFunctionBlock(
            name: name,
            direction: direction,
            firstGroup: firstGroup,
            totalGroupsSpanned: totalGroupsSpanned,
            maxSysEx8Streams: maxSysEx8Streams,
            midi1Info: midi1Info,
            uiHint: uiHint,
            isEnabled: isEnabled
        ) else {
            throw cmrError("failed to create MIDIUMPMutableFunctionBlock")
        }

        outBlock.pointee = cmrRetain(block)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

@_cdecl("cmr_ump_function_block_json")
public func cmr_ump_function_block_json(_ blockPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        return cmrString("null")
    }
    guard let block: MIDIUMPFunctionBlock = cmrBorrow(blockPtr) else {
        return cmrString("null")
    }
    return cmrString(cmrJSONString(cmrFunctionBlockPayload(block)))
}

@_cdecl("cmr_ump_function_block_set_enabled")
public func cmr_ump_function_block_set_enabled(
    _ blockPtr: UnsafeMutableRawPointer?,
    _ isEnabled: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableFunctionBlock requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let block = cmrMutableFunctionBlock(blockPtr) else {
        cmrWriteError(errorOut, "function block must not be null")
        return CMR_INVALID_ARGUMENT
    }
    do {
        try block.setEnabled(isEnabled)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

@_cdecl("cmr_ump_function_block_set_name")
public func cmr_ump_function_block_set_name(
    _ blockPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableFunctionBlock requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let block = cmrMutableFunctionBlock(blockPtr) else {
        cmrWriteError(errorOut, "function block must not be null")
        return CMR_INVALID_ARGUMENT
    }
    do {
        let name = try cmrRequireString(name, "function block name")
        try block.setName(name)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_ump_function_block_reconfigure")
public func cmr_ump_function_block_reconfigure(
    _ blockPtr: UnsafeMutableRawPointer?,
    _ firstGroup: UInt8,
    _ direction: Int32,
    _ midi1Info: Int32,
    _ uiHint: Int32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableFunctionBlock requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let block = cmrMutableFunctionBlock(blockPtr) else {
        cmrWriteError(errorOut, "function block must not be null")
        return CMR_INVALID_ARGUMENT
    }
    guard let direction = MIDIUMPFunctionBlockDirection(rawValue: direction),
          let midi1Info = MIDIUMPFunctionBlockMIDI1Info(rawValue: midi1Info),
          let uiHint = MIDIUMPFunctionBlockUIHint(rawValue: uiHint) else {
        cmrWriteError(errorOut, "invalid function block enum value")
        return CMR_INVALID_ARGUMENT
    }

    do {
        try block.reconfigure(firstGroup: firstGroup, direction: direction, MIDI1Info: midi1Info, UIHint: uiHint)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

@_cdecl("cmr_ump_mutable_endpoint_new")
public func cmr_ump_mutable_endpoint_new(
    _ name: UnsafePointer<CChar>?,
    _ deviceInfoPtr: UnsafeMutableRawPointer?,
    _ productInstanceID: UnsafePointer<CChar>?,
    _ midiProtocol: Int32,
    _ callback: CMRReceiveBlockCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outEndpoint: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outEndpoint.pointee = nil
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableEndpoint requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }

    do {
        let name = try cmrRequireString(name, "UMP endpoint name")
        let productInstanceID = try cmrRequireString(productInstanceID, "product instance ID")
        guard let deviceInfo: MIDI2DeviceInfo = cmrBorrow(deviceInfoPtr) else {
            throw cmrError("deviceInfo must not be null")
        }
        guard let receiveBox = callback.map({ CMRReceiveBox(callback: $0, userInfo: userInfo) }) else {
            throw cmrError("destination callback must not be null")
        }
        guard let midiProtocol = MIDIProtocolID(rawValue: midiProtocol) else {
            throw cmrError("invalid MIDI protocol value \(midiProtocol)")
        }
        guard let endpoint = MIDIUMPMutableEndpoint(
            name: name,
            deviceInfo: deviceInfo,
            productInstanceID: productInstanceID,
            midiProtocol: midiProtocol,
            destinationCallback: { eventList, sourceRefCon in
                receiveBox.block(eventList, sourceRefCon)
            }
        ) else {
            throw cmrError("failed to create MIDIUMPMutableEndpoint")
        }
        let box = CMRMutableEndpointBox(endpoint: endpoint, receiveBox: receiveBox)
        outEndpoint.pointee = cmrRetain(box)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

@_cdecl("cmr_ump_endpoint_json")
public func cmr_ump_endpoint_json(_ endpointPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.0, *) else {
        return cmrString("null")
    }
    if let box = cmrMutableEndpointBox(endpointPtr) {
        return cmrString(cmrJSONString(cmrEndpointPayload(box.endpoint)))
    }
    guard let endpoint: MIDIUMPEndpoint = cmrBorrow(endpointPtr) else {
        return cmrString("null")
    }
    return cmrString(cmrJSONString(cmrEndpointPayload(endpoint)))
}

@_cdecl("cmr_ump_mutable_endpoint_set_name")
public func cmr_ump_mutable_endpoint_set_name(
    _ endpointPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableEndpoint requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let endpoint = cmrMutableEndpointBox(endpointPtr)?.endpoint else {
        cmrWriteError(errorOut, "mutable endpoint must not be null")
        return CMR_INVALID_ARGUMENT
    }
    do {
        let name = try cmrRequireString(name, "UMP endpoint name")
        try endpoint.setName(name)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_ump_mutable_endpoint_register_function_blocks")
public func cmr_ump_mutable_endpoint_register_function_blocks(
    _ endpointPtr: UnsafeMutableRawPointer?,
    _ functionBlocks: UnsafePointer<UnsafeMutableRawPointer>?,
    _ count: Int,
    _ markAsStatic: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableEndpoint requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let endpoint = cmrMutableEndpointBox(endpointPtr)?.endpoint else {
        cmrWriteError(errorOut, "mutable endpoint must not be null")
        return CMR_INVALID_ARGUMENT
    }
    let buffer = UnsafeBufferPointer(start: functionBlocks, count: count)
    let blocks = buffer.compactMap(cmrMutableFunctionBlock)
    do {
        try endpoint.registerFunctionBlocks(blocks, markAsStatic: markAsStatic)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

@_cdecl("cmr_ump_mutable_endpoint_set_enabled")
public func cmr_ump_mutable_endpoint_set_enabled(
    _ endpointPtr: UnsafeMutableRawPointer?,
    _ isEnabled: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 15.0, *) else {
        cmrWriteError(errorOut, "MIDIUMPMutableEndpoint requires macOS 15 or newer")
        return CMR_UNAVAILABLE
    }
    guard let endpoint = cmrMutableEndpointBox(endpointPtr)?.endpoint else {
        cmrWriteError(errorOut, "mutable endpoint must not be null")
        return CMR_INVALID_ARGUMENT
    }
    do {
        try endpoint.setEnabled(isEnabled)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_FRAMEWORK_ERROR
    }
}

private func cmrWriteNSError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: NSError?
) -> Int32 {
    cmrWriteError(errorOut, error?.localizedDescription ?? "unknown CoreMIDI bridge error")
    return CMR_FRAMEWORK_ERROR
}
