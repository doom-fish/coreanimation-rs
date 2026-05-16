import Foundation
import QuartzCore

public typealias CAMetalDisplayLinkUpdateCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Void

@available(macOS 14.0, *)
final class CAMetalDisplayLinkDelegateBox: NSObject, CAMetalDisplayLinkDelegate {
    let callback: CAMetalDisplayLinkUpdateCallback
    let context: UnsafeMutableRawPointer?

    init(
        callback: @escaping CAMetalDisplayLinkUpdateCallback,
        context: UnsafeMutableRawPointer?
    ) {
        self.callback = callback
        self.context = context
    }

    func metalDisplayLink(_ link: CAMetalDisplayLink, needsUpdate update: CAMetalDisplayLink.Update) {
        callback(context, caRetain(update))
    }
}

@available(macOS 14.0, *)
final class CAMetalDisplayLinkBox {
    let link: CAMetalDisplayLink
    var delegateBox: CAMetalDisplayLinkDelegateBox?

    init(link: CAMetalDisplayLink) {
        self.link = link
    }
}

@available(macOS 14.0, *)
private func caBorrowMetalDisplayLinkBox(_ handle: UnsafeMutableRawPointer?) -> CAMetalDisplayLinkBox? {
    guard let handle else { return nil }
    return Unmanaged<CAMetalDisplayLinkBox>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("ca_run_current_run_loop")
public func ca_run_current_run_loop(_ seconds: Double) {
    guard seconds > 0 else { return }
    RunLoop.current.run(until: Date().addingTimeInterval(seconds))
}

@_cdecl("ca_metal_display_link_is_available")
public func ca_metal_display_link_is_available() -> Bool {
    if #available(macOS 14.0, *) {
        return true
    }
    return false
}

@_cdecl("ca_metal_display_link_new")
public func ca_metal_display_link_new(_ layerHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let layer: CAMetalLayer = caBorrow(layerHandle) else {
        return nil
    }
    return caRetain(CAMetalDisplayLinkBox(link: CAMetalDisplayLink(metalLayer: layer)))
}

@_cdecl("ca_metal_display_link_add_to_current_run_loop")
public func ca_metal_display_link_add_to_current_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    box.link.add(to: .current, forMode: .default)
}

@_cdecl("ca_metal_display_link_remove_from_current_run_loop")
public func ca_metal_display_link_remove_from_current_run_loop(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    box.link.remove(from: .current, forMode: .default)
}

@_cdecl("ca_metal_display_link_invalidate")
public func ca_metal_display_link_invalidate(_ handle: UnsafeMutableRawPointer?) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    box.link.invalidate()
}

@_cdecl("ca_metal_display_link_is_paused")
public func ca_metal_display_link_is_paused(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else {
        return false
    }
    return box.link.isPaused
}

@_cdecl("ca_metal_display_link_set_paused")
public func ca_metal_display_link_set_paused(_ handle: UnsafeMutableRawPointer?, _ value: Bool) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    box.link.isPaused = value
}

@_cdecl("ca_metal_display_link_get_preferred_frame_latency")
public func ca_metal_display_link_get_preferred_frame_latency(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else {
        return 0
    }
    return box.link.preferredFrameLatency
}

@_cdecl("ca_metal_display_link_set_preferred_frame_latency")
public func ca_metal_display_link_set_preferred_frame_latency(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    box.link.preferredFrameLatency = value
}

@_cdecl("ca_metal_display_link_set_delegate")
public func ca_metal_display_link_set_delegate(
    _ handle: UnsafeMutableRawPointer?,
    _ callback: CAMetalDisplayLinkUpdateCallback?,
    _ context: UnsafeMutableRawPointer?
) {
    guard #available(macOS 14.0, *), let box = caBorrowMetalDisplayLinkBox(handle) else { return }
    guard let callback else {
        box.delegateBox = nil
        box.link.delegate = nil
        return
    }
    let delegate = CAMetalDisplayLinkDelegateBox(callback: callback, context: context)
    box.delegateBox = delegate
    box.link.delegate = delegate
}

@_cdecl("ca_metal_display_link_update_get_drawable")
public func ca_metal_display_link_update_get_drawable(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let update: CAMetalDisplayLink.Update = caBorrow(handle) else {
        return nil
    }
    return caRetain(update.drawable as AnyObject)
}

@_cdecl("ca_metal_display_link_update_get_target_timestamp")
public func ca_metal_display_link_update_get_target_timestamp(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let update: CAMetalDisplayLink.Update = caBorrow(handle) else {
        return 0
    }
    return update.targetTimestamp
}

@_cdecl("ca_metal_display_link_update_get_target_presentation_timestamp")
public func ca_metal_display_link_update_get_target_presentation_timestamp(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard #available(macOS 14.0, *), let update: CAMetalDisplayLink.Update = caBorrow(handle) else {
        return 0
    }
    return update.targetPresentationTimestamp
}
