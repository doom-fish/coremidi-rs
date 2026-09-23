import CoreMIDI
import Foundation

@_cdecl("cmr_flush_output")
public func cmr_flush_output(
    _ destination: MIDIEndpointRef,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cmrCheckStatus(MIDIFlushOutput(destination), errorOut)
}

@_cdecl("cmr_input_port_create_with_protocol")
public func cmr_input_port_create_with_protocol(
    _ client: MIDIClientRef,
    _ name: UnsafePointer<CChar>?,
    _ protocolID: MIDIProtocolID,
    _ callback: CMRReceiveBlockCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ contextRetain: CMRContextRetainCallback?,
    _ contextRelease: CMRContextReleaseCallback?,
    _ outPort: UnsafeMutablePointer<MIDIPortRef>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outPort else {
        cmrWriteError(errorOut, "output port pointer must not be null")
        return CMR_INVALID_ARGUMENT
    }
    outPort.pointee = 0

    do {
        let portName = try cmrRequireString(name, "port name")
        guard let callback else {
            throw cmrError("callback must not be null")
        }
        let protocolValue = try cmrProtocol(protocolID)
        let context = CMRRetainedContext(userInfo, retain: contextRetain, release: contextRelease)
        let status = MIDIInputPortCreateWithProtocol(
            client,
            portName as CFString,
            protocolValue,
            outPort
        ) { eventList, srcConnRefCon in
            callback(context.pointer, eventList, srcConnRefCon)
        }
        return cmrCheckStatus(status, errorOut)
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}
