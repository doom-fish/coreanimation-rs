use core::ffi::c_void;

pub use crate::ffi::{CVSMPTETime, CVTime, CVTimeStamp};

pub type CVReturn = i32;

#[derive(Debug)]
pub struct DisplayLink {
    ptr: *mut c_void,
}

impl Clone for DisplayLink {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { crate::ffi::CVDisplayLinkRetain(self.ptr) },
        }
    }
}

impl Drop for DisplayLink {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { crate::ffi::CVDisplayLinkRelease(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl DisplayLink {
    pub fn with_active_displays() -> Result<Self, CVReturn> {
        let mut ptr = core::ptr::null_mut();
        let status = unsafe { crate::ffi::CVDisplayLinkCreateWithActiveCGDisplays(&mut ptr) };
        if status == 0 && !ptr.is_null() {
            Ok(Self { ptr })
        } else {
            Err(status)
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn set_current_display(&self, display_id: u32) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkSetCurrentCGDisplay(self.ptr, display_id) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[must_use]
    pub fn current_display(&self) -> u32 {
        unsafe { crate::ffi::CVDisplayLinkGetCurrentCGDisplay(self.ptr) }
    }

    pub fn set_output_callback(
        &self,
        callback: crate::ffi::CVDisplayLinkOutputCallback,
        user_info: *mut c_void,
    ) -> Result<(), CVReturn> {
        let status =
            unsafe { crate::ffi::CVDisplayLinkSetOutputCallback(self.ptr, callback, user_info) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn start(&self) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkStart(self.ptr) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    pub fn stop(&self) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkStop(self.ptr) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[must_use]
    pub fn nominal_output_video_refresh_period(&self) -> CVTime {
        unsafe { crate::ffi::CVDisplayLinkGetNominalOutputVideoRefreshPeriod(self.ptr) }
    }

    #[must_use]
    pub fn actual_output_video_refresh_period(&self) -> f64 {
        unsafe { crate::ffi::CVDisplayLinkGetActualOutputVideoRefreshPeriod(self.ptr) }
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        unsafe { crate::ffi::CVDisplayLinkIsRunning(self.ptr) }
    }

    pub fn current_time(&self) -> Result<CVTimeStamp, CVReturn> {
        let mut out_time = CVTimeStamp::default();
        let status = unsafe { crate::ffi::CVDisplayLinkGetCurrentTime(self.ptr, &mut out_time) };
        if status == 0 {
            Ok(out_time)
        } else {
            Err(status)
        }
    }

    pub fn translate_time(&self, in_time: &CVTimeStamp) -> Result<CVTimeStamp, CVReturn> {
        let mut out_time = CVTimeStamp::default();
        let status =
            unsafe { crate::ffi::CVDisplayLinkTranslateTime(self.ptr, in_time, &mut out_time) };
        if status == 0 {
            Ok(out_time)
        } else {
            Err(status)
        }
    }
}
