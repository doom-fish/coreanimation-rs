import Foundation
import QuartzCore

@_cdecl("ca_layer_get_z_position")
public func ca_layer_get_z_position(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.zPosition
}

@_cdecl("ca_layer_set_z_position")
public func ca_layer_set_z_position(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.zPosition = value
}

@_cdecl("ca_layer_get_anchor_point_z")
public func ca_layer_get_anchor_point_z(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.anchorPointZ
}

@_cdecl("ca_layer_set_anchor_point_z")
public func ca_layer_set_anchor_point_z(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.anchorPointZ = value
}

@_cdecl("ca_layer_get_sublayer_transform")
public func ca_layer_get_sublayer_transform(_ handle: UnsafeMutableRawPointer?, _ outTransform: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWriteTransform(layer.sublayerTransform, out: outTransform)
}

@_cdecl("ca_layer_set_sublayer_transform")
public func ca_layer_set_sublayer_transform(_ handle: UnsafeMutableRawPointer?, _ transformRaw: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle), let transform = caReadTransform(transformRaw) else { return }
    layer.sublayerTransform = transform
}

@_cdecl("ca_layer_get_double_sided")
public func ca_layer_get_double_sided(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return layer.isDoubleSided
}

@_cdecl("ca_layer_set_double_sided")
public func ca_layer_set_double_sided(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.isDoubleSided = value
}

@_cdecl("ca_layer_get_geometry_flipped")
public func ca_layer_get_geometry_flipped(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return layer.isGeometryFlipped
}

@_cdecl("ca_layer_set_geometry_flipped")
public func ca_layer_set_geometry_flipped(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.isGeometryFlipped = value
}

@_cdecl("ca_layer_supports_tone_map_mode")
public func ca_layer_supports_tone_map_mode() -> Bool {
    if #available(macOS 15.0, *) {
        return true
    }
    return false
}

@_cdecl("ca_layer_get_tone_map_mode")
public func ca_layer_get_tone_map_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard #available(macOS 15.0, *), let layer: CALayer = caBorrow(handle) else { return 0 }
    return caToneMapModeRaw(layer.toneMapMode)
}

@_cdecl("ca_layer_set_tone_map_mode")
public func ca_layer_set_tone_map_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard #available(macOS 15.0, *), let layer: CALayer = caBorrow(handle) else { return }
    layer.toneMapMode = caToneMapMode(value)
}
