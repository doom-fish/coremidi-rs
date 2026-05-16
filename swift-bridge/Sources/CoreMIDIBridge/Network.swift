import CoreMIDI
import Foundation

private struct CMRNetworkHostPayload: Codable, Hashable {
    var name: String
    var address: String?
    var port: UInt
    var netServiceName: String?
    var netServiceDomain: String?
}

private struct CMRNetworkConnectionPayload: Codable, Hashable {
    var host: CMRNetworkHostPayload
}

private func cmrNetworkSession() -> MIDINetworkSession {
    MIDINetworkSession.default()
}

private func cmrNetworkHostPayload(_ host: MIDINetworkHost) -> CMRNetworkHostPayload {
    CMRNetworkHostPayload(
        name: host.name,
        address: host.address,
        port: UInt(host.port),
        netServiceName: host.netServiceName,
        netServiceDomain: host.netServiceDomain
    )
}

private func cmrNetworkConnectionPayload(_ connection: MIDINetworkConnection) -> CMRNetworkConnectionPayload {
    CMRNetworkConnectionPayload(host: cmrNetworkHostPayload(connection.host))
}

private func cmrMakeNetworkHost(_ payload: CMRNetworkHostPayload) throws -> MIDINetworkHost {
    if let address = payload.address {
        return MIDINetworkHost(name: payload.name, address: address, port: Int(payload.port))
    }
    if let netServiceName = payload.netServiceName, let netServiceDomain = payload.netServiceDomain {
        return MIDINetworkHost(name: payload.name, netServiceName: netServiceName, netServiceDomain: netServiceDomain)
    }
    throw cmrError("network host requires either address/port or netServiceName/netServiceDomain")
}

@_cdecl("cmr_network_constants_json")
public func cmr_network_constants_json() -> UnsafeMutablePointer<CChar>? {
    let payload: [String: Any] = [
        "bonjour_service_type": MIDINetworkBonjourServiceType,
        "contacts_changed_notification": MIDINetworkNotificationContactsDidChange,
        "session_changed_notification": MIDINetworkNotificationSessionDidChange,
    ]
    return cmrString(cmrJSONString(payload))
}

@_cdecl("cmr_network_session_is_enabled")
public func cmr_network_session_is_enabled() -> Bool {
    cmrNetworkSession().isEnabled
}

@_cdecl("cmr_network_session_set_enabled")
public func cmr_network_session_set_enabled(_ enabled: Bool) {
    cmrNetworkSession().isEnabled = enabled
}

@_cdecl("cmr_network_session_network_port")
public func cmr_network_session_network_port() -> Int {
    Int(cmrNetworkSession().networkPort)
}

@_cdecl("cmr_network_session_network_name")
public func cmr_network_session_network_name() -> UnsafeMutablePointer<CChar>? {
    cmrString(cmrNetworkSession().networkName)
}

@_cdecl("cmr_network_session_local_name")
public func cmr_network_session_local_name() -> UnsafeMutablePointer<CChar>? {
    cmrString(cmrNetworkSession().localName)
}

@_cdecl("cmr_network_session_connection_policy")
public func cmr_network_session_connection_policy() -> Int32 {
    Int32(cmrNetworkSession().connectionPolicy.rawValue)
}

@_cdecl("cmr_network_session_set_connection_policy")
public func cmr_network_session_set_connection_policy(
    _ rawValue: Int32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let policy = MIDINetworkConnectionPolicy(rawValue: UInt(rawValue)) else {
        cmrWriteError(errorOut, "invalid MIDINetworkConnectionPolicy value \(rawValue)")
        return CMR_INVALID_ARGUMENT
    }
    cmrNetworkSession().connectionPolicy = policy
    return CMR_OK
}

@_cdecl("cmr_network_session_contacts_json")
public func cmr_network_session_contacts_json() -> UnsafeMutablePointer<CChar>? {
    let contacts = cmrNetworkSession().contacts().map(cmrNetworkHostPayload)
    return cmrString(cmrJSONString(contacts))
}

@_cdecl("cmr_network_session_add_contact_json")
public func cmr_network_session_add_contact_json(
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let payload = try cmrDecodeJSON(json, as: CMRNetworkHostPayload.self)
        let host = try cmrMakeNetworkHost(payload)
        return cmrNetworkSession().addContact(host) ? CMR_OK : CMR_FRAMEWORK_ERROR
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_network_session_remove_contact_json")
public func cmr_network_session_remove_contact_json(
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let payload = try cmrDecodeJSON(json, as: CMRNetworkHostPayload.self)
        let host = try cmrMakeNetworkHost(payload)
        return cmrNetworkSession().removeContact(host) ? CMR_OK : CMR_FRAMEWORK_ERROR
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_network_session_connections_json")
public func cmr_network_session_connections_json() -> UnsafeMutablePointer<CChar>? {
    let connections = cmrNetworkSession().connections().map(cmrNetworkConnectionPayload)
    return cmrString(cmrJSONString(connections))
}

@_cdecl("cmr_network_session_add_connection_json")
public func cmr_network_session_add_connection_json(
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let payload = try cmrDecodeJSON(json, as: CMRNetworkConnectionPayload.self)
        let host = try cmrMakeNetworkHost(payload.host)
        let connection = MIDINetworkConnection(host: host)
        return cmrNetworkSession().addConnection(connection) ? CMR_OK : CMR_FRAMEWORK_ERROR
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_network_session_remove_connection_json")
public func cmr_network_session_remove_connection_json(
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        let payload = try cmrDecodeJSON(json, as: CMRNetworkConnectionPayload.self)
        let host = try cmrMakeNetworkHost(payload.host)
        let connection = MIDINetworkConnection(host: host)
        return cmrNetworkSession().removeConnection(connection) ? CMR_OK : CMR_FRAMEWORK_ERROR
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_network_session_source_endpoint")
public func cmr_network_session_source_endpoint() -> MIDIEndpointRef {
    cmrNetworkSession().sourceEndpoint()
}

@_cdecl("cmr_network_session_destination_endpoint")
public func cmr_network_session_destination_endpoint() -> MIDIEndpointRef {
    cmrNetworkSession().destinationEndpoint()
}

@_cdecl("cmr_network_activate_bluetooth_connections")
public func cmr_network_activate_bluetooth_connections(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        cmrWriteError(errorOut, "MIDIBluetoothDriverActivateAllConnections requires macOS 13 or newer")
        return CMR_UNAVAILABLE
    }
    return cmrCheckStatus(MIDIBluetoothDriverActivateAllConnections(), errorOut)
}

@_cdecl("cmr_network_disconnect_bluetooth")
public func cmr_network_disconnect_bluetooth(
    _ uuid: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        cmrWriteError(errorOut, "MIDIBluetoothDriverDisconnect requires macOS 13 or newer")
        return CMR_UNAVAILABLE
    }
    do {
        let uuid = try cmrRequireString(uuid, "Bluetooth peripheral UUID")
        return cmrCheckStatus(MIDIBluetoothDriverDisconnect(uuid as CFString), errorOut)
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}
