import CoreGraphics
import CoreVideo
import Foundation
import Metal
import QuartzCore

@inline(__always)
func caRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func caBorrow<T>(_ handle: UnsafeMutableRawPointer?) -> T? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? T
}

@inline(__always)
func caReleaseHandle(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
func caDup(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else { return nil }
    return strdup(value)
}

func caCString(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else { return nil }
    return String(cString: value)
}

func caWritePoint(_ point: CGPoint, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Double.self)
    ptr[0] = point.x
    ptr[1] = point.y
    return true
}

func caWriteSize(_ size: CGSize, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Double.self)
    ptr[0] = size.width
    ptr[1] = size.height
    return true
}

func caWriteRect(_ rect: CGRect, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Double.self)
    ptr[0] = rect.origin.x
    ptr[1] = rect.origin.y
    ptr[2] = rect.size.width
    ptr[3] = rect.size.height
    return true
}

func caWriteTransform(_ transform: CATransform3D, out: UnsafeMutableRawPointer?) -> Bool {
    guard let out else { return false }
    let ptr = out.assumingMemoryBound(to: Double.self)
    ptr[0] = transform.m11
    ptr[1] = transform.m12
    ptr[2] = transform.m13
    ptr[3] = transform.m14
    ptr[4] = transform.m21
    ptr[5] = transform.m22
    ptr[6] = transform.m23
    ptr[7] = transform.m24
    ptr[8] = transform.m31
    ptr[9] = transform.m32
    ptr[10] = transform.m33
    ptr[11] = transform.m34
    ptr[12] = transform.m41
    ptr[13] = transform.m42
    ptr[14] = transform.m43
    ptr[15] = transform.m44
    return true
}

func caReadTransform(_ raw: UnsafeRawPointer?) -> CATransform3D? {
    guard let raw else { return nil }
    let ptr = raw.assumingMemoryBound(to: Double.self)
    return CATransform3D(
        m11: ptr[0], m12: ptr[1], m13: ptr[2], m14: ptr[3],
        m21: ptr[4], m22: ptr[5], m23: ptr[6], m24: ptr[7],
        m31: ptr[8], m32: ptr[9], m33: ptr[10], m34: ptr[11],
        m41: ptr[12], m42: ptr[13], m43: ptr[14], m44: ptr[15]
    )
}

func caContentsGravity(_ raw: Int32) -> CALayerContentsGravity {
    switch raw {
    case 0: return .center
    case 1: return .top
    case 2: return .bottom
    case 3: return .left
    case 4: return .right
    case 5: return .topLeft
    case 6: return .topRight
    case 7: return .bottomLeft
    case 8: return .bottomRight
    case 10: return .resizeAspect
    case 11: return .resizeAspectFill
    default: return .resize
    }
}

func caContentsGravityRaw(_ value: CALayerContentsGravity) -> Int32 {
    switch value {
    case .center: return 0
    case .top: return 1
    case .bottom: return 2
    case .left: return 3
    case .right: return 4
    case .topLeft: return 5
    case .topRight: return 6
    case .bottomLeft: return 7
    case .bottomRight: return 8
    case .resizeAspect: return 10
    case .resizeAspectFill: return 11
    default: return 9
    }
}

@available(macOS 15.0, *)
func caToneMapMode(_ raw: Int32) -> CALayer.ToneMapMode {
    switch raw {
    case 1: return .never
    case 2: return .ifSupported
    default: return .automatic
    }
}

@available(macOS 15.0, *)
func caToneMapModeRaw(_ value: CALayer.ToneMapMode) -> Int32 {
    switch value {
    case .never: return 1
    case .ifSupported: return 2
    default: return 0
    }
}

func caLineCap(_ raw: Int32) -> CAShapeLayerLineCap {
    switch raw {
    case 1: return .round
    case 2: return .square
    default: return .butt
    }
}

func caLineCapRaw(_ value: CAShapeLayerLineCap) -> Int32 {
    switch value {
    case .round: return 1
    case .square: return 2
    default: return 0
    }
}

func caLineJoin(_ raw: Int32) -> CAShapeLayerLineJoin {
    switch raw {
    case 1: return .round
    case 2: return .bevel
    default: return .miter
    }
}

func caLineJoinRaw(_ value: CAShapeLayerLineJoin) -> Int32 {
    switch value {
    case .round: return 1
    case .bevel: return 2
    default: return 0
    }
}

func caTextAlignment(_ raw: Int32) -> CATextLayerAlignmentMode {
    switch raw {
    case 1: return .left
    case 2: return .right
    case 3: return .center
    case 4: return .justified
    default: return .natural
    }
}

func caTextAlignmentRaw(_ value: CATextLayerAlignmentMode) -> Int32 {
    switch value {
    case .left: return 1
    case .right: return 2
    case .center: return 3
    case .justified: return 4
    default: return 0
    }
}

func caTextTruncation(_ raw: Int32) -> CATextLayerTruncationMode {
    switch raw {
    case 1: return .start
    case 2: return .middle
    case 3: return .end
    default: return .none
    }
}

func caTextTruncationRaw(_ value: CATextLayerTruncationMode) -> Int32 {
    switch value {
    case .start: return 1
    case .middle: return 2
    case .end: return 3
    default: return 0
    }
}

func caGradientType(_ raw: Int32) -> CAGradientLayerType {
    switch raw {
    case 1: return .radial
    case 2: return .conic
    default: return .axial
    }
}

func caGradientTypeRaw(_ value: CAGradientLayerType) -> Int32 {
    switch value {
    case .radial: return 1
    case .conic: return 2
    default: return 0
    }
}

func caEmitterShape(_ raw: Int32) -> CAEmitterLayerEmitterShape {
    switch raw {
    case 1: return .line
    case 2: return .rectangle
    case 3: return .cuboid
    case 4: return .circle
    case 5: return .sphere
    default: return .point
    }
}

func caEmitterShapeRaw(_ value: CAEmitterLayerEmitterShape) -> Int32 {
    switch value {
    case .line: return 1
    case .rectangle: return 2
    case .cuboid: return 3
    case .circle: return 4
    case .sphere: return 5
    default: return 0
    }
}

func caEmitterMode(_ raw: Int32) -> CAEmitterLayerEmitterMode {
    switch raw {
    case 0: return .points
    case 1: return .outline
    case 2: return .surface
    default: return .volume
    }
}

func caEmitterModeRaw(_ value: CAEmitterLayerEmitterMode) -> Int32 {
    switch value {
    case .points: return 0
    case .outline: return 1
    case .surface: return 2
    default: return 3
    }
}

func caEmitterRenderMode(_ raw: Int32) -> CAEmitterLayerRenderMode {
    switch raw {
    case 1: return .oldestFirst
    case 2: return .oldestLast
    case 3: return .backToFront
    case 4: return .additive
    default: return .unordered
    }
}

func caEmitterRenderModeRaw(_ value: CAEmitterLayerRenderMode) -> Int32 {
    switch value {
    case .oldestFirst: return 1
    case .oldestLast: return 2
    case .backToFront: return 3
    case .additive: return 4
    default: return 0
    }
}

func caCalculationMode(_ raw: Int32) -> CAAnimationCalculationMode {
    switch raw {
    case 1: return .discrete
    case 2: return .paced
    case 3: return .cubic
    case 4: return .cubicPaced
    default: return .linear
    }
}

func caCalculationModeRaw(_ value: CAAnimationCalculationMode) -> Int32 {
    switch value {
    case .discrete: return 1
    case .paced: return 2
    case .cubic: return 3
    case .cubicPaced: return 4
    default: return 0
    }
}

func caRotationMode(_ raw: Int32) -> CAAnimationRotationMode? {
    switch raw {
    case 1: return .rotateAuto
    case 2: return .rotateAutoReverse
    default: return nil
    }
}

func caRotationModeRaw(_ value: CAAnimationRotationMode?) -> Int32 {
    guard let value else { return 0 }
    switch value {
    case .rotateAuto: return 1
    case .rotateAutoReverse: return 2
    default: return 0
    }
}

func caTransitionType(_ raw: Int32) -> CATransitionType {
    switch raw {
    case 1: return .moveIn
    case 2: return .push
    case 3: return .reveal
    default: return .fade
    }
}

func caTransitionTypeRaw(_ value: CATransitionType) -> Int32 {
    switch value {
    case .moveIn: return 1
    case .push: return 2
    case .reveal: return 3
    default: return 0
    }
}

func caTransitionSubtype(_ raw: Int32) -> CATransitionSubtype? {
    switch raw {
    case 1: return .fromRight
    case 2: return .fromLeft
    case 3: return .fromTop
    case 4: return .fromBottom
    default: return nil
    }
}

func caTransitionSubtypeRaw(_ value: CATransitionSubtype?) -> Int32 {
    guard let value else { return 0 }
    switch value {
    case .fromRight: return 1
    case .fromLeft: return 2
    case .fromTop: return 3
    case .fromBottom: return 4
    default: return 0
    }
}

func caValueFunctionName(_ raw: Int32) -> CAValueFunctionName {
    switch raw {
    case 1: return .rotateY
    case 2: return .rotateZ
    case 3: return .scale
    case 4: return .scaleX
    case 5: return .scaleY
    case 6: return .scaleZ
    case 7: return .translate
    case 8: return .translateX
    case 9: return .translateY
    case 10: return .translateZ
    default: return .rotateX
    }
}

func caValueFunctionNameRaw(_ value: CAValueFunctionName) -> Int32 {
    switch value {
    case .rotateY: return 1
    case .rotateZ: return 2
    case .scale: return 3
    case .scaleX: return 4
    case .scaleY: return 5
    case .scaleZ: return 6
    case .translate: return 7
    case .translateX: return 8
    case .translateY: return 9
    case .translateZ: return 10
    default: return 0
    }
}

@_cdecl("ca_retain")
public func ca_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).retain().toOpaque()
}

@_cdecl("ca_release")
public func ca_release(_ handle: UnsafeMutableRawPointer?) {
    caReleaseHandle(handle)
}

@_cdecl("ca_color_new_rgba")
public func ca_color_new_rgba(_ red: Double, _ green: Double, _ blue: Double, _ alpha: Double) -> UnsafeMutableRawPointer? {
    caRetain(CGColor(red: red, green: green, blue: blue, alpha: alpha))
}

@_cdecl("ca_color_get_components")
public func ca_color_get_components(_ handle: UnsafeMutableRawPointer?, _ outComponents: UnsafeMutableRawPointer?) -> Bool {
    guard let color: CGColor = caBorrow(handle), let outComponents else { return false }
    let space = CGColorSpace(name: CGColorSpace.sRGB)
    let converted = color.converted(to: space ?? CGColorSpaceCreateDeviceRGB(), intent: .defaultIntent, options: nil) ?? color
    let ptr = outComponents.assumingMemoryBound(to: Double.self)
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

@_cdecl("ca_path_new_mutable")
public func ca_path_new_mutable() -> UnsafeMutableRawPointer? {
    caRetain(CGMutablePath())
}

@_cdecl("ca_path_move_to")
public func ca_path_move_to(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let path: CGMutablePath = caBorrow(handle) else { return }
    path.move(to: CGPoint(x: x, y: y))
}

@_cdecl("ca_path_add_line_to")
public func ca_path_add_line_to(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double) {
    guard let path: CGMutablePath = caBorrow(handle) else { return }
    path.addLine(to: CGPoint(x: x, y: y))
}

@_cdecl("ca_path_add_rect")
public func ca_path_add_rect(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let path: CGMutablePath = caBorrow(handle) else { return }
    path.addRect(CGRect(x: x, y: y, width: width, height: height))
}

@_cdecl("ca_path_add_ellipse")
public func ca_path_add_ellipse(_ handle: UnsafeMutableRawPointer?, _ x: Double, _ y: Double, _ width: Double, _ height: Double) {
    guard let path: CGMutablePath = caBorrow(handle) else { return }
    path.addEllipse(in: CGRect(x: x, y: y, width: width, height: height))
}

@_cdecl("ca_path_close_subpath")
public func ca_path_close_subpath(_ handle: UnsafeMutableRawPointer?) {
    guard let path: CGMutablePath = caBorrow(handle) else { return }
    path.closeSubpath()
}

@_cdecl("ca_path_get_bounding_box")
public func ca_path_get_bounding_box(_ handle: UnsafeMutableRawPointer?, _ outRect: UnsafeMutableRawPointer?) -> Bool {
    guard let path: CGPath = caBorrow(handle) else { return false }
    return caWriteRect(path.boundingBox, out: outRect)
}
