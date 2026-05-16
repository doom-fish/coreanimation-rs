use crate::ca_media_timing::TimingFunctionName;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct TransactionLockGuard {
    _private: (),
}

impl Drop for TransactionLockGuard {
    fn drop(&mut self) {
        Transaction::unlock();
    }
}

impl Transaction {
    pub fn lock() {
        unsafe { crate::ffi::ca_transaction_lock() };
    }

    pub fn unlock() {
        unsafe { crate::ffi::ca_transaction_unlock() };
    }

    #[must_use]
    pub fn lock_guard() -> TransactionLockGuard {
        Self::lock();
        TransactionLockGuard { _private: () }
    }

    #[must_use]
    pub fn animation_timing_function_name() -> Option<TimingFunctionName> {
        TimingFunctionName::from_raw(unsafe {
            crate::ffi::ca_transaction_get_animation_timing_function_name()
        })
    }

    pub fn set_animation_timing_function_name(value: Option<TimingFunctionName>) {
        unsafe {
            crate::ffi::ca_transaction_set_animation_timing_function_name(
                value.map_or(-1, TimingFunctionName::raw),
            )
        };
    }
}
