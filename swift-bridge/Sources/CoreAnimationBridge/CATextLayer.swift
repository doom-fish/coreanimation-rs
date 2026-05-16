import Foundation
import QuartzCore

@_cdecl("ca_text_layer_get_wrapped")
public func ca_text_layer_get_wrapped(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CATextLayer = caBorrow(handle) else { return false }
    return layer.isWrapped
}

@_cdecl("ca_text_layer_set_wrapped")
public func ca_text_layer_set_wrapped(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.isWrapped = value
}

@_cdecl("ca_text_layer_get_allows_font_subpixel_quantization")
public func ca_text_layer_get_allows_font_subpixel_quantization(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CATextLayer = caBorrow(handle) else { return false }
    return layer.allowsFontSubpixelQuantization
}

@_cdecl("ca_text_layer_set_allows_font_subpixel_quantization")
public func ca_text_layer_set_allows_font_subpixel_quantization(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CATextLayer = caBorrow(handle) else { return }
    layer.allowsFontSubpixelQuantization = value
}
