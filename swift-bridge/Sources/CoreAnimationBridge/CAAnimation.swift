import Foundation
import QuartzCore

@_cdecl("ca_animation_get_timing_function_name")
public func ca_animation_get_timing_function_name(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let animation: CAAnimation = caBorrow(handle) else { return -1 }
    return caTimingFunctionNameRaw(animation.timingFunction)
}

@_cdecl("ca_animation_set_timing_function_name")
public func ca_animation_set_timing_function_name(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.timingFunction = value < 0 ? nil : CAMediaTimingFunction(name: caTimingFunctionName(value))
}
