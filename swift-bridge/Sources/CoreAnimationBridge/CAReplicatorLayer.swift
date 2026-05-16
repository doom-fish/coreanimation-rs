import Foundation
import QuartzCore

@_cdecl("ca_replicator_layer_new")
public func ca_replicator_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAReplicatorLayer())
}

@_cdecl("ca_replicator_layer_get_instance_count")
public func ca_replicator_layer_get_instance_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceCount
}

@_cdecl("ca_replicator_layer_set_instance_count")
public func ca_replicator_layer_set_instance_count(_ handle: UnsafeMutableRawPointer?, _ value: Int) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceCount = value
}

@_cdecl("ca_replicator_layer_get_preserves_depth")
public func ca_replicator_layer_get_preserves_depth(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return false }
    return layer.preservesDepth
}

@_cdecl("ca_replicator_layer_set_preserves_depth")
public func ca_replicator_layer_set_preserves_depth(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.preservesDepth = value
}

@_cdecl("ca_replicator_layer_get_instance_delay")
public func ca_replicator_layer_get_instance_delay(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceDelay
}

@_cdecl("ca_replicator_layer_set_instance_delay")
public func ca_replicator_layer_set_instance_delay(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceDelay = value
}

@_cdecl("ca_replicator_layer_get_instance_transform")
public func ca_replicator_layer_get_instance_transform(_ handle: UnsafeMutableRawPointer?, _ outTransform: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return false }
    return caWriteTransform(layer.instanceTransform, out: outTransform)
}

@_cdecl("ca_replicator_layer_set_instance_transform")
public func ca_replicator_layer_set_instance_transform(_ handle: UnsafeMutableRawPointer?, _ transformRaw: UnsafeMutableRawPointer?) {
    guard let layer: CAReplicatorLayer = caBorrow(handle), let transform = caReadTransform(transformRaw) else { return }
    layer.instanceTransform = transform
}

@_cdecl("ca_replicator_layer_set_instance_color")
public func ca_replicator_layer_set_instance_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.instanceColor = color
}

@_cdecl("ca_replicator_layer_get_instance_color")
public func ca_replicator_layer_get_instance_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAReplicatorLayer = caBorrow(handle), let color = layer.instanceColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_replicator_layer_get_instance_red_offset")
public func ca_replicator_layer_get_instance_red_offset(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceRedOffset
}

@_cdecl("ca_replicator_layer_set_instance_red_offset")
public func ca_replicator_layer_set_instance_red_offset(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceRedOffset = value
}

@_cdecl("ca_replicator_layer_get_instance_green_offset")
public func ca_replicator_layer_get_instance_green_offset(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceGreenOffset
}

@_cdecl("ca_replicator_layer_set_instance_green_offset")
public func ca_replicator_layer_set_instance_green_offset(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceGreenOffset = value
}

@_cdecl("ca_replicator_layer_get_instance_blue_offset")
public func ca_replicator_layer_get_instance_blue_offset(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceBlueOffset
}

@_cdecl("ca_replicator_layer_set_instance_blue_offset")
public func ca_replicator_layer_set_instance_blue_offset(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceBlueOffset = value
}

@_cdecl("ca_replicator_layer_get_instance_alpha_offset")
public func ca_replicator_layer_get_instance_alpha_offset(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return 0 }
    return layer.instanceAlphaOffset
}

@_cdecl("ca_replicator_layer_set_instance_alpha_offset")
public func ca_replicator_layer_set_instance_alpha_offset(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAReplicatorLayer = caBorrow(handle) else { return }
    layer.instanceAlphaOffset = value
}
