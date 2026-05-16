import Foundation
import QuartzCore

private func caShapeFillRule(_ raw: Int32) -> CAShapeLayerFillRule {
    raw == 1 ? .evenOdd : .nonZero
}

private func caShapeFillRuleRaw(_ value: CAShapeLayerFillRule) -> Int32 {
    value == .evenOdd ? 1 : 0
}

@_cdecl("ca_shape_layer_get_fill_rule")
public func ca_shape_layer_get_fill_rule(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return caShapeFillRuleRaw(layer.fillRule)
}

@_cdecl("ca_shape_layer_set_fill_rule")
public func ca_shape_layer_set_fill_rule(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.fillRule = caShapeFillRule(value)
}

@_cdecl("ca_shape_layer_get_stroke_start")
public func ca_shape_layer_get_stroke_start(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.strokeStart
}

@_cdecl("ca_shape_layer_set_stroke_start")
public func ca_shape_layer_set_stroke_start(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.strokeStart = value
}

@_cdecl("ca_shape_layer_get_stroke_end")
public func ca_shape_layer_get_stroke_end(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.strokeEnd
}

@_cdecl("ca_shape_layer_set_stroke_end")
public func ca_shape_layer_set_stroke_end(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.strokeEnd = value
}

@_cdecl("ca_shape_layer_get_line_dash_phase")
public func ca_shape_layer_get_line_dash_phase(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.lineDashPhase
}

@_cdecl("ca_shape_layer_set_line_dash_phase")
public func ca_shape_layer_set_line_dash_phase(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.lineDashPhase = value
}
