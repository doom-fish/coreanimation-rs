use core::ffi::c_void;

use crate::animation::{Animation, AnimationLike};
use crate::ca_frame_rate_range::FrameRateRange;

struct AnimationDidStartContext {
    callback: Box<dyn FnMut(Animation)>,
}

struct AnimationDidStopContext {
    callback: Box<dyn FnMut(Animation, bool)>,
}

pub struct AnimationDelegate {
    ptr: *mut c_void,
    did_start_context: Option<*mut AnimationDidStartContext>,
    did_stop_context: Option<*mut AnimationDidStopContext>,
}

impl core::fmt::Debug for AnimationDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AnimationDelegate")
            .field("ptr", &self.ptr)
            .field("has_did_start", &self.did_start_context.is_some())
            .field("has_did_stop", &self.did_stop_context.is_some())
            .finish()
    }
}

impl AnimationDelegate {
    #[must_use]
    pub fn new() -> Option<Self> {
        let ptr = unsafe { crate::ffi::ca_animation_delegate_new() };
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                did_start_context: None,
                did_stop_context: None,
            })
        }
    }

    pub fn set_did_start<F>(&mut self, callback: F)
    where
        F: FnMut(Animation) + 'static,
    {
        self.clear_did_start();
        let context = Box::into_raw(Box::new(AnimationDidStartContext {
            callback: Box::new(callback),
        }));
        unsafe {
            crate::ffi::ca_animation_delegate_set_did_start_callback(
                self.ptr,
                Some(animation_delegate_did_start_trampoline),
                context.cast(),
            )
        };
        self.did_start_context = Some(context);
    }

    pub fn clear_did_start(&mut self) {
        unsafe {
            crate::ffi::ca_animation_delegate_set_did_start_callback(
                self.ptr,
                None,
                core::ptr::null_mut(),
            )
        };
        if let Some(context) = self.did_start_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    pub fn set_did_stop<F>(&mut self, callback: F)
    where
        F: FnMut(Animation, bool) + 'static,
    {
        self.clear_did_stop();
        let context = Box::into_raw(Box::new(AnimationDidStopContext {
            callback: Box::new(callback),
        }));
        unsafe {
            crate::ffi::ca_animation_delegate_set_did_stop_callback(
                self.ptr,
                Some(animation_delegate_did_stop_trampoline),
                context.cast(),
            )
        };
        self.did_stop_context = Some(context);
    }

    pub fn clear_did_stop(&mut self) {
        unsafe {
            crate::ffi::ca_animation_delegate_set_did_stop_callback(
                self.ptr,
                None,
                core::ptr::null_mut(),
            )
        };
        if let Some(context) = self.did_stop_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for AnimationDelegate {
    fn drop(&mut self) {
        self.clear_did_start();
        self.clear_did_stop();
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
    if context.is_null() || animation_handle.is_null() {
        return;
    }

    let context = unsafe { &mut *context.cast::<AnimationDidStartContext>() };
    let animation = unsafe { Animation::from_raw_unchecked(animation_handle) };
    (context.callback)(animation);
}

unsafe extern "C" fn animation_delegate_did_stop_trampoline(
    context: *mut c_void,
    animation_handle: *mut c_void,
    finished: bool,
) {
    if context.is_null() || animation_handle.is_null() {
        return;
    }

    let context = unsafe { &mut *context.cast::<AnimationDidStopContext>() };
    let animation = unsafe { Animation::from_raw_unchecked(animation_handle) };
    (context.callback)(animation, finished);
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
