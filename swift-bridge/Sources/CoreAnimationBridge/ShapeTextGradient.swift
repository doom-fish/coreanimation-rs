import CoreGraphics
import Foundation
import QuartzCore

@_cdecl("ca_shape_layer_new")
public func ca_shape_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAShapeLayer())
}

@_cdecl("ca_shape_layer_set_path")
public func ca_shape_layer_set_path(_ handle: UnsafeMutableRawPointer?, _ pathHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    let path: CGPath? = caBorrow(pathHandle)
    layer.path = path
}

@_cdecl("ca_shape_layer_get_path")
public func ca_shape_layer_get_path(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAShapeLayer = caBorrow(handle), let path = layer.path else { return nil }
    return caRetain(path)
}

@_cdecl("ca_shape_layer_set_fill_color")
public func ca_shape_layer_set_fill_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.fillColor = color
}

@_cdecl("ca_shape_layer_get_fill_color")
public func ca_shape_layer_get_fill_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAShapeLayer = caBorrow(handle), let color = layer.fillColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_shape_layer_set_stroke_color")
public func ca_shape_layer_set_stroke_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.strokeColor = color
}

@_cdecl("ca_shape_layer_get_stroke_color")
public func ca_shape_layer_get_stroke_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAShapeLayer = caBorrow(handle), let color = layer.strokeColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_shape_layer_get_line_width")
public func ca_shape_layer_get_line_width(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.lineWidth
}

@_cdecl("ca_shape_layer_set_line_width")
public func ca_shape_layer_set_line_width(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.lineWidth = value
}

@_cdecl("ca_shape_layer_get_line_cap")
public func ca_shape_layer_get_line_cap(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return caLineCapRaw(layer.lineCap)
}

@_cdecl("ca_shape_layer_set_line_cap")
public func ca_shape_layer_set_line_cap(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.lineCap = caLineCap(value)
}

@_cdecl("ca_shape_layer_get_line_join")
public func ca_shape_layer_get_line_join(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return caLineJoinRaw(layer.lineJoin)
}

@_cdecl("ca_shape_layer_set_line_join")
public func ca_shape_layer_set_line_join(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.lineJoin = caLineJoin(value)
}

@_cdecl("ca_shape_layer_set_line_dash_pattern")
public func ca_shape_layer_set_line_dash_pattern(_ handle: UnsafeMutableRawPointer?, _ pattern: UnsafePointer<Double>?, _ length: Int) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    guard let pattern, length > 0 else {
        layer.lineDashPattern = nil
        return
    }
    layer.lineDashPattern = UnsafeBufferPointer(start: pattern, count: length).map { NSNumber(value: $0) }
}

@_cdecl("ca_shape_layer_line_dash_pattern_count")
public func ca_shape_layer_line_dash_pattern_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.lineDashPattern?.count ?? 0
}

@_cdecl("ca_shape_layer_line_dash_pattern_at")
public func ca_shape_layer_line_dash_pattern_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle), let pattern = layer.lineDashPattern, index >= 0, index < pattern.count else { return 0 }
    return pattern[index].doubleValue
}

@_cdecl("ca_shape_layer_get_miter_limit")
public func ca_shape_layer_get_miter_limit(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return 0 }
    return layer.miterLimit
}

@_cdecl("ca_shape_layer_set_miter_limit")
public func ca_shape_layer_set_miter_limit(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CAShapeLayer = caBorrow(handle) else { return }
    layer.miterLimit = value
}

@_cdecl("ca_text_layer_new")
public func ca_text_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CATextLayer())
}

@_cdecl("ca_text_layer_set_string")
public func ca_text_layer_set_string(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.string = caCString(value)
}

@_cdecl("ca_text_layer_get_string")
public func ca_text_layer_get_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let layer: CATextLayer = caBorrow(handle) else { return nil }
    if let string = layer.string as? String {
        return caDup(string)
    }
    return caDup((layer.string as AnyObject?)?.description ?? layer.string.map(String.init(describing:)))
}

@_cdecl("ca_text_layer_set_font_name")
public func ca_text_layer_set_font_name(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.font = caCString(value) as CFString?
}

@_cdecl("ca_text_layer_get_font_name")
public func ca_text_layer_get_font_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let layer: CATextLayer = caBorrow(handle) else { return nil }
    if let font = layer.font as? String {
        return caDup(font)
    }
    return caDup((layer.font as AnyObject?)?.description)
}

@_cdecl("ca_text_layer_get_font_size")
public func ca_text_layer_get_font_size(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CATextLayer = caBorrow(handle) else { return 0 }
    return layer.fontSize
}

@_cdecl("ca_text_layer_set_font_size")
public func ca_text_layer_set_font_size(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.fontSize = value
}

@_cdecl("ca_text_layer_set_foreground_color")
public func ca_text_layer_set_foreground_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.foregroundColor = color
}

@_cdecl("ca_text_layer_get_foreground_color")
public func ca_text_layer_get_foreground_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CATextLayer = caBorrow(handle), let color = layer.foregroundColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_text_layer_get_alignment_mode")
public func ca_text_layer_get_alignment_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CATextLayer = caBorrow(handle) else { return 0 }
    return caTextAlignmentRaw(layer.alignmentMode)
}

@_cdecl("ca_text_layer_set_alignment_mode")
public func ca_text_layer_set_alignment_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.alignmentMode = caTextAlignment(value)
}

@_cdecl("ca_text_layer_get_truncation_mode")
public func ca_text_layer_get_truncation_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CATextLayer = caBorrow(handle) else { return 0 }
    return caTextTruncationRaw(layer.truncationMode)
}

@_cdecl("ca_text_layer_set_truncation_mode")
public func ca_text_layer_set_truncation_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.truncationMode = caTextTruncation(value)
}

@_cdecl("ca_gradient_layer_new")
public func ca_gradient_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAGradientLayer())
}

@_cdecl("ca_gradient_layer_set_colors")
public func ca_gradient_layer_set_colors(_ handle: UnsafeMutableRawPointer?, _ colors: UnsafePointer<UnsafeMutableRawPointer?>?, _ count: Int) {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return }
    guard let colors, count > 0 else {
        layer.colors = nil
        return
    }
    layer.colors = (0..<count).compactMap { index in
        let handle = colors[index]
        let color: CGColor? = caBorrow(handle)
        return color
    }
}

@_cdecl("ca_gradient_layer_color_count")
public func ca_gradient_layer_color_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return 0 }
    return layer.colors?.count ?? 0
}

@_cdecl("ca_gradient_layer_color_at")
public func ca_gradient_layer_color_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let layer: CAGradientLayer = caBorrow(handle), let colors = layer.colors, index >= 0, index < colors.count else { return nil }
    return caRetain(colors[index] as AnyObject)
}

@_cdecl("ca_gradient_layer_set_locations")
public func ca_gradient_layer_set_locations(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return }
    guard let values, count > 0 else {
        layer.locations = nil
        return
    }
    layer.locations = UnsafeBufferPointer(start: values, count: count).map { NSNumber(value: $0) }
}

@_cdecl("ca_gradient_layer_location_count")
public func ca_gradient_layer_location_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return 0 }
    return layer.locations?.count ?? 0
}

@_cdecl("ca_gradient_layer_location_at")
public func ca_gradient_layer_location_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let layer: CAGradientLayer = caBorrow(handle), let locations = layer.locations, index >= 0, index < locations.count else { return 0 }
    return locations[index].doubleValue
}

@_cdecl("ca_gradient_layer_get_start_point")
public func ca_gradient_layer_get_start_point(_ handle: UnsafeMutableRawPointer?, _ outPoint: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return false }
    return caWritePoint(layer.startPoint, out: outPoint)
}

@_cdecl("ca_gradient_layer_set_start_point")
public func ca_gradient_layer_set_start_point(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return }
    layer.startPoint = CGPoint(x: x, y: y)
}

@_cdecl("ca_gradient_layer_get_end_point")
public func ca_gradient_layer_get_end_point(_ handle: UnsafeMutableRawPointer?, _ outPoint: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return false }
    return caWritePoint(layer.endPoint, out: outPoint)
}

@_cdecl("ca_gradient_layer_set_end_point")
public func ca_gradient_layer_set_end_point(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return }
    layer.endPoint = CGPoint(x: x, y: y)
}

@_cdecl("ca_gradient_layer_get_type")
public func ca_gradient_layer_get_type(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return 0 }
    return caGradientTypeRaw(layer.type)
}

@_cdecl("ca_gradient_layer_set_type")
public func ca_gradient_layer_set_type(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAGradientLayer = caBorrow(handle) else { return }
    layer.type = caGradientType(value)
}
