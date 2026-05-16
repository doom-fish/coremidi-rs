import CoreMIDI
import Foundation

@_cdecl("cmr_event_packet_sysex_bytes_for_group")
public func cmr_event_packet_sysex_bytes_for_group(
    _ packet: UnsafePointer<MIDIEventPacket>?,
    _ groupIndex: UInt8,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0

    guard let packet else {
        cmrWriteError(errorOut, "event packet must not be null")
        return CMR_INVALID_ARGUMENT
    }

    guard #available(macOS 14.0, *) else {
        cmrWriteError(errorOut, "MIDIEventPacketSysexBytesForGroup requires macOS 14 or newer")
        return CMR_UNAVAILABLE
    }

    var dataRef: Unmanaged<CFData>?
    let status = MIDIEventPacketSysexBytesForGroup(packet, groupIndex, &dataRef)
    let checked = cmrCheckStatus(status, errorOut)
    guard checked == CMR_OK else {
        return checked
    }
    guard let dataRef else {
        return CMR_OK
    }
    let data = dataRef.takeRetainedValue() as Data
    cmrCopyData(data, outBytes, outLen)
    return CMR_OK
}
