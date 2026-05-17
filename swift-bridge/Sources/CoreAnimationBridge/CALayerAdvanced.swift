import Foundation
import QuartzCore

private func caBorrowActionObject(_ handle: UnsafeMutableRawPointer?) -> AnyObject? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
}

private func caBorrowObject(_ handle: UnsafeMutableRawPointer?) -> AnyObject? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
}

public typealias CALayerDisplayCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias CALayerLayoutCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

public typealias CALayerActionCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer?

final class CALayerDelegateBox: NSObject, CALayerDelegate {
    var displayCallback: CALayerDisplayCallback?
    var displayContext: UnsafeMutableRawPointer?
    var layoutCallback: CALayerLayoutCallback?
    var layoutContext: UnsafeMutableRawPointer?
    var actionCallback: CALayerActionCallback?
    var actionContext: UnsafeMutableRawPointer?

    func display(_ layer: CALayer) {
        displayCallback?(displayContext, caRetain(layer))
    }

    func layoutSublayers(of layer: CALayer) {
        layoutCallback?(layoutContext, caRetain(layer))
    }

    func action(for layer: CALayer, forKey event: String) -> CAAction? {
        guard let actionCallback else { return nil }
        let handle = event.withCString { cString in
            actionCallback(actionContext, caRetain(layer), cString)
        }
        guard let handle, let action = caBorrowActionObject(handle) as? CAAction else { return nil }
        caReleaseHandle(handle)
        return action
    }
}

@_cdecl("ca_layer_delegate_new")
public func ca_layer_delegate_new() -> UnsafeMutableRawPointer? {
    caRetain(CALayerDelegateBox())
}

@_cdecl("ca_action_null")
public func ca_action_null() -> UnsafeMutableRawPointer? {
    caRetain(NSNull())
}

@_cdecl("ca_action_run_for_key")
public func ca_action_run_for_key(
    _ handle: UnsafeMutableRawPointer?,
    _ event: UnsafePointer<CChar>?,
    _ objectHandle: UnsafeMutableRawPointer?
) {
    guard let action = caBorrowActionObject(handle) as? CAAction,
          let event = caCString(event),
          let object = caBorrowObject(objectHandle)
    else {
        return
    }
    action.run(forKey: event, object: object, arguments: nil)
}

@_cdecl("ca_layer_delegate_set_display_callback")
public func ca_layer_delegate_set_display_callback(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: CALayerDisplayCallback?,
    _ context: UnsafeMutableRawPointer?
) {
    guard let delegate: CALayerDelegateBox = caBorrow(handle) else { return }
    delegate.displayCallback = callback
    delegate.displayContext = context
}

@_cdecl("ca_layer_delegate_set_layout_callback")
public func ca_layer_delegate_set_layout_callback(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: CALayerLayoutCallback?,
    _ context: UnsafeMutableRawPointer?
) {
    guard let delegate: CALayerDelegateBox = caBorrow(handle) else { return }
    delegate.layoutCallback = callback
    delegate.layoutContext = context
}

@_cdecl("ca_layer_delegate_set_action_callback")
public func ca_layer_delegate_set_action_callback(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: CALayerActionCallback?,
    _ context: UnsafeMutableRawPointer?
) {
    guard let delegate: CALayerDelegateBox = caBorrow(handle) else { return }
    delegate.actionCallback = callback
    delegate.actionContext = context
}

@_cdecl("ca_layer_set_delegate")
public func ca_layer_set_delegate(_ handle: UnsafeMutableRawPointer?, _ delegateHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let delegate: CALayerDelegateBox? = caBorrow(delegateHandle)
    layer.delegate = delegate
}

@_cdecl("ca_layer_supports_preferred_dynamic_range")
public func ca_layer_supports_preferred_dynamic_range() -> Bool {
    if #available(macOS 26.0, *) {
        return true
    }
    return false
}

@_cdecl("ca_layer_get_preferred_dynamic_range")
public func ca_layer_get_preferred_dynamic_range(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard #available(macOS 26.0, *), let layer: CALayer = caBorrow(handle) else { return 0 }
    return caDynamicRangeRaw(layer.preferredDynamicRange)
}

@_cdecl("ca_layer_set_preferred_dynamic_range")
public func ca_layer_set_preferred_dynamic_range(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard #available(macOS 26.0, *), let layer: CALayer = caBorrow(handle) else { return }
    layer.preferredDynamicRange = caDynamicRange(value)
}

@_cdecl("ca_layer_get_contents_format")
public func ca_layer_get_contents_format(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return caContentsFormatRaw(layer.contentsFormat)
}

@_cdecl("ca_layer_set_contents_format")
public func ca_layer_set_contents_format(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.contentsFormat = caContentsFormat(value)
}

@_cdecl("ca_layer_get_minification_filter")
public func ca_layer_get_minification_filter(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return caContentsFilterRaw(layer.minificationFilter)
}

@_cdecl("ca_layer_set_minification_filter")
public func ca_layer_set_minification_filter(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.minificationFilter = caContentsFilter(value)
}

@_cdecl("ca_layer_get_magnification_filter")
public func ca_layer_get_magnification_filter(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return caContentsFilterRaw(layer.magnificationFilter)
}

@_cdecl("ca_layer_set_magnification_filter")
public func ca_layer_set_magnification_filter(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.magnificationFilter = caContentsFilter(value)
}

@_cdecl("ca_layer_get_edge_antialiasing_mask")
public func ca_layer_get_edge_antialiasing_mask(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.edgeAntialiasingMask.rawValue
}

@_cdecl("ca_layer_set_edge_antialiasing_mask")
public func ca_layer_set_edge_antialiasing_mask(_ handle: UnsafeMutableRawPointer?, _ value: UInt32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.edgeAntialiasingMask = CAEdgeAntialiasingMask(rawValue: value)
}

@_cdecl("ca_layer_get_masked_corners")
public func ca_layer_get_masked_corners(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return UInt64(layer.maskedCorners.rawValue)
}

@_cdecl("ca_layer_set_masked_corners")
public func ca_layer_set_masked_corners(_ handle: UnsafeMutableRawPointer?, _ value: UInt64) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.maskedCorners = CACornerMask(rawValue: UInt(value))
}

@_cdecl("ca_layer_get_corner_curve")
public func ca_layer_get_corner_curve(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return caCornerCurveRaw(layer.cornerCurve)
}

@_cdecl("ca_layer_set_corner_curve")
public func ca_layer_set_corner_curve(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.cornerCurve = caCornerCurve(value)
}

@_cdecl("ca_layer_corner_curve_expansion_factor")
public func ca_layer_corner_curve_expansion_factor(_ value: Int32) -> Double {
    CALayer.cornerCurveExpansionFactor(caCornerCurve(value))
}

@_cdecl("ca_layer_get_autoresizing_mask")
public func ca_layer_get_autoresizing_mask(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.autoresizingMask.rawValue
}

@_cdecl("ca_layer_set_autoresizing_mask")
public func ca_layer_set_autoresizing_mask(_ handle: UnsafeMutableRawPointer?, _ value: UInt32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.autoresizingMask = CAAutoresizingMask(rawValue: value)
}

@_cdecl("ca_layer_get_name")
public func ca_layer_get_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let layer: CALayer = caBorrow(handle) else { return nil }
    return caDup(layer.name)
}

@_cdecl("ca_layer_set_name")
public func ca_layer_set_name(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.name = caCString(value)
}

@_cdecl("ca_layer_display")
public func ca_layer_display(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.display()
}

@_cdecl("ca_layer_set_needs_display")
public func ca_layer_set_needs_display(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.setNeedsDisplay()
}

@_cdecl("ca_layer_display_if_needed")
public func ca_layer_display_if_needed(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.displayIfNeeded()
}

@_cdecl("ca_layer_set_needs_layout")
public func ca_layer_set_needs_layout(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.setNeedsLayout()
}

@_cdecl("ca_layer_layout_if_needed")
public func ca_layer_layout_if_needed(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.layoutIfNeeded()
}

@_cdecl("ca_layer_layout_sublayers")
public func ca_layer_layout_sublayers(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.layoutSublayers()
}

@_cdecl("ca_layer_default_action_for_key")
public func ca_layer_default_action_for_key(_ key: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let key = caCString(key), let action = CALayer.defaultAction(forKey: key) as? CAAnimation else {
        return nil
    }
    return caRetain(action)
}

@_cdecl("ca_layer_default_action_handle_for_key")
public func ca_layer_default_action_handle_for_key(_ key: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let key = caCString(key), let action = CALayer.defaultAction(forKey: key) else {
        return nil
    }
    return caRetain(action as AnyObject)
}

@_cdecl("ca_layer_action_for_key")
public func ca_layer_action_for_key(_ handle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let key = caCString(key), let action = layer.action(forKey: key) as? CAAnimation else {
        return nil
    }
    return caRetain(action)
}

@_cdecl("ca_layer_action_handle_for_key")
public func ca_layer_action_handle_for_key(_ handle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let key = caCString(key), let action = layer.action(forKey: key) else {
        return nil
    }
    return caRetain(action as AnyObject)
}

@_cdecl("ca_layer_set_action_for_key")
public func ca_layer_set_action_for_key(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ actionHandle: UnsafeMutableRawPointer?
) {
    guard let layer: CALayer = caBorrow(handle), let key = caCString(key) else { return }
    var actions = layer.actions ?? [:]
    let action = caBorrowActionObject(actionHandle) as? CAAction
    if let action {
        actions[key] = action
        layer.actions = actions
    } else {
        actions.removeValue(forKey: key)
        layer.actions = actions.isEmpty ? nil : actions
    }
}
