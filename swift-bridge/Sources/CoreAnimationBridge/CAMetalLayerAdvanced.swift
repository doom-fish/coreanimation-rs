import Foundation
import QuartzCore

@_cdecl("ca_metal_layer_get_colorspace")
public func ca_metal_layer_get_colorspace(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAMetalLayer = caBorrow(handle), let colorSpace = layer.colorspace else { return nil }
    return caRetain(colorSpace)
}

@_cdecl("ca_metal_layer_set_colorspace")
public func ca_metal_layer_set_colorspace(_ handle: UnsafeMutableRawPointer?, _ valueHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    let colorSpace: CGColorSpace? = caBorrow(valueHandle)
    layer.colorspace = colorSpace
}

@_cdecl("ca_metal_layer_get_edr_metadata")
public func ca_metal_layer_get_edr_metadata(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *), let layer: CAMetalLayer = caBorrow(handle), let metadata = layer.edrMetadata else {
        return nil
    }
    return caRetain(metadata)
}

@_cdecl("ca_metal_layer_set_edr_metadata")
public func ca_metal_layer_set_edr_metadata(_ handle: UnsafeMutableRawPointer?, _ valueHandle: UnsafeMutableRawPointer?) {
    guard #available(macOS 10.15, *), let layer: CAMetalLayer = caBorrow(handle) else { return }
    let metadata: CAEDRMetadata? = caBorrow(valueHandle)
    layer.edrMetadata = metadata
}

@_cdecl("ca_metal_layer_get_wants_extended_dynamic_range_content")
public func ca_metal_layer_get_wants_extended_dynamic_range_content(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return layer.wantsExtendedDynamicRangeContent
}

@_cdecl("ca_metal_layer_set_wants_extended_dynamic_range_content")
public func ca_metal_layer_set_wants_extended_dynamic_range_content(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.wantsExtendedDynamicRangeContent = value
}

@_cdecl("ca_metal_layer_get_preferred_device_registry_id")
public func ca_metal_layer_get_preferred_device_registry_id(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let layer: CAMetalLayer = caBorrow(handle), let device = layer.preferredDevice else { return 0 }
    return device.registryID
}

@_cdecl("ca_metal_layer_supports_developer_hud_properties")
public func ca_metal_layer_supports_developer_hud_properties() -> Bool {
    if #available(macOS 13.0, *) {
        return true
    }
    return false
}

@_cdecl("ca_metal_layer_copy_developer_hud_properties")
public func ca_metal_layer_copy_developer_hud_properties(
    _ handle: UnsafeMutableRawPointer?,
    _ outCount: UnsafeMutablePointer<Int>?
) -> UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>? {
    guard let outCount else { return nil }
    outCount.pointee = 0
    guard #available(macOS 13.0, *),
          let layer: CAMetalLayer = caBorrow(handle),
          let properties = layer.developerHUDProperties
    else {
        return nil
    }
    let strings = properties.flatMap { ["\($0.key)", "\($0.value)"] }
    let stride = MemoryLayout<UnsafeMutablePointer<CChar>?>.stride
    guard let raw = malloc(max(1, strings.count) * stride) else { return nil }
    let buffer = raw.bindMemory(to: UnsafeMutablePointer<CChar>?.self, capacity: max(1, strings.count))
    for (index, string) in strings.enumerated() {
        buffer[index] = strdup(string)
    }
    outCount.pointee = properties.count
    return buffer
}

@_cdecl("ca_metal_layer_set_developer_hud_properties")
public func ca_metal_layer_set_developer_hud_properties(
    _ handle: UnsafeMutableRawPointer?,
    _ keys: UnsafePointer<UnsafePointer<CChar>?>?,
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ present: Bool
) -> Bool {
    guard #available(macOS 13.0, *), let layer: CAMetalLayer = caBorrow(handle) else { return false }
    guard present else {
        layer.developerHUDProperties = nil
        return true
    }
    guard count >= 0 else { return false }
    var properties: [String: String] = [:]
    if count > 0 {
        guard let keys, let values else { return false }
        for index in 0..<count {
            guard let key = caCString(keys[index]), let value = caCString(values[index]) else { return false }
            properties[key] = value
        }
    }
    layer.developerHUDProperties = properties
    return true
}
