import CoreMIDI
import Foundation

func cmrNotificationPayload(_ message: UnsafePointer<MIDINotification>) -> String {
    let notification = message.pointee
    var payload: [String: Any] = [
        "message_id": Int(notification.messageID.rawValue),
        "message_size": Int(notification.messageSize),
    ]

    switch Int(notification.messageID.rawValue) {
    case 2, 3:
        let typed = UnsafeRawPointer(message)
            .assumingMemoryBound(to: MIDIObjectAddRemoveNotification.self)
            .pointee
        payload["parent"] = Int(typed.parent)
        payload["parent_type"] = Int(typed.parentType.rawValue)
        payload["child"] = Int(typed.child)
        payload["child_type"] = Int(typed.childType.rawValue)
    case 4:
        let typed = UnsafeRawPointer(message)
            .assumingMemoryBound(to: MIDIObjectPropertyChangeNotification.self)
            .pointee
        payload["object"] = Int(typed.object)
        payload["object_type"] = Int(typed.objectType.rawValue)
        payload["property_name"] = typed.propertyName.takeUnretainedValue() as String
    case 7:
        let typed = UnsafeRawPointer(message)
            .assumingMemoryBound(to: MIDIIOErrorNotification.self)
            .pointee
        payload["driver_device"] = Int(typed.driverDevice)
        payload["error_code"] = Int(typed.errorCode)
    default:
        break
    }

    return cmrJSONString(payload)
}
