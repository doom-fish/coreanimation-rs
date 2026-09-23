use core::cell::RefCell;
use core::ffi::c_void;
use std::rc::Rc;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;

pub use crate::ffi::{CVSMPTETime, CVTime, CVTimeStamp};
pub use apple_cf::raw::{CVDisplayLinkOutputCallback, CVReturn};

type OutputFn = dyn FnMut(&CVTimeStamp, &CVTimeStamp) + Send;
type OutputHandler = Mutex<Box<OutputFn>>;

struct DisplayLinkInner {
    ptr: *mut c_void,
    output: RefCell<Option<CallbackContext<OutputHandler>>>,
}

impl Drop for DisplayLinkInner {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::CVDisplayLinkStop(self.ptr);
            crate::ffi::CVDisplayLinkSetOutputCallback(self.ptr, None, core::ptr::null_mut());
            crate::ffi::CVDisplayLinkRelease(self.ptr);
        }
    }
}

#[derive(Clone)]
/// Safe wrapper around `CVDisplayLink`. See <https://developer.apple.com/documentation/corevideo/cvdisplaylink>.
pub struct DisplayLink {
    inner: Rc<DisplayLinkInner>,
}

impl core::fmt::Debug for DisplayLink {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DisplayLink")
            .field("ptr", &self.inner.ptr)
            .field("has_output_handler", &self.inner.output.borrow().is_some())
            .finish()
    }
}

const fn cv_result(status: CVReturn) -> Result<(), CVReturn> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

impl DisplayLink {
    /// Creates a `CVDisplayLink` for the active displays.
    pub fn with_active_displays() -> Result<Self, CVReturn> {
        let mut ptr = core::ptr::null_mut();
        let status = unsafe { crate::ffi::CVDisplayLinkCreateWithActiveCGDisplays(&mut ptr) };
        if status == 0 && !ptr.is_null() {
            Ok(Self {
                inner: Rc::new(DisplayLinkInner {
                    ptr,
                    output: RefCell::new(None),
                }),
            })
        } else {
            Err(status)
        }
    }

    #[must_use]
    /// Returns the raw `CVDisplayLinkRef` pointer.
    pub fn as_ptr(&self) -> *mut c_void {
        self.inner.ptr
    }

    /// Sets the display used by the display link.
    pub fn set_current_display(&self, display_id: u32) -> Result<(), CVReturn> {
        let status =
            unsafe { crate::ffi::CVDisplayLinkSetCurrentCGDisplay(self.inner.ptr, display_id) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[must_use]
    /// Returns the display identifier used by the display link.
    pub fn current_display(&self) -> u32 {
        unsafe { crate::ffi::CVDisplayLinkGetCurrentCGDisplay(self.inner.ptr) }
    }

    pub fn set_output_handler<F>(&self, handler: F) -> Result<(), CVReturn>
    where
        F: FnMut(&CVTimeStamp, &CVTimeStamp) + Send + 'static,
    {
        let handler: Box<OutputFn> = Box::new(handler);
        let context = CallbackContext::<OutputHandler>::new(Mutex::new(handler));
        let user_info = context.as_ptr();
        self.replace_output(
            Some(display_link_output_trampoline),
            user_info,
            Some(context),
        )
    }

    pub fn clear_output_handler(&self) -> Result<(), CVReturn> {
        self.replace_output(None, core::ptr::null_mut(), None)
    }

    #[allow(clippy::missing_safety_doc)]
    /// Installs the output callback for the display link.
    pub unsafe fn set_output_callback(
        &self,
        callback: CVDisplayLinkOutputCallback,
        user_info: *mut c_void,
    ) -> Result<(), CVReturn> {
        self.replace_output(callback, user_info, None)
    }

    fn replace_output(
        &self,
        callback: CVDisplayLinkOutputCallback,
        user_info: *mut c_void,
        context: Option<CallbackContext<OutputHandler>>,
    ) -> Result<(), CVReturn> {
        let ptr = self.inner.ptr;
        let running = self.is_running();
        if running {
            cv_result(unsafe { crate::ffi::CVDisplayLinkStop(ptr) })?;
        }
        let installed = cv_result(unsafe {
            crate::ffi::CVDisplayLinkSetOutputCallback(ptr, callback, user_info)
        });
        if installed.is_ok() {
            drop(self.inner.output.replace(context));
        }
        if running && callback.is_some() {
            cv_result(unsafe { crate::ffi::CVDisplayLinkStart(ptr) })?;
        }
        installed
    }

    /// Starts the display link.
    pub fn start(&self) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkStart(self.inner.ptr) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    /// Stops the display link.
    pub fn stop(&self) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkStop(self.inner.ptr) };
        if status == 0 {
            Ok(())
        } else {
            Err(status)
        }
    }

    #[must_use]
    /// Returns the nominal refresh period reported by the display link.
    pub fn nominal_output_video_refresh_period(&self) -> CVTime {
        unsafe { crate::ffi::CVDisplayLinkGetNominalOutputVideoRefreshPeriod(self.inner.ptr) }
    }

    #[must_use]
    /// Returns the measured refresh period reported by the display link.
    pub fn actual_output_video_refresh_period(&self) -> f64 {
        unsafe { crate::ffi::CVDisplayLinkGetActualOutputVideoRefreshPeriod(self.inner.ptr) }
    }

    #[must_use]
    /// Returns whether the display link is running.
    pub fn is_running(&self) -> bool {
        unsafe { crate::ffi::CVDisplayLinkIsRunning(self.inner.ptr) }
    }

    /// Returns the current timestamp from the display link.
    pub fn current_time(&self) -> Result<CVTimeStamp, CVReturn> {
        let mut out_time = core::mem::MaybeUninit::<CVTimeStamp>::zeroed();
        let status = unsafe {
            crate::ffi::CVDisplayLinkGetCurrentTime(self.inner.ptr, out_time.as_mut_ptr())
        };
        if status == 0 {
            Ok(unsafe { out_time.assume_init() })
        } else {
            Err(status)
        }
    }

    /// Translates a timestamp into the display link timebase.
    pub fn translate_time(&self, in_time: &CVTimeStamp) -> Result<CVTimeStamp, CVReturn> {
        let mut out_time = core::mem::MaybeUninit::<CVTimeStamp>::zeroed();
        let status = unsafe {
            crate::ffi::CVDisplayLinkTranslateTime(self.inner.ptr, in_time, out_time.as_mut_ptr())
        };
        if status == 0 {
            Ok(unsafe { out_time.assume_init() })
        } else {
            Err(status)
        }
    }
}

unsafe extern "C" fn display_link_output_trampoline(
    _display_link: apple_cf::raw::CVDisplayLinkRef,
    now: *const CVTimeStamp,
    output_time: *const CVTimeStamp,
    _flags_in: u64,
    _flags_out: *mut u64,
    context: *mut c_void,
) -> CVReturn {
    if now.is_null() || output_time.is_null() {
        return 0;
    }
    let (now, output_time) = unsafe { (&*now, &*output_time) };
    let _ = unsafe {
        CallbackContext::<OutputHandler>::with(context, "DisplayLink output handler", |handler| {
            let mut handler = handler.lock().unwrap_or_else(PoisonError::into_inner);
            handler(now, output_time);
        })
    };
    0
}
