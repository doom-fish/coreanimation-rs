import CoreGraphics
import CoreVideo
import Foundation
import Metal
import QuartzCore

@_cdecl("ca_renderer_new")
public func ca_renderer_new(_ textureHandle: UnsafeMutableRawPointer?, _ queueHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let texture: MTLTexture = caBorrow(textureHandle) else { return nil }
    let queue: MTLCommandQueue? = caBorrow(queueHandle)
    let options: [AnyHashable: Any]? = queue.map { [kCARendererMetalCommandQueue: $0] }
    return caRetain(CARenderer(mtlTexture: texture, options: options))
}

@_cdecl("ca_renderer_set_layer")
public func ca_renderer_set_layer(_ handle: UnsafeMutableRawPointer?, _ layerHandle: UnsafeMutableRawPointer?) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    let layer: CALayer? = caBorrow(layerHandle)
    renderer.layer = layer
}

@_cdecl("ca_renderer_get_bounds")
public func ca_renderer_get_bounds(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer: CARenderer = caBorrow(handle) else { return false }
    return caWriteRect(renderer.bounds, out: outRect)
}

@_cdecl("ca_renderer_set_bounds")
public func ca_renderer_set_bounds(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    renderer.bounds = CGRect(x: x, y: y, width: width, height: height)
}

@_cdecl("ca_renderer_begin_frame")
public func ca_renderer_begin_frame(_ handle: UnsafeMutableRawPointer?, _ time: Double, _ timestamp: UnsafeMutableRawPointer?) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    if let timestamp {
        renderer.beginFrame(atTime: time, timeStamp: timestamp.assumingMemoryBound(to: CVTimeStamp.self))
    } else {
        renderer.beginFrame(atTime: time, timeStamp: nil)
    }
}

@_cdecl("ca_renderer_update_bounds")
public func ca_renderer_update_bounds(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let renderer: CARenderer = caBorrow(handle) else { return false }
    return caWriteRect(renderer.updateBounds(), out: outRect)
}

@_cdecl("ca_renderer_render")
public func ca_renderer_render(_ handle: UnsafeMutableRawPointer?) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    renderer.render()
}

@_cdecl("ca_renderer_end_frame")
public func ca_renderer_end_frame(_ handle: UnsafeMutableRawPointer?) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    renderer.endFrame()
}

@_cdecl("ca_renderer_next_frame_time")
public func ca_renderer_next_frame_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let renderer: CARenderer = caBorrow(handle) else { return .infinity }
    return renderer.nextFrameTime()
}

@_cdecl("ca_renderer_set_destination")
public func ca_renderer_set_destination(_ handle: UnsafeMutableRawPointer?, _ textureHandle: UnsafeMutableRawPointer?) {
    guard let renderer: CARenderer = caBorrow(handle), let texture: MTLTexture = caBorrow(textureHandle) else { return }
    renderer.setDestination(texture)
}

@_cdecl("ca_renderer_render_at_time")
public func ca_renderer_render_at_time(_ handle: UnsafeMutableRawPointer?, _ time: Double) {
    guard let renderer: CARenderer = caBorrow(handle) else { return }
    CATransaction.flush()
    renderer.beginFrame(atTime: time, timeStamp: nil)
    renderer.render()
    renderer.endFrame()
}

@_cdecl("ca_texture_copy_bytes")
public func ca_texture_copy_bytes(_ textureHandle: UnsafeMutableRawPointer?, _ outBytes: UnsafeMutableRawPointer?, _ bytesPerRow: Int) -> Bool {
    guard let texture: MTLTexture = caBorrow(textureHandle), let outBytes else { return false }
    let region = MTLRegionMake2D(0, 0, texture.width, texture.height)
    texture.getBytes(outBytes, bytesPerRow: bytesPerRow, from: region, mipmapLevel: 0)
    return true
}
