import CoreMIDI
import Foundation

@_cdecl("cmr_setup_current_xml")
public func cmr_setup_current_xml(
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0
    cmrWriteError(errorOut, "MIDISetupGetCurrent/MIDISetupToData are unavailable in the current macOS SDK")
    return CMR_UNAVAILABLE
}

@_cdecl("cmr_setup_serial_port_owner")
public func cmr_setup_serial_port_owner(
    _ portName: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    _ = portName
    cmrWriteError(errorOut, "MIDIGetSerialPortOwner is unavailable in the current macOS SDK")
    return nil
}

@_cdecl("cmr_setup_serial_port_drivers_json")
public func cmr_setup_serial_port_drivers_json(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    cmrWriteError(errorOut, "MIDIGetSerialPortDrivers is unavailable in the current macOS SDK")
    return nil
}
