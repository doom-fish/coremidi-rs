import CoreMIDI
import Foundation

public typealias CMRNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

private final class CMRClientBox: NSObject {
    let client: MIDIClientRef

    init(client: MIDIClientRef) {
        self.client = client
        super.init()
    }

    deinit {
        MIDIClientDispose(client)
    }
}

private func cmrClientBox(_ ptr: UnsafeMutableRawPointer?) -> CMRClientBox? {
    cmrBorrow(ptr)
}

@_cdecl("cmr_client_new_with_notifications")
public func cmr_client_new_with_notifications(
    _ name: UnsafePointer<CChar>?,
    _ callback: CMRNotificationCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outClient: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outClient.pointee = nil

    do {
        let clientName = try cmrRequireString(name, "client name")
        var client: MIDIClientRef = 0
        let status = MIDIClientCreateWithBlock(clientName as CFString, &client) { message in
            guard let callback else {
                return
            }
            let json = cmrNotificationPayload(message)
            json.withCString { callback(userInfo, $0) }
        }
        let checked = cmrCheckStatus(status, errorOut)
        guard checked == CMR_OK else {
            return checked
        }
        outClient.pointee = cmrRetain(CMRClientBox(client: client))
        return CMR_OK
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_client_raw")
public func cmr_client_raw(_ clientPtr: UnsafeMutableRawPointer?) -> MIDIClientRef {
    cmrClientBox(clientPtr)?.client ?? 0
}

@_cdecl("cmr_client_restart")
public func cmr_client_restart(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    cmrCheckStatus(MIDIRestart(), errorOut)
}
