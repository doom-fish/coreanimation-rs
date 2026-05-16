import AppKit
import Foundation
import QuartzCore

@available(macOS 14.0, *)
@objc final class CAQuartzDisplayLinkTarget: NSObject {
    @objc func tick(_ sender: CADisplayLink) {}
}

@available(macOS 14.0, *)
final class CAQuartzDisplayLinkBox {
    let target: CAQuartzDisplayLinkTarget
    let link: CADisplayLink

    init(target: CAQuartzDisplayLinkTarget, link: CADisplayLink) {
        self.target = target
        self.link = link
    }
}

@available(macOS 14.0, *)
private func caBorrowQuartzDisplayLinkBox(_ handle: UnsafeMutableRawPointer?) -> CAQuartzDisplayLinkBox? {
    guard let handle else { return nil }
    return Unmanaged<CAQuartzDisplayLinkBox>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("ca_quartz_display_link_new_main_screen")
public func ca_quartz_display_link_new_main_screen() -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), Thread.isMainThread else { return nil }
    guard let screen = NSScreen.screens.first ?? NSScreen.main else { return nil }
    let target = CAQuartzDisplayLinkTarget()
    let link = screen.displayLink(target: target, selector: #selector(CAQuartzDisplayLinkTarget.tick(_:)))
    return caRetain(CAQuartzDisplayLinkBox(target: target, link: link))
}

@_cdecl("ca_quartz_display_link_add_to_main_run_loop")
public func ca_quartz_display_link_add_to_main_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), Thread.isMainThread, let box = caBorrowQuartzDisplayLinkBox(handle) else { return }
    box.link.add(to: .main, forMode: .default)
}

@_cdecl("ca_quartz_display_link_remove_from_main_run_loop")
public func ca_quartz_display_link_remove_from_main_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), Thread.isMainThread, let box = caBorrowQuartzDisplayLinkBox(handle) else { return }
    box.link.remove(from: .main, forMode: .default)
}

@_cdecl("ca_quartz_display_link_invalidate")
public func ca_quartz_display_link_invalidate(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return }
    box.link.invalidate()
}

@_cdecl("ca_quartz_display_link_is_paused")
public func ca_quartz_display_link_is_paused(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return false }
    return box.link.isPaused
}

@_cdecl("ca_quartz_display_link_set_paused")
public func ca_quartz_display_link_set_paused(_ handle: UnsafeMutableRawPointer?, _ paused: Bool) {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return }
    box.link.isPaused = paused
}

@_cdecl("ca_quartz_display_link_get_timestamp")
public func ca_quartz_display_link_get_timestamp(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return 0 }
    return box.link.timestamp
}

@_cdecl("ca_quartz_display_link_get_duration")
public func ca_quartz_display_link_get_duration(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return 0 }
    return box.link.duration
}

@_cdecl("ca_quartz_display_link_get_target_timestamp")
public func ca_quartz_display_link_get_target_timestamp(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let box = caBorrowQuartzDisplayLinkBox(handle) else { return 0 }
    return box.link.targetTimestamp
}
