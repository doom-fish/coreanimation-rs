import Foundation
import QuartzCore

@_cdecl("ca_animation_group_append_animation")
public func ca_animation_group_append_animation(_ handle: UnsafeMutableRawPointer?, _ animationHandle: UnsafeMutableRawPointer?) {
    guard let group: CAAnimationGroup = caBorrow(handle), let animation: CAAnimation = caBorrow(animationHandle) else { return }
    var animations = group.animations ?? []
    animations.append(animation)
    group.animations = animations
}

@_cdecl("ca_animation_group_clear_animations")
public func ca_animation_group_clear_animations(_ handle: UnsafeMutableRawPointer?) {
    guard let group: CAAnimationGroup = caBorrow(handle) else { return }
    group.animations = nil
}
