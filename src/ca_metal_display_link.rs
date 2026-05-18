use core::ffi::c_void;

use crate::ca_frame_rate_range::FrameRateRange;
use crate::layer::{LayerLike, MetalDrawable, MetalLayer};
use crate::private::handle_type;

handle_type!(MetalDisplayLinkUpdate);

struct MetalDisplayLinkDelegateContext {
    callback: Box<dyn FnMut(MetalDisplayLinkUpdate)>,
}

/// Safe wrapper around `CAMetalDisplayLink`. See <https://developer.apple.com/documentation/quartzcore/cametaldisplaylink>.
pub struct MetalDisplayLink {
    ptr: *mut c_void,
    delegate_context: Option<*mut MetalDisplayLinkDelegateContext>,
}

impl core::fmt::Debug for MetalDisplayLink {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MetalDisplayLink")
            .field("ptr", &self.ptr)
            .field("has_delegate", &self.delegate_context.is_some())
            .finish()
    }
}

impl MetalDisplayLink {
    #[must_use]
    /// Returns whether `CAMetalDisplayLink` is available on this system.
    pub fn is_available() -> bool {
        unsafe { crate::ffi::ca_metal_display_link_is_available() }
    }

    #[must_use]
    /// Creates a `CAMetalDisplayLink` for a Metal layer.
    pub fn new(layer: &MetalLayer) -> Option<Self> {
        let ptr = unsafe { crate::ffi::ca_metal_display_link_new(layer.as_layer_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                delegate_context: None,
            })
        }
    }

    /// Adds the Metal display link to the current run loop.
    pub fn add_to_current_run_loop(&self) {
        unsafe { crate::ffi::ca_metal_display_link_add_to_current_run_loop(self.ptr) };
    }

    /// Removes the Metal display link from the current run loop.
    pub fn remove_from_current_run_loop(&self) {
        unsafe { crate::ffi::ca_metal_display_link_remove_from_current_run_loop(self.ptr) };
    }

    /// Invalidates the Metal display link.
    pub fn invalidate(&self) {
        unsafe { crate::ffi::ca_metal_display_link_invalidate(self.ptr) };
    }

    #[must_use]
    /// Returns whether the Metal display link is paused.
    pub fn is_paused(&self) -> bool {
        unsafe { crate::ffi::ca_metal_display_link_is_paused(self.ptr) }
    }

    /// Pauses or resumes the Metal display link.
    pub fn set_paused(&self, value: bool) {
        unsafe { crate::ffi::ca_metal_display_link_set_paused(self.ptr, value) };
    }

    #[must_use]
    /// Returns the preferred frame latency.
    pub fn preferred_frame_latency(&self) -> f32 {
        unsafe { crate::ffi::ca_metal_display_link_get_preferred_frame_latency(self.ptr) }
    }

    /// Sets the preferred frame latency.
    pub fn set_preferred_frame_latency(&self, value: f32) {
        unsafe { crate::ffi::ca_metal_display_link_set_preferred_frame_latency(self.ptr, value) };
    }

    #[must_use]
    /// Returns the preferred frame-rate range.
    pub fn preferred_frame_rate_range(&self) -> FrameRateRange {
        let mut range = FrameRateRange::default();
        unsafe {
            crate::ffi::ca_metal_display_link_get_preferred_frame_rate_range(
                self.ptr,
                (&mut range as *mut FrameRateRange).cast(),
            )
        };
        range
    }

    /// Sets the preferred frame-rate range.
    pub fn set_preferred_frame_rate_range(&self, range: FrameRateRange) {
        unsafe {
            crate::ffi::ca_metal_display_link_set_preferred_frame_rate_range(
                self.ptr,
                (&range as *const FrameRateRange).cast(),
            )
        };
    }

    /// Installs a callback that receives Metal display-link updates.
    pub fn set_delegate<F>(&mut self, callback: F)
    where
        F: FnMut(MetalDisplayLinkUpdate) + 'static,
    {
        self.clear_delegate();
        let context = Box::into_raw(Box::new(MetalDisplayLinkDelegateContext {
            callback: Box::new(callback),
        }));
        unsafe {
            crate::ffi::ca_metal_display_link_set_delegate(
                self.ptr,
                Some(metal_display_link_delegate_trampoline),
                context.cast(),
            )
        };
        self.delegate_context = Some(context);
    }

    /// Clears the Metal display-link callback.
    pub fn clear_delegate(&mut self) {
        unsafe {
            crate::ffi::ca_metal_display_link_set_delegate(self.ptr, None, core::ptr::null_mut())
        };
        if let Some(context) = self.delegate_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    /// Runs the current run loop for the supplied number of seconds.
    pub fn run_current_run_loop_for(seconds: f64) {
        if seconds <= 0.0 {
            return;
        }
        unsafe { crate::ffi::ca_run_current_run_loop(seconds) };
    }
}

impl Drop for MetalDisplayLink {
    fn drop(&mut self) {
        self.clear_delegate();
        if !self.ptr.is_null() {
            unsafe {
                crate::ffi::ca_metal_display_link_invalidate(self.ptr);
                crate::ffi::ca_release(self.ptr);
            }
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl MetalDisplayLinkUpdate {
    #[must_use]
    /// Returns the update drawable, if one is available.
    pub fn drawable(&self) -> Option<MetalDrawable> {
        unsafe {
            MetalDrawable::from_raw(crate::ffi::ca_metal_display_link_update_get_drawable(
                self.as_ptr(),
            ))
        }
    }

    #[must_use]
    /// Returns the target timestamp for the update.
    pub fn target_timestamp(&self) -> f64 {
        unsafe { crate::ffi::ca_metal_display_link_update_get_target_timestamp(self.as_ptr()) }
    }

    #[must_use]
    /// Returns the target presentation timestamp for the update.
    pub fn target_presentation_timestamp(&self) -> f64 {
        unsafe {
            crate::ffi::ca_metal_display_link_update_get_target_presentation_timestamp(
                self.as_ptr(),
            )
        }
    }
}

unsafe extern "C" fn metal_display_link_delegate_trampoline(
    context: *mut c_void,
    update_handle: *mut c_void,
) {
    if context.is_null() || update_handle.is_null() {
        return;
    }

    let context = unsafe { &mut *context.cast::<MetalDisplayLinkDelegateContext>() };
    let update = unsafe { MetalDisplayLinkUpdate::from_raw_unchecked(update_handle) };
    (context.callback)(update);
}
