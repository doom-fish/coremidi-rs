import CoreMIDI
import Foundation

public typealias CMRNotificationCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void
public typealias CMRContextRetainCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CMRContextReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

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

// Owns the +1 reference taken on the Rust `NotificationContext` for the
// lifetime of the notification block. The notification block strongly captures
// this object, so while a block invocation is in flight ARC keeps this object
// (and therefore its context reference) alive for the duration of the call.
// This guarantees an in-flight notification can never observe a freed context,
// even though `MIDIClientDispose` does not synchronously drain pending blocks.
private final class CMRNotificationContext {
    let userInfo: UnsafeMutableRawPointer?
    let callback: CMRNotificationCallback?
    let contextRelease: CMRContextReleaseCallback?

    init(
        userInfo: UnsafeMutableRawPointer?,
        callback: CMRNotificationCallback?,
        contextRetain: CMRContextRetainCallback?,
        contextRelease: CMRContextReleaseCallback?
    ) {
        self.userInfo = userInfo
        self.callback = callback
        self.contextRelease = contextRelease
        if let userInfo {
            contextRetain?(userInfo)
        }
    }

    deinit {
        if let userInfo {
            contextRelease?(userInfo)
        }
    }

    func deliver(_ message: UnsafePointer<MIDINotification>) {
        guard let callback else {
            return
        }
        let json = cmrNotificationPayload(message)
        json.withCString { callback(userInfo, $0) }
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
    _ contextRetain: CMRContextRetainCallback?,
    _ contextRelease: CMRContextReleaseCallback?,
    _ outClient: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outClient.pointee = nil

    do {
        let clientName = try cmrRequireString(name, "client name")
        let notifyContext = CMRNotificationContext(
            userInfo: userInfo,
            callback: callback,
            contextRetain: contextRetain,
            contextRelease: contextRelease
        )
        var client: MIDIClientRef = 0
        let status = MIDIClientCreateWithBlock(clientName as CFString, &client) { message in
            notifyContext.deliver(message)
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
