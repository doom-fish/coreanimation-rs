import CoreGraphics
import Foundation
import QuartzCore

@_cdecl("ca_animation_new")
public func ca_animation_new() -> UnsafeMutableRawPointer? {
    caRetain(CAAnimation())
}

@_cdecl("ca_animation_get_duration")
public func ca_animation_get_duration(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.duration
}

@_cdecl("ca_animation_set_duration")
public func ca_animation_set_duration(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.duration = value
}

@_cdecl("ca_animation_get_repeat_count")
public func ca_animation_get_repeat_count(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let animation: CAAnimation = caBorrow(handle) else { return 0 }
    return animation.repeatCount
}

@_cdecl("ca_animation_set_repeat_count")
public func ca_animation_set_repeat_count(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.repeatCount = value
}

@_cdecl("ca_animation_get_autoreverses")
public func ca_animation_get_autoreverses(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CAAnimation = caBorrow(handle) else { return false }
    return animation.autoreverses
}

@_cdecl("ca_animation_set_autoreverses")
public func ca_animation_set_autoreverses(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.autoreverses = value
}

@_cdecl("ca_animation_get_removed_on_completion")
public func ca_animation_get_removed_on_completion(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let animation: CAAnimation = caBorrow(handle) else { return true }
    return animation.isRemovedOnCompletion
}

@_cdecl("ca_animation_set_removed_on_completion")
public func ca_animation_set_removed_on_completion(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let animation: CAAnimation = caBorrow(handle) else { return }
    animation.isRemovedOnCompletion = value
}

@_cdecl("ca_basic_animation_new")
public func ca_basic_animation_new(_ keyPath: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    caRetain(CABasicAnimation(keyPath: caCString(keyPath)))
}

@_cdecl("ca_property_animation_get_key_path")
public func ca_property_animation_get_key_path(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return nil }
    return caDup(animation.keyPath)
}

@_cdecl("ca_property_animation_set_key_path")
public func ca_property_animation_set_key_path(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let animation: CAPropertyAnimation = caBorrow(handle) else { return }
    animation.keyPath = caCString(value)
}

@_cdecl("ca_basic_animation_set_from_number")
public func ca_basic_animation_set_from_number(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return }
    animation.fromValue = NSNumber(value: value)
}

@_cdecl("ca_basic_animation_set_to_number")
public func ca_basic_animation_set_to_number(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return }
    animation.toValue = NSNumber(value: value)
}

@_cdecl("ca_basic_animation_set_by_number")
public func ca_basic_animation_set_by_number(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CABasicAnimation = caBorrow(handle) else { return }
    animation.byValue = NSNumber(value: value)
}

@_cdecl("ca_keyframe_animation_new")
public func ca_keyframe_animation_new(_ keyPath: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    caRetain(CAKeyframeAnimation(keyPath: caCString(keyPath)))
}

@_cdecl("ca_keyframe_animation_set_values")
public func ca_keyframe_animation_set_values(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    guard let values, count > 0 else {
        animation.values = nil
        return
    }
    animation.values = UnsafeBufferPointer(start: values, count: count).map { NSNumber(value: $0) }
}

@_cdecl("ca_keyframe_animation_value_count")
public func ca_keyframe_animation_value_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return animation.values?.count ?? 0
}

@_cdecl("ca_keyframe_animation_value_at")
public func ca_keyframe_animation_value_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let animation: CAKeyframeAnimation = caBorrow(handle), let values = animation.values, index >= 0, index < values.count, let value = values[index] as? NSNumber else { return 0 }
    return value.doubleValue
}

@_cdecl("ca_keyframe_animation_set_path")
public func ca_keyframe_animation_set_path(_ handle: UnsafeMutableRawPointer?, _ pathHandle: UnsafeMutableRawPointer?) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    let path: CGPath? = caBorrow(pathHandle)
    animation.path = path
}

@_cdecl("ca_keyframe_animation_get_path")
public func ca_keyframe_animation_get_path(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let animation: CAKeyframeAnimation = caBorrow(handle), let path = animation.path else { return nil }
    return caRetain(path)
}

@_cdecl("ca_keyframe_animation_set_key_times")
public func ca_keyframe_animation_set_key_times(_ handle: UnsafeMutableRawPointer?, _ values: UnsafePointer<Double>?, _ count: Int) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    guard let values, count > 0 else {
        animation.keyTimes = nil
        return
    }
    animation.keyTimes = UnsafeBufferPointer(start: values, count: count).map { NSNumber(value: $0) }
}

@_cdecl("ca_keyframe_animation_key_time_count")
public func ca_keyframe_animation_key_time_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return animation.keyTimes?.count ?? 0
}

@_cdecl("ca_keyframe_animation_key_time_at")
public func ca_keyframe_animation_key_time_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let animation: CAKeyframeAnimation = caBorrow(handle), let keyTimes = animation.keyTimes, index >= 0, index < keyTimes.count else { return 0 }
    return keyTimes[index].doubleValue
}

@_cdecl("ca_keyframe_animation_get_calculation_mode")
public func ca_keyframe_animation_get_calculation_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caCalculationModeRaw(animation.calculationMode)
}

@_cdecl("ca_keyframe_animation_set_calculation_mode")
public func ca_keyframe_animation_set_calculation_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    animation.calculationMode = caCalculationMode(value)
}

@_cdecl("ca_keyframe_animation_get_rotation_mode")
public func ca_keyframe_animation_get_rotation_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return 0 }
    return caRotationModeRaw(animation.rotationMode)
}

@_cdecl("ca_keyframe_animation_set_rotation_mode")
public func ca_keyframe_animation_set_rotation_mode(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let animation: CAKeyframeAnimation = caBorrow(handle) else { return }
    animation.rotationMode = caRotationMode(value)
}

@_cdecl("ca_spring_animation_new")
public func ca_spring_animation_new(_ keyPath: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    caRetain(CASpringAnimation(keyPath: caCString(keyPath)))
}

@_cdecl("ca_spring_animation_get_mass")
public func ca_spring_animation_get_mass(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return 0 }
    return animation.mass
}

@_cdecl("ca_spring_animation_set_mass")
public func ca_spring_animation_set_mass(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return }
    animation.mass = value
}

@_cdecl("ca_spring_animation_get_stiffness")
public func ca_spring_animation_get_stiffness(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return 0 }
    return animation.stiffness
}

@_cdecl("ca_spring_animation_set_stiffness")
public func ca_spring_animation_set_stiffness(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return }
    animation.stiffness = value
}

@_cdecl("ca_spring_animation_get_damping")
public func ca_spring_animation_get_damping(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return 0 }
    return animation.damping
}

@_cdecl("ca_spring_animation_set_damping")
public func ca_spring_animation_set_damping(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return }
    animation.damping = value
}

@_cdecl("ca_spring_animation_get_initial_velocity")
public func ca_spring_animation_get_initial_velocity(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return 0 }
    return animation.initialVelocity
}

@_cdecl("ca_spring_animation_set_initial_velocity")
public func ca_spring_animation_set_initial_velocity(_ handle: UnsafeMutableRawPointer?, _ value: Double) {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return }
    animation.initialVelocity = value
}

@_cdecl("ca_spring_animation_get_settling_duration")
public func ca_spring_animation_get_settling_duration(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return 0 }
    return animation.settlingDuration
}

@_cdecl("ca_animation_group_new")
public func ca_animation_group_new() -> UnsafeMutableRawPointer? {
    caRetain(CAAnimationGroup())
}

@_cdecl("ca_animation_group_set_animations")
public func ca_animation_group_set_animations(_ handle: UnsafeMutableRawPointer?, _ animations: UnsafePointer<UnsafeMutableRawPointer?>?, _ count: Int) {
    guard let group: CAAnimationGroup = caBorrow(handle) else { return }
    guard let animations, count > 0 else {
        group.animations = nil
        return
    }
    group.animations = (0..<count).compactMap { index in
        let handle = animations[index]
        let animation: CAAnimation? = caBorrow(handle)
        return animation
    }
}

@_cdecl("ca_animation_group_animation_count")
public func ca_animation_group_animation_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let group: CAAnimationGroup = caBorrow(handle) else { return 0 }
    return group.animations?.count ?? 0
}

@_cdecl("ca_animation_group_animation_at")
public func ca_animation_group_animation_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let group: CAAnimationGroup = caBorrow(handle), let animations = group.animations, index >= 0, index < animations.count else { return nil }
    return caRetain(animations[index])
}

@_cdecl("ca_transition_new")
public func ca_transition_new() -> UnsafeMutableRawPointer? {
    caRetain(CATransition())
}

@_cdecl("ca_transition_get_type")
public func ca_transition_get_type(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let transition: CATransition = caBorrow(handle) else { return 0 }
    return caTransitionTypeRaw(transition.type)
}

@_cdecl("ca_transition_set_type")
public func ca_transition_set_type(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let transition: CATransition = caBorrow(handle) else { return }
    transition.type = caTransitionType(value)
}

@_cdecl("ca_transition_get_subtype")
public func ca_transition_get_subtype(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let transition: CATransition = caBorrow(handle) else { return 0 }
    return caTransitionSubtypeRaw(transition.subtype)
}

@_cdecl("ca_transition_set_subtype")
public func ca_transition_set_subtype(_ handle: UnsafeMutableRawPointer?, _ value: Int32) {
    guard let transition: CATransition = caBorrow(handle) else { return }
    transition.subtype = caTransitionSubtype(value)
}

@_cdecl("ca_transition_get_start_progress")
public func ca_transition_get_start_progress(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let transition: CATransition = caBorrow(handle) else { return 0 }
    return transition.startProgress
}

@_cdecl("ca_transition_set_start_progress")
public func ca_transition_set_start_progress(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let transition: CATransition = caBorrow(handle) else { return }
    transition.startProgress = value
}

@_cdecl("ca_transition_get_end_progress")
public func ca_transition_get_end_progress(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let transition: CATransition = caBorrow(handle) else { return 1 }
    return transition.endProgress
}

@_cdecl("ca_transition_set_end_progress")
public func ca_transition_set_end_progress(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard let transition: CATransition = caBorrow(handle) else { return }
    transition.endProgress = value
}
