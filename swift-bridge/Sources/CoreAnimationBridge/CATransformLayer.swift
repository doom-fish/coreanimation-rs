import Foundation
import QuartzCore

@_cdecl("ca_transform_layer_new")
public func ca_transform_layer_new() -> UnsafeMutableRawPointer? {
    caRetain(CATransformLayer())
}
