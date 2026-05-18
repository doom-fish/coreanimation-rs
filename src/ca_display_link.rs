use core::ffi::c_void;

#[derive(Debug)]
/// Safe wrapper around `CADisplayLink`. See <https://developer.apple.com/documentation/quartzcore/cadisplaylink>.
pub struct QuartzDisplayLink {
    ptr: *mut c_void,
}

impl QuartzDisplayLink {
    #[must_use]
    /// Creates a `CADisplayLink` wrapper for the main screen.
    pub fn new_main_screen() -> Option<Self> {
        let ptr = unsafe { crate::ffi::ca_quartz_display_link_new_main_screen() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Adds the display link to the main run loop.
    pub fn add_to_main_run_loop(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_add_to_main_run_loop(self.ptr) };
    }

    /// Removes the display link from the main run loop.
    pub fn remove_from_main_run_loop(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_remove_from_main_run_loop(self.ptr) };
    }

    /// Invalidates the display link.
    pub fn invalidate(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_invalidate(self.ptr) };
    }

    #[must_use]
    /// Returns whether the display link is paused.
    pub fn is_paused(&self) -> bool {
        unsafe { crate::ffi::ca_quartz_display_link_is_paused(self.ptr) }
    }

    /// Pauses or resumes the display link.
    pub fn set_paused(&self, paused: bool) {
        unsafe { crate::ffi::ca_quartz_display_link_set_paused(self.ptr, paused) };
    }

    #[must_use]
    /// Returns the display link timestamp.
    pub fn timestamp(&self) -> f64 {
        unsafe { crate::ffi::ca_quartz_display_link_get_timestamp(self.ptr) }
    }

    #[must_use]
    /// Returns the display link frame duration.
    pub fn duration(&self) -> f64 {
        unsafe { crate::ffi::ca_quartz_display_link_get_duration(self.ptr) }
    }

    #[must_use]
    /// Returns the display link target timestamp.
    pub fn target_timestamp(&self) -> f64 {
        unsafe { crate::ffi::ca_quartz_display_link_get_target_timestamp(self.ptr) }
    }
}

impl Drop for QuartzDisplayLink {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                crate::ffi::ca_quartz_display_link_invalidate(self.ptr);
                crate::ffi::ca_release(self.ptr);
            }
            self.ptr = core::ptr::null_mut();
        }
    }
}
