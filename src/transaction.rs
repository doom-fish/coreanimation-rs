pub type TransactionCompletion = unsafe extern "C" fn(context: *mut core::ffi::c_void);

#[derive(Debug, Clone, Copy)]
pub struct Transaction;

impl Transaction {
    pub fn begin() {
        unsafe { crate::ffi::ca_transaction_begin() };
    }

    pub fn commit() {
        unsafe { crate::ffi::ca_transaction_commit() };
    }

    pub fn flush() {
        unsafe { crate::ffi::ca_transaction_flush() };
    }

    #[must_use]
    pub fn animation_duration() -> f64 {
        unsafe { crate::ffi::ca_transaction_get_animation_duration() }
    }

    pub fn set_animation_duration(value: f64) {
        unsafe { crate::ffi::ca_transaction_set_animation_duration(value) };
    }

    #[must_use]
    pub fn disable_actions() -> bool {
        unsafe { crate::ffi::ca_transaction_get_disable_actions() }
    }

    pub fn set_disable_actions(value: bool) {
        unsafe { crate::ffi::ca_transaction_set_disable_actions(value) };
    }

    pub fn set_completion_block(
        callback: Option<TransactionCompletion>,
        context: *mut core::ffi::c_void,
    ) {
        unsafe { crate::ffi::ca_transaction_set_completion_block(callback, context) };
    }
}
