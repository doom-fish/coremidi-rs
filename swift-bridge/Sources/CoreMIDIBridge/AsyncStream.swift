import CoreMIDI
import Foundation

public typealias CMRAsyncEventListCallback = @convention(c) (
    UnsafePointer<MIDIEventList>?, UnsafeMutableRawPointer?
) -> Void

public typealias CMRCIChangedCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

@available(macOS 15.0, *)
private final class CMRCIObserver: NSObject {
    private var observation: NSKeyValueObservation?
    let callback: CMRCIChangedCallback
    let ctx: UnsafeMutableRawPointer?

    init(callback: @escaping CMRCIChangedCallback, ctx: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.ctx = ctx
        super.init()
        let mgr = MIDICIDeviceManager.shared
        observation = mgr.observe(\MIDICIDeviceManager.discoveredCIDevices, options: [.initial, .new]) { [weak self] _, _ in
            guard let self else {
                return
            }
            self.callback(self.ctx)
        }
    }
}

@_cdecl("cmr_vdest_stream_create")
public func cmr_vdest_stream_create(
    _ client: MIDIClientRef,
    _ name: UnsafePointer<CChar>?,
    _ protocolID: MIDIProtocolID,
    _ callback: CMRAsyncEventListCallback?,
    _ ctx: UnsafeMutableRawPointer?,
    _ outEndpoint: UnsafeMutablePointer<MIDIEndpointRef>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let outEndpoint else {
        cmrWriteError(errorOut, "output endpoint pointer must not be null")
        return CMR_INVALID_ARGUMENT
    }
    outEndpoint.pointee = 0

    guard #available(macOS 11.0, *) else {
        cmrWriteError(errorOut, "MIDIDestinationCreateWithProtocol requires macOS 11 or newer")
        return CMR_UNAVAILABLE
    }

    do {
        let destinationName = try cmrRequireString(name, "destination name")
        guard let callback else {
            throw cmrError("callback must not be null")
        }

        let protocolValue: MIDIProtocolID
        switch protocolID {
        case ._1_0:
            protocolValue = ._1_0
        case ._2_0:
            protocolValue = ._2_0
        default:
            throw cmrError("invalid MIDI protocol ID")
        }

        let status = MIDIDestinationCreateWithProtocol(
            client,
            destinationName as CFString,
            protocolValue,
            outEndpoint
        ) { eventList, _ in
            callback(eventList, ctx)
        }
        return cmrCheckStatus(status, errorOut)
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_ci_discovery_subscribe")
public func cmr_ci_discovery_subscribe(
    _ callback: CMRCIChangedCallback?,
    _ ctx: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        return nil
    }
    guard let callback else {
        return nil
    }
    return cmrRetain(CMRCIObserver(callback: callback, ctx: ctx))
}

@_cdecl("cmr_ci_discovery_unsubscribe")
public func cmr_ci_discovery_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    cmr_object_release(handle)
}
