import CoreGraphics
import Foundation
import QuartzCore

private func caWriteGradientColorComponents(_ color: CGColor, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let space = CGColorSpace(name: CGColorSpace.sRGB)
    let converted = color.converted(to: space ?? CGColorSpaceCreateDeviceRGB(), intent: .defaultIntent, options: nil) ?? color
    let ptr = out.assumingMemoryBound(to: Double.self)
    let components = converted.components ?? [0, 0, 0, 0]
    switch converted.numberOfComponents {
    case 4:
        ptr[0] = components[0]
        ptr[1] = components[1]
        ptr[2] = components[2]
        ptr[3] = components[3]
    case 2:
        ptr[0] = components[0]
        ptr[1] = components[0]
        ptr[2] = components[0]
        ptr[3] = components[1]
    default:
        ptr[0] = 0
        ptr[1] = 0
        ptr[2] = 0
        ptr[3] = 0
    }
    return true
}

@_cdecl("ca_gradient_layer_get_color_components_at")
public func ca_gradient_layer_get_color_components_at(_ handle: UnsafeMutableRawPointer?, _ index: Int, _ outComponents: UnsafeMutableRawPointer?) -> Bool {
    guard let layer: CAGradientLayer = caBorrow(handle), let colors = layer.colors, index >= 0, index < colors.count else { return false }
    let color = colors[index] as! CGColor
    return caWriteGradientColorComponents(color, out: outComponents)
}
