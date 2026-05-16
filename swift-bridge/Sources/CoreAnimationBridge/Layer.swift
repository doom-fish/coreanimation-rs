import CoreGraphics
import Foundation
import QuartzCore

@_cdecl("ca_layer_new")
public func ca_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CALayer())
}

@_cdecl("ca_layer_get_frame")
public func ca_layer_get_frame(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWriteRect(layer.frame, out: outRect)
}

@_cdecl("ca_layer_set_frame")
public func ca_layer_set_frame(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.frame = CGRect(x: x, y: y, width: width, height: height)
}

@_cdecl("ca_layer_get_bounds")
public func ca_layer_get_bounds(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWriteRect(layer.bounds, out: outRect)
}

@_cdecl("ca_layer_set_bounds")
public func ca_layer_set_bounds(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.bounds = CGRect(x: x, y: y, width: width, height: height)
}

@_cdecl("ca_layer_get_position")
public func ca_layer_get_position(_ handle: UnsafeMutableRawPointer?, _ outPoint: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWritePoint(layer.position, out: outPoint)
}

@_cdecl("ca_layer_set_position")
public func ca_layer_set_position(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.position = CGPoint(x: x, y: y)
}

@_cdecl("ca_layer_get_anchor_point")
public func ca_layer_get_anchor_point(_ handle: UnsafeMutableRawPointer?, _ outPoint: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWritePoint(layer.anchorPoint, out: outPoint)
}

@_cdecl("ca_layer_set_anchor_point")
public func ca_layer_set_anchor_point(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.anchorPoint = CGPoint(x: x, y: y)
}

@_cdecl("ca_layer_get_transform")
public func ca_layer_get_transform(_ handle: UnsafeMutableRawPointer?, _ outTransform: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWriteTransform(layer.transform, out: outTransform)
}

@_cdecl("ca_layer_set_transform")
public func ca_layer_set_transform(_ handle: UnsafeMutableRawPointer?, _ transformRaw: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle), let transform = caReadTransform(transformRaw) else { return }
    layer.transform = transform
}

@_cdecl("ca_layer_sublayer_count")
public func ca_layer_sublayer_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.sublayers?.count ?? 0
}

@_cdecl("ca_layer_sublayer_at")
public func ca_layer_sublayer_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let sublayers = layer.sublayers, index >= 0, index < sublayers.count else { return nil }
    return caRetain(sublayers[index])
}

@_cdecl("ca_layer_add_sublayer")
public func ca_layer_add_sublayer(_ handle: UnsafeMutableRawPointer?, _ childHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle), let child: CALayer = caBorrow(childHandle) else { return }
    layer.addSublayer(child)
}

@_cdecl("ca_layer_remove_from_superlayer")
public func ca_layer_remove_from_superlayer(_ handle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.removeFromSuperlayer()
}

@_cdecl("ca_layer_set_contents")
public func ca_layer_set_contents(_ handle: UnsafeMutableRawPointer?, _ imageHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let image: CGImage? = caBorrow(imageHandle)
    layer.contents = image
}

@_cdecl("ca_layer_get_contents")
public func ca_layer_get_contents(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let contents = layer.contents else { return nil }
    return caRetain(contents as AnyObject)
}

@_cdecl("ca_layer_get_contents_scale")
public func ca_layer_get_contents_scale(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 1.0 }
    return layer.contentsScale
}

@_cdecl("ca_layer_set_contents_scale")
public func ca_layer_set_contents_scale(_ handle: UnsafeMutableRawPointer?, _ scale: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.contentsScale = scale
}

@_cdecl("ca_layer_set_background_color")
public func ca_layer_set_background_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.backgroundColor = color
}

@_cdecl("ca_layer_get_background_color")
public func ca_layer_get_background_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let color = layer.backgroundColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_layer_set_border_color")
public func ca_layer_set_border_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.borderColor = color
}

@_cdecl("ca_layer_get_border_color")
public func ca_layer_get_border_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let color = layer.borderColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_layer_get_border_width")
public func ca_layer_get_border_width(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.borderWidth
}

@_cdecl("ca_layer_set_border_width")
public func ca_layer_set_border_width(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.borderWidth = value
}

@_cdecl("ca_layer_get_corner_radius")
public func ca_layer_get_corner_radius(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.cornerRadius
}

@_cdecl("ca_layer_set_corner_radius")
public func ca_layer_set_corner_radius(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.cornerRadius = value
}

@_cdecl("ca_layer_get_opacity")
public func ca_layer_get_opacity(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.opacity
}

@_cdecl("ca_layer_set_opacity")
public func ca_layer_set_opacity(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.opacity = value
}

@_cdecl("ca_layer_is_hidden")
public func ca_layer_is_hidden(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return layer.isHidden
}

@_cdecl("ca_layer_set_hidden")
public func ca_layer_set_hidden(_ handle: UnsafeMutableRawPointer?, _ hidden: Bool) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.isHidden = hidden
}

@_cdecl("ca_layer_set_mask")
public func ca_layer_set_mask(_ handle: UnsafeMutableRawPointer?, _ maskHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let mask: CALayer? = caBorrow(maskHandle)
    layer.mask = mask
}

@_cdecl("ca_layer_get_mask")
public func ca_layer_get_mask(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let mask = layer.mask else { return nil }
    return caRetain(mask)
}

@_cdecl("ca_layer_get_masks_to_bounds")
public func ca_layer_get_masks_to_bounds(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return layer.masksToBounds
}

@_cdecl("ca_layer_set_masks_to_bounds")
public func ca_layer_set_masks_to_bounds(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.masksToBounds = value
}

@_cdecl("ca_layer_get_shadow_offset")
public func ca_layer_get_shadow_offset(_ handle: UnsafeMutableRawPointer?, _ outSize: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CALayer = caBorrow(handle) else { return false }
    return caWriteSize(layer.shadowOffset, out: outSize)
}

@_cdecl("ca_layer_set_shadow_offset")
public func ca_layer_set_shadow_offset(_ handle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.shadowOffset = CGSize(width: width, height: height)
}

@_cdecl("ca_layer_get_shadow_radius")
public func ca_layer_get_shadow_radius(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.shadowRadius
}

@_cdecl("ca_layer_set_shadow_radius")
public func ca_layer_set_shadow_radius(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.shadowRadius = value
}

@_cdecl("ca_layer_get_shadow_opacity")
public func ca_layer_get_shadow_opacity(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.shadowOpacity
}

@_cdecl("ca_layer_set_shadow_opacity")
public func ca_layer_set_shadow_opacity(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.shadowOpacity = value
}

@_cdecl("ca_layer_set_shadow_color")
public func ca_layer_set_shadow_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    layer.shadowColor = color
}

@_cdecl("ca_layer_get_shadow_color")
public func ca_layer_get_shadow_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let color = layer.shadowColor else { return nil }
    return caRetain(color)
}

@_cdecl("ca_layer_get_contents_gravity")
public func ca_layer_get_contents_gravity(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CALayer = caBorrow(handle) else { return 9 }
    return caContentsGravityRaw(layer.contentsGravity)
}

@_cdecl("ca_layer_set_contents_gravity")
public func ca_layer_set_contents_gravity(_ handle: UnsafeMutableRawPointer?, _ gravity: Int32) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.contentsGravity = caContentsGravity(gravity)
}

@_cdecl("ca_layer_add_animation")
public func ca_layer_add_animation(_ handle: UnsafeMutableRawPointer?, _ animationHandle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) {
    guard let layer: CALayer = caBorrow(handle), let animation: CAAnimation = caBorrow(animationHandle) else { return }
    layer.add(animation, forKey: caCString(key))
}

@_cdecl("ca_layer_remove_animation")
public func ca_layer_remove_animation(_ handle: UnsafeMutableRawPointer?, _ key: UnsafePointer<CChar>?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    if let key = caCString(key) {
        layer.removeAnimation(forKey: key)
    } else {
        layer.removeAllAnimations()
    }
}
