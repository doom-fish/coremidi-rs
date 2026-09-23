import CoreMIDI
import Foundation

public typealias CMRCIChangedCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

@available(macOS 15.0, *)
private final class CMRCIObserver: NSObject {
    private var observation: NSKeyValueObservation?
    let callback: CMRCIChangedCallback
    let context: CMRRetainedContext

    init(callback: @escaping CMRCIChangedCallback, context: CMRRetainedContext) {
        self.callback = callback
        self.context = context
        super.init()
        let mgr = MIDICIDeviceManager.shared
        observation = mgr.observe(\MIDICIDeviceManager.discoveredCIDevices, options: [.initial, .new]) { [weak self] _, _ in
            guard let self else {
                return
            }
            self.callback(self.context.pointer)
        }
    }
}

@_cdecl("cmr_ci_discovery_subscribe")
public func cmr_ci_discovery_subscribe(
    _ callback: CMRCIChangedCallback?,
    _ ctx: UnsafeMutableRawPointer?,
    _ contextRetain: CMRContextRetainCallback?,
    _ contextRelease: CMRContextReleaseCallback?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        return nil
    }
    guard let callback else {
        return nil
    }
    let context = CMRRetainedContext(ctx, retain: contextRetain, release: contextRelease)
    return cmrRetain(CMRCIObserver(callback: callback, context: context))
}

@_cdecl("cmr_ci_discovery_unsubscribe")
public func cmr_ci_discovery_unsubscribe(_ handle: UnsafeMutableRawPointer?) {
    cmr_object_release(handle)
}
