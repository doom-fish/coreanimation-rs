import Foundation
import QuartzCore

private func caScrollMode(_ raw: Int32) -> CAScrollLayerScrollMode {
    switch raw {
    case 1: return .vertically
    case 2: return .horizontally
    case 3: return .both
    default: return .none
    }
}

private func caScrollModeRaw(_ value: CAScrollLayerScrollMode) -> Int32 {
    switch value {
    case .vertically: return 1
    case .horizontally: return 2
    case .both: return 3
    default: return 0
    }
}

@_cdecl("ca_scroll_layer_new")
public func ca_scroll_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAScrollLayer())
}

@_cdecl("ca_scroll_layer_get_scroll_mode")
public func ca_scroll_layer_get_scroll_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAScrollLayer = caBorrow(handle) else { return 0 }
    return caScrollModeRaw(layer.scrollMode)
}

@_cdecl("ca_scroll_layer_set_scroll_mode")
public func ca_scroll_layer_set_scroll_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAScrollLayer = caBorrow(handle) else { return }
    layer.scrollMode = caScrollMode(value)
}

@_cdecl("ca_scroll_layer_get_visible_rect")
public func ca_scroll_layer_get_visible_rect(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAScrollLayer = caBorrow(handle) else { return false }
    return caWriteRect(layer.visibleRect, out: outRect)
}

@_cdecl("ca_scroll_layer_scroll_to_point")
public func ca_scroll_layer_scroll_to_point(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CAScrollLayer = caBorrow(handle) else { return }
    var bounds = layer.bounds
    bounds.origin = CGPoint(x: x, y: y)
    layer.bounds = bounds
}

@_cdecl("ca_scroll_layer_scroll_to_rect")
public func ca_scroll_layer_scroll_to_rect(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let layer: CAScrollLayer = caBorrow(handle) else { return }
    layer.bounds = CGRect(x: x, y: y, width: width, height: height)
}
