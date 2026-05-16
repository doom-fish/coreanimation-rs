import CoreGraphics
import Foundation
import QuartzCore

@_cdecl("ca_emitter_layer_new")
public func ca_emitter_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CAEmitterLayer())
}

@_cdecl("ca_emitter_layer_set_emitter_cells")
public func ca_emitter_layer_set_emitter_cells(_ handle: UnsafeMutableRawPointer?, _ cells: UnsafePointer<UnsafeMutableRawPointer?>?, _ count: Int) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    guard let cells, count > 0 else {
        layer.emitterCells = nil
        return
    }
    layer.emitterCells = (0..<count).compactMap { index in
        let handle = cells[index]
        let cell: CAEmitterCell? = caBorrow(handle)
        return cell
    }
}

@_cdecl("ca_emitter_layer_emitter_cell_count")
public func ca_emitter_layer_emitter_cell_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.emitterCells?.count ?? 0
}

@_cdecl("ca_emitter_layer_emitter_cell_at")
public func ca_emitter_layer_emitter_cell_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let layer: CAEmitterLayer = caBorrow(handle), let cells = layer.emitterCells, index >= 0, index < cells.count else { return nil }
    return caRetain(cells[index])
}

@_cdecl("ca_emitter_layer_get_birth_rate")
public func ca_emitter_layer_get_birth_rate(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.birthRate
}

@_cdecl("ca_emitter_layer_set_birth_rate")
public func ca_emitter_layer_set_birth_rate(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.birthRate = value
}

@_cdecl("ca_emitter_layer_get_lifetime")
public func ca_emitter_layer_get_lifetime(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.lifetime
}

@_cdecl("ca_emitter_layer_set_lifetime")
public func ca_emitter_layer_set_lifetime(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.lifetime = value
}

@_cdecl("ca_emitter_layer_get_emitter_position")
public func ca_emitter_layer_get_emitter_position(_ handle: UnsafeMutableRawPointer?, _ outPoint: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return false }
    return caWritePoint(layer.emitterPosition, out: outPoint)
}

@_cdecl("ca_emitter_layer_set_emitter_position")
public func ca_emitter_layer_set_emitter_position(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterPosition = CGPoint(x: x, y: y)
}

@_cdecl("ca_emitter_layer_get_emitter_size")
public func ca_emitter_layer_get_emitter_size(_ handle: UnsafeMutableRawPointer?, _ outSize: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return false }
    return caWriteSize(layer.emitterSize, out: outSize)
}

@_cdecl("ca_emitter_layer_set_emitter_size")
public func ca_emitter_layer_set_emitter_size(_ handle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterSize = CGSize(width: width, height: height)
}

@_cdecl("ca_emitter_layer_get_emitter_shape")
public func ca_emitter_layer_get_emitter_shape(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return caEmitterShapeRaw(layer.emitterShape)
}

@_cdecl("ca_emitter_layer_set_emitter_shape")
public func ca_emitter_layer_set_emitter_shape(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterShape = caEmitterShape(value)
}

@_cdecl("ca_emitter_layer_get_emitter_mode")
public func ca_emitter_layer_get_emitter_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 3 }
    return caEmitterModeRaw(layer.emitterMode)
}

@_cdecl("ca_emitter_layer_set_emitter_mode")
public func ca_emitter_layer_set_emitter_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.emitterMode = caEmitterMode(value)
}

@_cdecl("ca_emitter_layer_get_render_mode")
public func ca_emitter_layer_get_render_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return caEmitterRenderModeRaw(layer.renderMode)
}

@_cdecl("ca_emitter_layer_set_render_mode")
public func ca_emitter_layer_set_render_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.renderMode = caEmitterRenderMode(value)
}

@_cdecl("ca_emitter_layer_get_velocity")
public func ca_emitter_layer_get_velocity(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.velocity
}

@_cdecl("ca_emitter_layer_set_velocity")
public func ca_emitter_layer_set_velocity(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.velocity = value
}

@_cdecl("ca_emitter_layer_get_scale")
public func ca_emitter_layer_get_scale(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return 0 }
    return layer.scale
}

@_cdecl("ca_emitter_layer_set_scale")
public func ca_emitter_layer_set_scale(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let layer: CAEmitterLayer = caBorrow(handle) else { return }
    layer.scale = value
}

@_cdecl("ca_emitter_cell_new")
public func ca_emitter_cell_new() -> UnsafeMutableRawPointer? {
    caRetain(CAEmitterCell())
}

@_cdecl("ca_emitter_cell_set_name")
public func ca_emitter_cell_set_name(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.name = caCString(value)
}

@_cdecl("ca_emitter_cell_get_name")
public func ca_emitter_cell_get_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return nil }
    return caDup(cell.name)
}

@_cdecl("ca_emitter_cell_get_enabled")
public func ca_emitter_cell_get_enabled(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return false }
    return cell.isEnabled
}

@_cdecl("ca_emitter_cell_set_enabled")
public func ca_emitter_cell_set_enabled(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.isEnabled = value
}

@_cdecl("ca_emitter_cell_get_birth_rate")
public func ca_emitter_cell_get_birth_rate(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.birthRate
}

@_cdecl("ca_emitter_cell_set_birth_rate")
public func ca_emitter_cell_set_birth_rate(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.birthRate = value
}

@_cdecl("ca_emitter_cell_get_lifetime")
public func ca_emitter_cell_get_lifetime(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.lifetime
}

@_cdecl("ca_emitter_cell_set_lifetime")
public func ca_emitter_cell_set_lifetime(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.lifetime = value
}

@_cdecl("ca_emitter_cell_get_velocity")
public func ca_emitter_cell_get_velocity(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.velocity
}

@_cdecl("ca_emitter_cell_set_velocity")
public func ca_emitter_cell_set_velocity(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.velocity = value
}

@_cdecl("ca_emitter_cell_get_scale")
public func ca_emitter_cell_get_scale(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.scale
}

@_cdecl("ca_emitter_cell_set_scale")
public func ca_emitter_cell_set_scale(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.scale = value
}

@_cdecl("ca_emitter_cell_get_emission_range")
public func ca_emitter_cell_get_emission_range(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.emissionRange
}

@_cdecl("ca_emitter_cell_set_emission_range")
public func ca_emitter_cell_set_emission_range(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.emissionRange = value
}

@_cdecl("ca_emitter_cell_get_emission_longitude")
public func ca_emitter_cell_get_emission_longitude(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.emissionLongitude
}

@_cdecl("ca_emitter_cell_set_emission_longitude")
public func ca_emitter_cell_set_emission_longitude(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.emissionLongitude = value
}

@_cdecl("ca_emitter_cell_set_color")
public func ca_emitter_cell_set_color(_ handle: UnsafeMutableRawPointer?, _ colorHandle: UnsafeMutableRawPointer?) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    let color: CGColor? = caBorrow(colorHandle)
    cell.color = color
}

@_cdecl("ca_emitter_cell_get_color")
public func ca_emitter_cell_get_color(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let cell: CAEmitterCell = caBorrow(handle), let color = cell.color else { return nil }
    return caRetain(color)
}

@_cdecl("ca_emitter_cell_set_contents")
public func ca_emitter_cell_set_contents(_ handle: UnsafeMutableRawPointer?, _ imageHandle: UnsafeMutableRawPointer?) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    let image: CGImage? = caBorrow(imageHandle)
    cell.contents = image
}

@_cdecl("ca_emitter_cell_get_contents")
public func ca_emitter_cell_get_contents(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let cell: CAEmitterCell = caBorrow(handle), let contents = cell.contents else { return nil }
    return caRetain(contents as AnyObject)
}

@_cdecl("ca_emitter_cell_get_alpha_speed")
public func ca_emitter_cell_get_alpha_speed(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return 0 }
    return cell.alphaSpeed
}

@_cdecl("ca_emitter_cell_set_alpha_speed")
public func ca_emitter_cell_set_alpha_speed(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let cell: CAEmitterCell = caBorrow(handle) else { return }
    cell.alphaSpeed = value
}
