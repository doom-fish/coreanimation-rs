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
