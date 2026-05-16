import CoreMIDI
import Foundation

public let CMR_OK: Int32 = 0
public let CMR_FRAMEWORK_ERROR: Int32 = -1
public let CMR_INVALID_ARGUMENT: Int32 = -2
public let CMR_UNAVAILABLE: Int32 = -3

@inline(__always)
public func cmrRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func cmrBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer?) -> T? {
    guard let ptr else {
        return nil
    }
    return Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("cmr_object_release")
public func cmr_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func cmrString(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

@inline(__always)
func cmrWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = cmrString(message)
}

func cmrError(_ message: String) -> NSError {
    NSError(domain: "coremidi-rs", code: Int(CMR_INVALID_ARGUMENT), userInfo: [
        NSLocalizedDescriptionKey: message,
    ])
}

func cmrRequireString(_ cString: UnsafePointer<CChar>?, _ name: String) throws -> String {
    guard let cString else {
        throw cmrError("\(name) must not be null")
    }
    return String(cString: cString)
}

func cmrStringIfPresent(_ cString: UnsafePointer<CChar>?) -> String? {
    guard let cString else {
        return nil
    }
    return String(cString: cString)
}

func cmrDecodeJSON<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    let string = try cmrRequireString(cString, "JSON")
    let data = Data(string.utf8)
    return try JSONDecoder().decode(T.self, from: data)
}

func cmrDecodeJSONIfPresent<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T? {
    guard cString != nil else {
        return nil
    }
    return try cmrDecodeJSON(cString, as: type)
}

func cmrSafeJSON(_ value: Any) -> Any {
    switch value {
    case let dict as [String: Any]:
        return dict.mapValues(cmrSafeJSON)
    case let dict as NSDictionary:
        var object: [String: Any] = [:]
        for (key, value) in dict {
            object[String(describing: key)] = cmrSafeJSON(value)
        }
        return object
    case let array as [Any]:
        return array.map(cmrSafeJSON)
    case let array as NSArray:
        return array.map(cmrSafeJSON)
    case let data as Data:
        return [UInt8](data)
    case let data as NSData:
        return [UInt8](data as Data)
    case let number as NSNumber:
        return number
    case let string as String:
        return string
    case let date as Date:
        return date.timeIntervalSince1970
    case _ as NSNull:
        return NSNull()
    default:
        return String(describing: value)
    }
}

func cmrJSONString<T: Encodable>(_ value: T) -> String {
    let encoder = JSONEncoder()
    if #available(macOS 10.13, *) {
        encoder.outputFormatting = [.sortedKeys]
    }
    guard let data = try? encoder.encode(value),
          let string = String(data: data, encoding: .utf8) else {
        return "null"
    }
    return string
}

func cmrJSONString(_ value: Any) -> String {
    let safe = cmrSafeJSON(value)

    func encode(_ object: Any) -> String? {
        guard JSONSerialization.isValidJSONObject(object) else {
            return nil
        }
        do {
            let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
            return String(data: data, encoding: .utf8)
        } catch {
            return nil
        }
    }

    if let encoded = encode(safe) {
        return encoded
    }
    if let encodedScalar = encode([safe]) {
        return String(encodedScalar.dropFirst().dropLast())
    }
    return "null"
}

func cmrCFString(_ ptr: UnsafeRawPointer?) -> CFString? {
    guard let ptr else {
        return nil
    }
    return unsafeBitCast(ptr, to: CFString.self)
}

func cmrCopyData(
    _ data: Data,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>,
    _ outLen: UnsafeMutablePointer<Int>
) {
    outLen.pointee = data.count
    guard !data.isEmpty else {
        outBytes.pointee = nil
        return
    }

    let raw = malloc(data.count)!.assumingMemoryBound(to: UInt8.self)
    data.copyBytes(to: raw, count: data.count)
    outBytes.pointee = raw
}

func cmrMakeData(_ bytes: UnsafePointer<UInt8>?, _ length: Int) -> Data {
    guard let bytes, length > 0 else {
        return Data()
    }
    return Data(bytes: bytes, count: length)
}

func cmrCheckStatus(
    _ status: OSStatus,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    if status == noErr {
        return CMR_OK
    }
    cmrWriteError(errorOut, "CoreMIDI returned OSStatus \(status)")
    return status
}
