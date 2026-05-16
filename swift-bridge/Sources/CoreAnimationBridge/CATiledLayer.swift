import Foundation
import QuartzCore

@_cdecl("ca_tiled_layer_new")
public func ca_tiled_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CATiledLayer())
}

@_cdecl("ca_tiled_layer_get_levels_of_detail")
public func ca_tiled_layer_get_levels_of_detail(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CATiledLayer = caBorrow(handle) else { return 0 }
    return Int(layer.levelsOfDetail)
}

@_cdecl("ca_tiled_layer_set_levels_of_detail")
public func ca_tiled_layer_set_levels_of_detail(_ handle: UnsafeMutableRawPointer?, _ value: Int) {
    guard let layer: CATiledLayer = caBorrow(handle) else { return }
    layer.levelsOfDetail = max(1, value)
}

@_cdecl("ca_tiled_layer_get_levels_of_detail_bias")
public func ca_tiled_layer_get_levels_of_detail_bias(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CATiledLayer = caBorrow(handle) else { return 0 }
    return Int(layer.levelsOfDetailBias)
}

@_cdecl("ca_tiled_layer_set_levels_of_detail_bias")
public func ca_tiled_layer_set_levels_of_detail_bias(_ handle: UnsafeMutableRawPointer?, _ value: Int) {
    guard let layer: CATiledLayer = caBorrow(handle) else { return }
    layer.levelsOfDetailBias = max(0, value)
}

@_cdecl("ca_tiled_layer_get_tile_size")
public func ca_tiled_layer_get_tile_size(_ handle: UnsafeMutableRawPointer?, _ outSize: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CATiledLayer = caBorrow(handle) else { return false }
    return caWriteSize(layer.tileSize, out: outSize)
}

@_cdecl("ca_tiled_layer_set_tile_size")
public func ca_tiled_layer_set_tile_size(_ handle: UnsafeMutableRawPointer?, _ width: Double, _ height: Double) {
    guard let layer: CATiledLayer = caBorrow(handle) else { return }
    layer.tileSize = CGSize(width: width, height: height)
}
