import Foundation
import QuartzCore

@_cdecl("ca_animation_get_timing_function")
public func ca_animation_get_timing_function(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation: CAAnimation = caBorrow(handle), let function = animation.timingFunction else {
        return nil
    }
    return caRetain(function)
}

@_cdecl("ca_animation_set_timing_function")
public func ca_animation_set_timing_function(_ handle: UnsafeMutableRawPointer?, _ valueHandle: UnsafeMutableRawPointer?) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    let value: CAMediaTimingFunction? = caBorrow(valueHandle)
    animation.timingFunction = value
}

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
