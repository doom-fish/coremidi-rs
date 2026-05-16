import CoreMIDI
import Foundation

private func cmrProperty(_ property: UnsafeRawPointer?) -> CFString? {
    cmrCFString(property)
}

@_cdecl("cmr_midi_object_get_data_property")
public func cmr_midi_object_get_data_property(
    _ object: MIDIObjectRef,
    _ property: UnsafeRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outBytes.pointee = nil
    outLen.pointee = 0
    guard let property = cmrProperty(property) else {
        cmrWriteError(errorOut, "property must not be null")
        return CMR_INVALID_ARGUMENT
    }

    var dataRef: Unmanaged<CFData>?
    let status = MIDIObjectGetDataProperty(object, property, &dataRef)
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

@_cdecl("cmr_midi_object_set_data_property")
public func cmr_midi_object_set_data_property(
    _ object: MIDIObjectRef,
    _ property: UnsafeRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let property = cmrProperty(property) else {
        cmrWriteError(errorOut, "property must not be null")
        return CMR_INVALID_ARGUMENT
    }
    let data = cmrMakeData(bytes, length) as CFData
    return cmrCheckStatus(MIDIObjectSetDataProperty(object, property, data), errorOut)
}

@_cdecl("cmr_midi_object_get_dictionary_property_json")
public func cmr_midi_object_get_dictionary_property_json(
    _ object: MIDIObjectRef,
    _ property: UnsafeRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let property = cmrProperty(property) else {
        cmrWriteError(errorOut, "property must not be null")
        return nil
    }

    var dictionaryRef: Unmanaged<CFDictionary>?
    let status = MIDIObjectGetDictionaryProperty(object, property, &dictionaryRef)
    let checked = cmrCheckStatus(status, errorOut)
    guard checked == CMR_OK else {
        return nil
    }
    let dictionary = dictionaryRef?.takeRetainedValue() as NSDictionary? ?? NSDictionary()
    return cmrString(cmrJSONString(dictionary))
}

@_cdecl("cmr_midi_object_set_dictionary_property_json")
public func cmr_midi_object_set_dictionary_property_json(
    _ object: MIDIObjectRef,
    _ property: UnsafeRawPointer?,
    _ json: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let property = cmrProperty(property) else {
        cmrWriteError(errorOut, "property must not be null")
        return CMR_INVALID_ARGUMENT
    }

    do {
        let jsonString = try cmrRequireString(json, "dictionary JSON")
        let payload = try JSONSerialization.jsonObject(with: Data(jsonString.utf8))
        guard let dictionary = payload as? NSDictionary else {
            throw cmrError("dictionary JSON must decode to an object")
        }
        return cmrCheckStatus(MIDIObjectSetDictionaryProperty(object, property, dictionary), errorOut)
    } catch {
        cmrWriteError(errorOut, error.localizedDescription)
        return CMR_INVALID_ARGUMENT
    }
}

@_cdecl("cmr_midi_object_get_properties_json")
public func cmr_midi_object_get_properties_json(
    _ object: MIDIObjectRef,
    _ deep: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    var propertyListRef: Unmanaged<CFPropertyList>?
    let status = MIDIObjectGetProperties(object, &propertyListRef, deep)
    let checked = cmrCheckStatus(status, errorOut)
    guard checked == CMR_OK else {
        return nil
    }
    let propertyList = propertyListRef?.takeRetainedValue() ?? NSDictionary()
    return cmrString(cmrJSONString(propertyList))
}
