import Foundation
import QuartzCore

private func caNumberArray(_ values: UnsafePointer<Double>?, _ count: Int) -> [NSNumber]? {
    guard let values, count > 0 else { return nil }
    return UnsafeBufferPointer(start: values, count: count).map { NSNumber(value: $0) }
}

private func caNumberArrayCount(_ values: [NSNumber]?) -> Int {
    values?.count ?? 0
}

private func caNumberArrayValue(_ values: [NSNumber]?, _ index: Int) -> Double {
    guard let values, index >= 0, index < values.count else { return 0 }
    return values[index].doubleValue
}

@_cdecl("ca_keyframe_animation_set_timing_function_names")
public func ca_keyframe_animation_set_timing_function_names(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Int32>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    guard let values, count > 0 else {
        animation.timingFunctions = nil
        return
    }
    animation.timingFunctions = (0..<count).map { index in
        CAMediaTimingFunction(name: caTimingFunctionName(values[index]))
    }
}

@_cdecl("ca_keyframe_animation_timing_function_name_count")
public func ca_keyframe_animation_timing_function_name_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return animation.timingFunctions?.count ?? 0
}

@_cdecl("ca_keyframe_animation_timing_function_name_at")
public func ca_keyframe_animation_timing_function_name_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Int32 {
    guard let animation: CAKeyframeAnimation = caBorrow(handle), let values = animation.timingFunctions, index >= 0, index < values.count else { return -1 }
    return caTimingFunctionNameRaw(values[index])
}

@_cdecl("ca_keyframe_animation_set_tension_values")
public func ca_keyframe_animation_set_tension_values(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    animation.tensionValues = caNumberArray(values, count)
}

@_cdecl("ca_keyframe_animation_tension_value_count")
public func ca_keyframe_animation_tension_value_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayCount(animation.tensionValues)
}

@_cdecl("ca_keyframe_animation_tension_value_at")
public func ca_keyframe_animation_tension_value_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayValue(animation.tensionValues, index)
}

@_cdecl("ca_keyframe_animation_set_continuity_values")
public func ca_keyframe_animation_set_continuity_values(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    animation.continuityValues = caNumberArray(values, count)
}

@_cdecl("ca_keyframe_animation_continuity_value_count")
public func ca_keyframe_animation_continuity_value_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayCount(animation.continuityValues)
}

@_cdecl("ca_keyframe_animation_continuity_value_at")
public func ca_keyframe_animation_continuity_value_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayValue(animation.continuityValues, index)
}

@_cdecl("ca_keyframe_animation_set_bias_values")
public func ca_keyframe_animation_set_bias_values(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    animation.biasValues = caNumberArray(values, count)
}

@_cdecl("ca_keyframe_animation_bias_value_count")
public func ca_keyframe_animation_bias_value_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayCount(animation.biasValues)
}

@_cdecl("ca_keyframe_animation_bias_value_at")
public func ca_keyframe_animation_bias_value_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caNumberArrayValue(animation.biasValues, index)
}
