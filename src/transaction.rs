use core::mem::ManuallyDrop;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;

/// Completion callback type used by `CATransaction`.
pub type TransactionCompletion = unsafe extern "C" fn(context: *mut core::ffi::c_void);

type CompletionFn = dyn FnOnce() + Send;
type CompletionHandler = Mutex<Option<Box<CompletionFn>>>;

#[derive(Debug, Clone, Copy)]
/// Namespace for `CATransaction` helpers. See <https://developer.apple.com/documentation/quartzcore/catransaction>.
pub struct Transaction;

impl Transaction {
    /// Begins a `CATransaction` scope.
    pub fn begin() {
        unsafe { crate::ffi::ca_transaction_begin() };
    }

    /// Commits the current `CATransaction`.
    pub fn commit() {
        unsafe { crate::ffi::ca_transaction_commit() };
    }

    /// Flushes pending `Core Animation` transactions.
    pub fn flush() {
        unsafe { crate::ffi::ca_transaction_flush() };
    }

    #[must_use]
    /// Returns the current transaction animation duration.
    pub fn animation_duration() -> f64 {
        unsafe { crate::ffi::ca_transaction_get_animation_duration() }
    }

    /// Sets the current transaction animation duration.
    pub fn set_animation_duration(value: f64) {
        unsafe { crate::ffi::ca_transaction_set_animation_duration(value) };
    }

    #[must_use]
    /// Returns whether implicit actions are disabled for the current transaction.
    pub fn disable_actions() -> bool {
        unsafe { crate::ffi::ca_transaction_get_disable_actions() }
    }

    /// Sets whether implicit actions are disabled for the current transaction.
    pub fn set_disable_actions(value: bool) {
        unsafe { crate::ffi::ca_transaction_set_disable_actions(value) };
    }

    pub fn set_completion_handler<F>(handler: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let handler: Box<CompletionFn> = Box::new(handler);
        let context = ManuallyDrop::new(CallbackContext::<CompletionHandler>::new(Mutex::new(
            Some(handler),
        )));
        unsafe {
            crate::ffi::ca_transaction_set_completion_handler(
                Some(transaction_completion_trampoline),
                context.as_ptr(),
                Some(CallbackContext::<CompletionHandler>::RELEASE),
            )
        };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Sets the completion callback for the current `CATransaction`.
    pub unsafe fn set_completion_block(
        callback: Option<TransactionCompletion>,
        context: *mut core::ffi::c_void,
    ) {
        unsafe { crate::ffi::ca_transaction_set_completion_block(callback, context) };
    }
}

unsafe extern "C" fn transaction_completion_trampoline(context: *mut core::ffi::c_void) {
    let _ = unsafe {
        CallbackContext::<CompletionHandler>::with(
            context,
            "Transaction completion handler",
            |slot| {
                let handler = slot.lock().unwrap_or_else(PoisonError::into_inner).take();
                if let Some(handler) = handler {
                    handler();
                }
            },
        )
    };
}
