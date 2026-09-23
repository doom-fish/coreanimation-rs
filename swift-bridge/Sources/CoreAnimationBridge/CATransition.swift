import CoreImage
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

@_cdecl("ca_transition_copy_filter_name")
public func ca_transition_copy_filter_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let transition: CATransition = caBorrow(handle), let filter = transition.filter as? CIFilter else {
        return nil
    }
    return caDup(filter.name)
}

@_cdecl("ca_transition_set_filter_name")
public func ca_transition_set_filter_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) -> Bool {
    guard let transition: CATransition = caBorrow(handle) else { return false }
    guard let name = caCString(name) else {
        transition.filter = nil
        return true
    }
    guard CIFilter.filterNames(inCategory: kCICategoryTransition).contains(name),
          let filter = CIFilter(name: name)
    else {
        return false
    }
    transition.filter = filter
    return true
}
