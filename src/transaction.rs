/// Completion callback type used by `CATransaction`.
pub type TransactionCompletion = unsafe extern "C" fn(context: *mut core::ffi::c_void);

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

    /// Sets the completion callback for the current `CATransaction`.
    pub fn set_completion_block(
        callback: Option<TransactionCompletion>,
        context: *mut core::ffi::c_void,
    ) {
        unsafe { crate::ffi::ca_transaction_set_completion_block(callback, context) };
    }
}
