import CoreMIDI
import Foundation

@_cdecl("cmr_flush_output")
public func cmr_flush_output(
    _ destination: MIDIEndpointRef,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cmrCheckStatus(MIDIFlushOutput(destination), errorOut)
}
