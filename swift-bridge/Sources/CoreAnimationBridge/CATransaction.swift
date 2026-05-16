import Foundation
import QuartzCore

@_cdecl("ca_transaction_lock")
public func ca_transaction_lock() {
    CATransaction.lock()
}

@_cdecl("ca_transaction_unlock")
public func ca_transaction_unlock() {
    CATransaction.unlock()
}

@_cdecl("ca_transaction_get_animation_timing_function_name")
public func ca_transaction_get_animation_timing_function_name() -> Int32 {
    caTimingFunctionNameRaw(CATransaction.animationTimingFunction())
}

@_cdecl("ca_transaction_set_animation_timing_function_name")
public func ca_transaction_set_animation_timing_function_name(_ value: Int32) {
    CATransaction.setAnimationTimingFunction(
        value < 0 ? nil : CAMediaTimingFunction(name: caTimingFunctionName(value))
    )
}
