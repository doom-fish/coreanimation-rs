import Foundation
import QuartzCore

@_cdecl("ca_spring_animation_configure")
public func ca_spring_animation_configure(
    _ handle: UnsafeMutableRawPointer?,
    _ mass: Double,
    _ stiffness: Double,
    _ damping: Double,
    _ initialVelocity: Double
) {
    guard let animation: CASpringAnimation = caBorrow(handle) else { return }
    animation.mass = mass
    animation.stiffness = stiffness
    animation.damping = damping
    animation.initialVelocity = initialVelocity
}

@_cdecl("ca_spring_animation_new_perceptual")
public func ca_spring_animation_new_perceptual(
    _ keyPath: UnsafePointer<CChar>?,
    _ perceptualDuration: Double,
    _ bounce: Double
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), perceptualDuration.isFinite, perceptualDuration > 0, bounce.isFinite else {
        return nil
    }
    let animation = CASpringAnimation(perceptualDuration: perceptualDuration, bounce: bounce)
    animation.keyPath = caCString(keyPath)
    return caRetain(animation)
}

@_cdecl("ca_spring_animation_supports_perceptual_parameters")
public func ca_spring_animation_supports_perceptual_parameters() -> Bool {
    if #available(macOS 14.0, *) {
        return true
    }
    return false
}

@_cdecl("ca_spring_animation_get_perceptual_duration")
public func ca_spring_animation_get_perceptual_duration(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let animation: CASpringAnimation = caBorrow(handle) else { return .nan }
    return animation.perceptualDuration
}

@_cdecl("ca_spring_animation_get_bounce")
public func ca_spring_animation_get_bounce(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let animation: CASpringAnimation = caBorrow(handle) else { return .nan }
    return animation.bounce
}

@_cdecl("ca_spring_animation_get_allows_overdamping")
public func ca_spring_animation_get_allows_overdamping(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard #available(macOS 14.0, *), let animation: CASpringAnimation = caBorrow(handle) else { return false }
    return animation.allowsOverdamping
}

@_cdecl("ca_spring_animation_set_allows_overdamping")
public func ca_spring_animation_set_allows_overdamping(_ handle: UnsafeMutableRawPointer?, _ value: Bool) -> Bool {
    guard #available(macOS 14.0, *), let animation: CASpringAnimation = caBorrow(handle) else { return false }
    animation.allowsOverdamping = value
    return true
}
