import Foundation
import QuartzCore

@_cdecl("ca_metal_layer_get_framebuffer_only")
public func ca_metal_layer_get_framebuffer_only(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return layer.framebufferOnly
}

@_cdecl("ca_metal_layer_set_framebuffer_only")
public func ca_metal_layer_set_framebuffer_only(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.framebufferOnly = value
}

@_cdecl("ca_metal_layer_get_maximum_drawable_count")
public func ca_metal_layer_get_maximum_drawable_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return 0 }
    return Int(layer.maximumDrawableCount)
}

@_cdecl("ca_metal_layer_set_maximum_drawable_count")
public func ca_metal_layer_set_maximum_drawable_count(_ handle: UnsafeMutableRawPointer?, _ value: Int) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.maximumDrawableCount = max(1, value)
}

@_cdecl("ca_metal_layer_get_presents_with_transaction")
public func ca_metal_layer_get_presents_with_transaction(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return layer.presentsWithTransaction
}

@_cdecl("ca_metal_layer_set_presents_with_transaction")
public func ca_metal_layer_set_presents_with_transaction(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.presentsWithTransaction = value
}

@_cdecl("ca_metal_layer_get_display_sync_enabled")
public func ca_metal_layer_get_display_sync_enabled(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return layer.displaySyncEnabled
}

@_cdecl("ca_metal_layer_set_display_sync_enabled")
public func ca_metal_layer_set_display_sync_enabled(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.displaySyncEnabled = value
}

@_cdecl("ca_metal_layer_get_allows_next_drawable_timeout")
public func ca_metal_layer_get_allows_next_drawable_timeout(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return layer.allowsNextDrawableTimeout
}

@_cdecl("ca_metal_layer_set_allows_next_drawable_timeout")
public func ca_metal_layer_set_allows_next_drawable_timeout(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.allowsNextDrawableTimeout = value
}
