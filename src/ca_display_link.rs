use core::ffi::c_void;

#[derive(Debug)]
pub struct QuartzDisplayLink {
    ptr: *mut c_void,
}

impl QuartzDisplayLink {
    #[must_use]
    pub fn new_main_screen() -> Option<Self> {
        let ptr = unsafe { crate::ffi::ca_quartz_display_link_new_main_screen() };
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn add_to_main_run_loop(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_add_to_main_run_loop(self.ptr) };
    }

    pub fn remove_from_main_run_loop(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_remove_from_main_run_loop(self.ptr) };
    }

    pub fn invalidate(&self) {
        unsafe { crate::ffi::ca_quartz_display_link_invalidate(self.ptr) };
    }

    #[must_use]
    pub fn is_paused(&self) -> bool {
        unsafe { crate::ffi::ca_quartz_display_link_is_paused(self.ptr) }
    }

    pub fn set_paused(&self, paused: bool) {
        unsafe { crate::ffi::ca_quartz_display_link_set_paused(self.ptr, paused) };
    }

    #[must_use]
    pub fn timestamp(&self) -> f64 {
        unsafe { crate::ffi::ca_quartz_display_link_get_timestamp(self.ptr) }
    }

    #[must_use]
    pub fn duration(&self) -> f64 {
        unsafe { crate::ffi::ca_quartz_display_link_get_duration(self.ptr) }
    }

    #[must_use]
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
