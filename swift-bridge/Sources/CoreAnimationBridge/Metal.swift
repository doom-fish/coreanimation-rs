import CoreAnimationObjCBridge
import CoreGraphics
import Foundation
import Metal
import QuartzCore

@_cdecl("ca_metal_layer_new")
public func ca_metal_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAMetalLayer())
}

@_cdecl("ca_metal_layer_set_device")
public func ca_metal_layer_set_device(_ handle: UnsafeMutableRawPointer?, _ deviceHandle: UnsafeMutableRawPointer?) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    let device: MTLDevice? = caBorrow(deviceHandle)
    layer.device = device
}

@_cdecl("ca_metal_layer_get_pixel_format")
public func ca_metal_layer_get_pixel_format(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return 0 }
    return Int(layer.pixelFormat.rawValue)
}

@_cdecl("ca_metal_layer_set_pixel_format")
public func ca_metal_layer_set_pixel_format(
    _ handle: UnsafeMutableRawPointer?,
    _ pixelFormat: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle),
          let raw = UInt(exactly: pixelFormat),
          let format = MTLPixelFormat(rawValue: raw)
    else {
        return false
    }
    var reason: NSString?
    let accepted = ca_objc_set_pixel_format(layer, format, &reason)
    if !accepted {
        outError?.pointee = caDup(reason as String?)
    }
    return accepted
}

@_cdecl("ca_metal_layer_get_drawable_size")
public func ca_metal_layer_get_drawable_size(_ handle: UnsafeMutableRawPointer?, _ outSize: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return false }
    return caWriteSize(layer.drawableSize, out: outSize)
}

@_cdecl("ca_metal_layer_set_drawable_size")
public func ca_metal_layer_set_drawable_size(_ handle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let layer: CAMetalLayer = caBorrow(handle) else { return }
    layer.drawableSize = CGSize(width: width, height: height)
}

@_cdecl("ca_metal_layer_next_drawable")
public func ca_metal_layer_next_drawable(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CAMetalLayer = caBorrow(handle), let drawable = layer.nextDrawable() else { return nil }
    return caRetain(drawable as AnyObject)
}

@_cdecl("ca_metal_drawable_get_texture")
public func ca_metal_drawable_get_texture(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let drawable: CAMetalDrawable = caBorrow(handle) else { return nil }
    return caRetain(drawable.texture as AnyObject)
}

@_cdecl("ca_metal_drawable_present")
public func ca_metal_drawable_present(_ handle: UnsafeMutableRawPointer?) {
    guard let drawable: CAMetalDrawable = caBorrow(handle) else { return }
    drawable.present()
}

@_cdecl("ca_metal_drawable_present_in_command_buffer")
public func ca_metal_drawable_present_in_command_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ commandBufferHandle: UnsafeMutableRawPointer?,
    _ hasTime: Bool,
    _ time: Double
) {
    guard let drawable: CAMetalDrawable = caBorrow(handle),
          let commandBuffer: MTLCommandBuffer = caBorrow(commandBufferHandle)
    else { return }
    if hasTime {
        commandBuffer.present(drawable, atTime: time)
    } else {
        commandBuffer.present(drawable)
    }
}

@_cdecl("ca_metal_drawable_presented_time")
public func ca_metal_drawable_presented_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let drawable: CAMetalDrawable = caBorrow(handle) else { return 0 }
    return drawable.presentedTime
}

@_cdecl("ca_metal_drawable_add_presented_handler")
public func ca_metal_drawable_add_presented_handler(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: (@convention(c) (UnsafeMutableRawPointer?, Double) -> Void)?,
    _ context: UnsafeMutableRawPointer?,
    _ release: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
) {
    guard let owner = CAContextOwner(context, release) else { return }
    guard let drawable: CAMetalDrawable = caBorrow(handle), let callback else { return }
    drawable.addPresentedHandler { presented in
        callback(owner.context, presented.presentedTime)
    }
}
