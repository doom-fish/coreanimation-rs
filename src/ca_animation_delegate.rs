use core::ffi::c_void;
use std::sync::{Mutex, PoisonError, TryLockError};

use doom_fish_utils::callback_context::CallbackContext;

use crate::animation::{Animation, AnimationLike};
use crate::ca_frame_rate_range::FrameRateRange;

type DidStartFn = dyn FnMut(Animation) + Send;
type DidStopFn = dyn FnMut(Animation, bool) + Send;

#[derive(Default)]
struct AnimationDelegateState {
    did_start: Mutex<Option<Box<DidStartFn>>>,
    did_stop: Mutex<Option<Box<DidStopFn>>>,
}

fn slot_is_set<T>(slot: &Mutex<Option<T>>) -> bool {
    match slot.try_lock() {
        Ok(guard) => guard.is_some(),
        Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner().is_some(),
        Err(TryLockError::WouldBlock) => true,
    }
}

pub struct AnimationDelegate {
    ptr: *mut c_void,
    state: CallbackContext<AnimationDelegateState>,
}

impl core::fmt::Debug for AnimationDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AnimationDelegate")
            .field("ptr", &self.ptr)
            .field("has_did_start", &slot_is_set(&self.state.get().did_start))
            .field("has_did_stop", &slot_is_set(&self.state.get().did_stop))
            .finish()
    }
}

impl AnimationDelegate {
    #[must_use]
    pub fn new() -> Option<Self> {
        let state = CallbackContext::new(AnimationDelegateState::default());
        let ptr = unsafe {
            crate::ffi::ca_animation_delegate_new(
                Some(animation_delegate_did_start_trampoline),
                Some(animation_delegate_did_stop_trampoline),
                state.retained_ptr(),
                Some(CallbackContext::<AnimationDelegateState>::RELEASE),
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, state })
        }
    }

    pub fn set_did_start<F>(&mut self, callback: F)
    where
        F: FnMut(Animation) + Send + 'static,
    {
        let callback: Box<DidStartFn> = Box::new(callback);
        *self
            .state
            .get()
            .did_start
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(callback);
    }

    pub fn clear_did_start(&mut self) {
        let previous = self
            .state
            .get()
            .did_start
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        drop(previous);
    }

    pub fn set_did_stop<F>(&mut self, callback: F)
    where
        F: FnMut(Animation, bool) + Send + 'static,
    {
        let callback: Box<DidStopFn> = Box::new(callback);
        *self
            .state
            .get()
            .did_stop
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(callback);
    }

    pub fn clear_did_stop(&mut self) {
        let previous = self
            .state
            .get()
            .did_stop
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        drop(previous);
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for AnimationDelegate {
    fn drop(&mut self) {
        self.state.deactivate();
        if !self.ptr.is_null() {
            unsafe { crate::ffi::ca_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl Animation {
    #[must_use]
    pub fn retained_from<A: AnimationLike>(animation: &A) -> Self {
        let ptr = unsafe { crate::ffi::ca_retain(animation.as_animation_ptr()) };
        debug_assert!(!ptr.is_null());
        unsafe { Self::from_raw_unchecked(ptr) }
    }

    pub fn set_delegate(&self, delegate: Option<&AnimationDelegate>) {
        unsafe {
            crate::ffi::ca_animation_set_delegate(
                self.as_ptr(),
                delegate.map_or(core::ptr::null_mut(), AnimationDelegate::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn supports_preferred_frame_rate_range() -> bool {
        unsafe { crate::ffi::ca_animation_supports_preferred_frame_rate_range() }
    }

    #[must_use]
    pub fn preferred_frame_rate_range(&self) -> FrameRateRange {
        let mut range = FrameRateRange::default();
        unsafe {
            crate::ffi::ca_animation_get_preferred_frame_rate_range(
                self.as_ptr(),
                (&mut range as *mut FrameRateRange).cast(),
            )
        };
        range
    }

    pub fn set_preferred_frame_rate_range(&self, range: FrameRateRange) {
        unsafe {
            crate::ffi::ca_animation_set_preferred_frame_rate_range(
                self.as_ptr(),
                (&range as *const FrameRateRange).cast(),
            )
        };
    }
}

#[must_use]
pub fn current_media_time() -> f64 {
    unsafe { crate::ffi::CACurrentMediaTime() }
}

unsafe extern "C" fn animation_delegate_did_start_trampoline(
    context: *mut c_void,
    animation_handle: *mut c_void,
) {
    if animation_handle.is_null() {
        return;
    }
    let animation = unsafe { Animation::from_raw_unchecked(animation_handle) };
    let _ = unsafe {
        CallbackContext::<AnimationDelegateState>::with(
            context,
            "AnimationDelegate did_start callback",
            move |state| {
                let mut slot = state
                    .did_start
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner);
                if let Some(callback) = slot.as_mut() {
                    callback(animation);
                }
            },
        )
    };
}

unsafe extern "C" fn animation_delegate_did_stop_trampoline(
    context: *mut c_void,
    animation_handle: *mut c_void,
    finished: bool,
) {
    if animation_handle.is_null() {
        return;
    }
    let animation = unsafe { Animation::from_raw_unchecked(animation_handle) };
    let _ = unsafe {
        CallbackContext::<AnimationDelegateState>::with(
            context,
            "AnimationDelegate did_stop callback",
            move |state| {
                let mut slot = state
                    .did_stop
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner);
                if let Some(callback) = slot.as_mut() {
                    callback(animation, finished);
                }
            },
        )
    };
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use super::{current_media_time, Animation, AnimationDelegate};

    #[test]
    fn caanimationdelegate_callbacks_fire() {
        let animation = Animation::new().expect("animation");
        let did_start = Arc::new(AtomicBool::new(false));
        let did_stop = Arc::new(AtomicBool::new(false));
        let finished = Arc::new(AtomicBool::new(false));

        let mut delegate = AnimationDelegate::new().expect("delegate");
        delegate.set_did_start({
            let did_start = Arc::clone(&did_start);
            move |_animation| {
                did_start.store(true, Ordering::SeqCst);
            }
        });
        delegate.set_did_stop({
            let did_stop = Arc::clone(&did_stop);
            let finished_flag = Arc::clone(&finished);
            move |_animation, value| {
                did_stop.store(true, Ordering::SeqCst);
                finished_flag.store(value, Ordering::SeqCst);
            }
        });
        animation.set_delegate(Some(&delegate));

        unsafe {
            crate::ffi::ca_animation_invoke_delegate_did_start(animation.as_ptr());
            crate::ffi::ca_animation_invoke_delegate_did_stop(animation.as_ptr(), true);
        }

        assert!(did_start.load(Ordering::SeqCst));
        assert!(did_stop.load(Ordering::SeqCst));
        assert!(finished.load(Ordering::SeqCst));
        assert!(current_media_time() >= 0.0);
    }
}
