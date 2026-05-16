import CoreMIDI
import Foundation

@_cdecl("cmr_thru_connection_create")
public func cmr_thru_connection_create(
    _ ownerID: UnsafePointer<CChar>?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ outConnection: UnsafeMutablePointer<MIDIThruConnectionRef>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let owner = cmrStringIfPresent(ownerID) as CFString?
    let data = cmrMakeData(bytes, length) as CFData
    return cmrCheckStatus(MIDIThruConnectionCreate(owner, data, outConnection), errorOut)
}

@_cdecl("cmr_thru_connection_dispose")
public func cmr_thru_connection_dispose(
    _ connection: MIDIThruConnectionRef,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cmrCheckStatus(MIDIThruConnectionDispose(connection), errorOut)
}

@_cdecl("cmr_thru_connection_get_params")
public func cmr_thru_connection_get_params(
    _ connection: MIDIThruConnectionRef,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0

    var dataRef = Unmanaged.passUnretained(Data() as CFData)
    let status = MIDIThruConnectionGetParams(connection, &dataRef)
    let checked = cmrCheckStatus(status, errorOut)
    guard checked == CMR_OK else {
        return checked
    }

    let data = dataRef.takeRetainedValue() as Data
    cmrCopyData(data, outBytes, outLen)
    return CMR_OK
}

@_cdecl("cmr_thru_connection_set_params")
public func cmr_thru_connection_set_params(
    _ connection: MIDIThruConnectionRef,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let data = cmrMakeData(bytes, length) as CFData
    return cmrCheckStatus(MIDIThruConnectionSetParams(connection, data), errorOut)
}

@_cdecl("cmr_thru_connection_find")
public func cmr_thru_connection_find(
    _ ownerID: UnsafePointer<CChar>?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0

    do {
        let owner = try cmrRequireString(ownerID, "persistent owner id") as CFString
        var dataRef = Unmanaged.passUnretained(Data() as CFData)
        let status = MIDIThruConnectionFind(owner, &dataRef)
        let checked = cmrCheckStatus(status, errorOut)
        guard checked == CMR_OK else {
            return checked
        }
        let data = dataRef.takeRetainedValue() as Data
        cmrCopyData(data, outBytes, outLen)
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}
