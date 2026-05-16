import Foundation
import QuartzCore

@_cdecl("ca_emitter_layer_get_emitter_z_position")
public func ca_emitter_layer_get_emitter_z_position(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.emitterZPosition
}

@_cdecl("ca_emitter_layer_set_emitter_z_position")
public func ca_emitter_layer_set_emitter_z_position(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterZPosition = value
}

@_cdecl("ca_emitter_layer_get_emitter_depth")
public func ca_emitter_layer_get_emitter_depth(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.emitterDepth
}

@_cdecl("ca_emitter_layer_set_emitter_depth")
public func ca_emitter_layer_set_emitter_depth(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterDepth = value
}

@_cdecl("ca_emitter_layer_get_preserves_depth")
public func ca_emitter_layer_get_preserves_depth(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return false }
    return layer.preservesDepth
}

@_cdecl("ca_emitter_layer_set_preserves_depth")
public func ca_emitter_layer_set_preserves_depth(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.preservesDepth = value
}

@_cdecl("ca_emitter_layer_get_spin")
public func ca_emitter_layer_get_spin(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.spin
}

@_cdecl("ca_emitter_layer_set_spin")
public func ca_emitter_layer_set_spin(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.spin = value
}

@_cdecl("ca_emitter_layer_get_seed")
public func ca_emitter_layer_get_seed(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.seed
}

@_cdecl("ca_emitter_layer_set_seed")
public func ca_emitter_layer_set_seed(_ handle: UnsafeMutableRawPointer?, _ value: UInt32) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.seed = value
}
