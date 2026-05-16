import Foundation
import QuartzCore

func caTimingFunctionName(_ raw: Int32) -> CAMediaTimingFunctionName {
    switch raw {
    case 0: return .linear
    case 1: return .easeIn
    case 2: return .easeOut
    case 3: return .easeInEaseOut
    default: return .default
    }
}

private func caTimingFunctionMatches(
    _ lhs: CAMediaTimingFunction,
    _ rhs: CAMediaTimingFunction
) -> Bool {
    for index in 0..<4 {
        var left = [Float](repeating: 0, count: 2)
        var right = [Float](repeating: 0, count: 2)
        lhs.getControlPoint(at: index, values: &left)
        rhs.getControlPoint(at: index, values: &right)
        if abs(left[0] - right[0]) > 0.000_1 || abs(left[1] - right[1]) > 0.000_1 {
            return false
        }
    }
    return true
}

func caTimingFunctionNameRaw(_ function: CAMediaTimingFunction?) -> Int32 {
    guard let function else { return -1 }
    let mappings: [(Int32, CAMediaTimingFunctionName)] = [
        (0, .linear),
        (1, .easeIn),
        (2, .easeOut),
        (3, .easeInEaseOut),
        (4, .default),
    ]
    for (raw, name) in mappings {
        if caTimingFunctionMatches(function, CAMediaTimingFunction(name: name)) {
            return raw
        }
    }
    return -1
}

@_cdecl("ca_timing_function_new_named")
public func ca_timing_function_new_named(_ value: Int32) -> UnsafeMutableRawPointer? {
    caRetain(CAMediaTimingFunction(name: caTimingFunctionName(value)))
}

@_cdecl("ca_timing_function_new_control_points")
public func ca_timing_function_new_control_points(_ c1x: Float, _ c1y: Float, _ c2x: Float, _ c2y: Float) -> UnsafeMutableRawPointer? {
    caRetain(CAMediaTimingFunction(controlPoints: c1x, c1y, c2x, c2y))
}

@_cdecl("ca_timing_function_get_name")
public func ca_timing_function_get_name(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let function: CAMediaTimingFunction = caBorrow(handle) else { return -1 }
    return caTimingFunctionNameRaw(function)
}

@_cdecl("ca_timing_function_get_control_point")
public func ca_timing_function_get_control_point(_ handle: UnsafeMutableRawPointer?, _ index: Int, _ outValues: UnsafeMutableRawPointer?) -> Bool {
    guard let function: CAMediaTimingFunction = caBorrow(handle), let outValues, index >= 0, index < 4 else {
        return false
    }
    var values = [Float](repeating: 0, count: 2)
    function.getControlPoint(at: index, values: &values)
    let ptr = outValues.assumingMemoryBound(to: Float.self)
    ptr[0] = values[0]
    ptr[1] = values[1]
    return true
}

func caFillMode(_ raw: Int32) -> CAMediaTimingFillMode {
    switch raw {
    case 1: return .forwards
    case 2: return .backwards
    case 3: return .both
    default: return .removed
    }
}

func caFillModeRaw(_ value: CAMediaTimingFillMode) -> Int32 {
    switch value {
    case .forwards: return 1
    case .backwards: return 2
    case .both: return 3
    default: return 0
    }
}

@_cdecl("ca_animation_get_begin_time")
public func ca_animation_get_begin_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.beginTime
}

@_cdecl("ca_animation_set_begin_time")
public func ca_animation_set_begin_time(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.beginTime = value
}

@_cdecl("ca_animation_get_speed")
public func ca_animation_get_speed(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.speed
}

@_cdecl("ca_animation_set_speed")
public func ca_animation_set_speed(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.speed = value
}

@_cdecl("ca_animation_get_time_offset")
public func ca_animation_get_time_offset(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.timeOffset
}

@_cdecl("ca_animation_set_time_offset")
public func ca_animation_set_time_offset(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.timeOffset = value
}

@_cdecl("ca_animation_get_repeat_duration")
public func ca_animation_get_repeat_duration(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.repeatDuration
}

@_cdecl("ca_animation_set_repeat_duration")
public func ca_animation_set_repeat_duration(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.repeatDuration = value
}

@_cdecl("ca_animation_get_fill_mode")
public func ca_animation_get_fill_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return caFillModeRaw(animation.fillMode)
}

@_cdecl("ca_animation_set_fill_mode")
public func ca_animation_set_fill_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.fillMode = caFillMode(value)
}
