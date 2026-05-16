import Foundation
import QuartzCore

@_cdecl("ca_transition_has_subtype")
public func ca_transition_has_subtype(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let transition: CATransition = caBorrow(handle) else { return false }
    return transition.subtype != nil
}

@_cdecl("ca_transition_clear_subtype")
public func ca_transition_clear_subtype(_ handle: UnsafeMutableRawPointer?) {
    guard let transition: CATransition = caBorrow(handle) else { return }
    transition.subtype = nil
}
