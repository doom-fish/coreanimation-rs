import Foundation
import QuartzCore

private func caWriteOptionalNumber(_ value: NSNumber?, outValue: UnsafeMutableRawPointer?) -> Bool {
    guard let value, let outValue else { return false }
    outValue.assumingMemoryBound(to: Double.self).pointee = value.doubleValue
    return true
}

@_cdecl("ca_basic_animation_get_from_number")
public func ca_basic_animation_get_from_number(_ handle: UnsafeMutableRawPointer?, _ outValue: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return false }
    return caWriteOptionalNumber(animation.fromValue as? NSNumber, outValue: outValue)
}

@_cdecl("ca_basic_animation_get_to_number")
public func ca_basic_animation_get_to_number(_ handle: UnsafeMutableRawPointer?, _ outValue: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return false }
    return caWriteOptionalNumber(animation.toValue as? NSNumber, outValue: outValue)
}

@_cdecl("ca_basic_animation_get_by_number")
public func ca_basic_animation_get_by_number(_ handle: UnsafeMutableRawPointer?, _ outValue: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return false }
    return caWriteOptionalNumber(animation.byValue as? NSNumber, outValue: outValue)
}

@_cdecl("ca_property_animation_get_additive")
public func ca_property_animation_get_additive(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return false }
    return animation.isAdditive
}

@_cdecl("ca_property_animation_set_additive")
public func ca_property_animation_set_additive(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return }
    animation.isAdditive = value
}

@_cdecl("ca_property_animation_get_cumulative")
public func ca_property_animation_get_cumulative(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return false }
    return animation.isCumulative
}

@_cdecl("ca_property_animation_set_cumulative")
public func ca_property_animation_set_cumulative(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return }
    animation.isCumulative = value
}
