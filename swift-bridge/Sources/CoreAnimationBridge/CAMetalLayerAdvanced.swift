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
