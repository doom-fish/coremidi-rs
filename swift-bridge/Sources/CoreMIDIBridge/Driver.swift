import CoreFoundation
import CoreMIDI
import Foundation

@_cdecl("cmr_driver_interface_ids_json")
public func cmr_driver_interface_ids_json() -> UnsafeMutablePointer<CChar>? {
    let payload: [String: Any] = [
        "driver_type_id": "ECDE9574-0FE4-11D4-BB1A-0050E4CEA526",
        "driver_interface_id": "49DFCA9E-0FE5-11D4-950D-0050E4CEA526",
        "driver_interface2_id": "43C98C3C-306C-11D5-AF73-003065A8301E",
        "driver_interface3_id": "2FD94D0F-8C2A-482A-8AD8-7D9EA381C9C1",
        "uses_serial_property": kMIDIDriverPropertyUsesSerial as String,
    ]
    return cmrString(cmrJSONString(payload))
}
