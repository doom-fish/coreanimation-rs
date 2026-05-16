import Foundation
import QuartzCore

public typealias CATransactionCompletionCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class TransactionCompletionBox {
    let callback: CATransactionCompletionCallback
    let context: UnsafeMutableRawPointer?

    init(callback: @escaping CATransactionCompletionCallback, context: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.context = context
    }

    func call() {
        callback(context)
    }
}

@_cdecl("ca_transaction_begin")
public func ca_transaction_begin() {
    CATransaction.begin()
}

@_cdecl("ca_transaction_commit")
public func ca_transaction_commit() {
    CATransaction.commit()
}

@_cdecl("ca_transaction_flush")
public func ca_transaction_flush() {
    CATransaction.flush()
}

@_cdecl("ca_transaction_get_animation_duration")
public func ca_transaction_get_animation_duration() -> Double {
    CATransaction.animationDuration()
}

@_cdecl("ca_transaction_set_animation_duration")
public func ca_transaction_set_animation_duration(_ value: Double) {
    CATransaction.setAnimationDuration(value)
}

@_cdecl("ca_transaction_get_disable_actions")
public func ca_transaction_get_disable_actions() -> Bool {
    CATransaction.disableActions()
}

@_cdecl("ca_transaction_set_disable_actions")
public func ca_transaction_set_disable_actions(_ value: Bool) {
    CATransaction.setDisableActions(value)
}

@_cdecl("ca_transaction_set_completion_block")
public func ca_transaction_set_completion_block(_ callback: CATransactionCompletionCallback?, _ context: UnsafeMutableRawPointer?) {
    guard let callback else {
        CATransaction.setCompletionBlock(nil)
        return
    }
    let box = TransactionCompletionBox(callback: callback, context: context)
    CATransaction.setCompletionBlock {
        box.call()
    }
}
