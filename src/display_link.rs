use core::cell::RefCell;
use core::ffi::c_void;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

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

type ImmortalOutput = dyn Fn(&CVTimeStamp, &CVTimeStamp) + Send + Sync + 'static;
const CV_RETURN_ERROR: CVReturn = -6660;

struct ImmortalCVDisplayLink(NonNull<c_void>);

unsafe impl Send for ImmortalCVDisplayLink {}
unsafe impl Sync for ImmortalCVDisplayLink {}

struct ImmortalDisplayLinkInner {
    ptr: ImmortalCVDisplayLink,
    display_id: u32,
    running: AtomicBool,
    delivering: AtomicBool,
    transition: Mutex<()>,
    output: Mutex<Option<Arc<ImmortalOutput>>>,
}

#[derive(Default)]
struct ImmortalDisplayLinkSlot {
    initialization: Mutex<()>,
    link: OnceLock<Arc<ImmortalDisplayLinkInner>>,
}

static IMMORTAL_LINKS: Mutex<BTreeMap<u32, Arc<ImmortalDisplayLinkSlot>>> =
    Mutex::new(BTreeMap::new());

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[derive(Clone)]
pub struct ImmortalDisplayLink {
    inner: Arc<ImmortalDisplayLinkInner>,
}

impl core::fmt::Debug for ImmortalDisplayLink {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ImmortalDisplayLink")
            .field("display_id", &self.inner.display_id)
            .field("running", &self.is_running())
            .finish()
    }
}

impl ImmortalDisplayLink {
    pub fn for_display(display_id: u32) -> Result<Self, CVReturn> {
        let slot = {
            let mut links = lock(&IMMORTAL_LINKS);
            Arc::clone(
                links
                    .entry(display_id)
                    .or_insert_with(|| Arc::new(ImmortalDisplayLinkSlot::default())),
            )
        };
        let initialization = lock(&slot.initialization);
        let inner = if let Some(inner) = slot.link.get() {
            Arc::clone(inner)
        } else {
            let mut ptr = core::ptr::null_mut();
            cv_result(unsafe {
                crate::ffi::CVDisplayLinkCreateWithActiveCGDisplays(&raw mut ptr)
            })?;
            let ptr = NonNull::new(ptr).ok_or(CV_RETURN_ERROR)?;
            let status =
                unsafe { crate::ffi::CVDisplayLinkSetCurrentCGDisplay(ptr.as_ptr(), display_id) };
            if status != 0 {
                unsafe { crate::ffi::CVDisplayLinkRelease(ptr.as_ptr()) };
                return Err(status);
            }
            let status = unsafe {
                crate::ffi::CVDisplayLinkSetOutputCallback(
                    ptr.as_ptr(),
                    Some(immortal_display_link_output),
                    display_id as usize as *mut c_void,
                )
            };
            if status != 0 {
                unsafe { crate::ffi::CVDisplayLinkRelease(ptr.as_ptr()) };
                return Err(status);
            }
            let inner = Arc::new(ImmortalDisplayLinkInner {
                ptr: ImmortalCVDisplayLink(ptr),
                display_id,
                running: AtomicBool::new(false),
                delivering: AtomicBool::new(false),
                transition: Mutex::new(()),
                output: Mutex::new(None),
            });
            assert!(slot.link.set(Arc::clone(&inner)).is_ok());
            inner
        };
        drop(initialization);
        Ok(Self { inner })
    }

    #[must_use]
    pub fn display_id(&self) -> u32 {
        self.inner.display_id
    }

    pub fn set_output_handler<F>(&self, handler: F)
    where
        F: Fn(&CVTimeStamp, &CVTimeStamp) + Send + Sync + 'static,
    {
        *lock(&self.inner.output) = Some(Arc::new(handler));
    }

    pub fn clear_output_handler(&self) {
        *lock(&self.inner.output) = None;
    }

    pub fn start(&self) -> Result<(), CVReturn> {
        let _transition = lock(&self.inner.transition);
        if !self.is_running() {
            let status = unsafe { crate::ffi::CVDisplayLinkStart(self.inner.ptr.0.as_ptr()) };
            if status != 0 {
                self.inner.running.store(
                    unsafe { crate::ffi::CVDisplayLinkIsRunning(self.inner.ptr.0.as_ptr()) },
                    Ordering::Release,
                );
                return Err(status);
            }
            self.inner.running.store(true, Ordering::Release);
        }
        self.inner.delivering.store(true, Ordering::Release);
        Ok(())
    }

    pub fn stop(&self) -> Result<(), CVReturn> {
        let _transition = lock(&self.inner.transition);
        self.inner.delivering.store(false, Ordering::Release);
        if !self.is_running() {
            return Ok(());
        }
        let status = unsafe { crate::ffi::CVDisplayLinkStop(self.inner.ptr.0.as_ptr()) };
        if status != 0 {
            self.inner.running.store(
                unsafe { crate::ffi::CVDisplayLinkIsRunning(self.inner.ptr.0.as_ptr()) },
                Ordering::Release,
            );
            return Err(status);
        }
        self.inner.running.store(false, Ordering::Release);
        Ok(())
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        self.inner.running.load(Ordering::Acquire)
    }
}

unsafe extern "C" fn immortal_display_link_output(
    _display_link: apple_cf::raw::CVDisplayLinkRef,
    now: *const CVTimeStamp,
    output_time: *const CVTimeStamp,
    _flags_in: u64,
    _flags_out: *mut u64,
    context: *mut c_void,
) -> CVReturn {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let (Some(now), Some(output_time)) =
            (unsafe { now.as_ref() }, unsafe { output_time.as_ref() })
        else {
            return;
        };
        let Some(display_id) = u32::try_from(context as usize).ok() else {
            return;
        };
        let slot = lock(&IMMORTAL_LINKS).get(&display_id).cloned();
        let Some(slot) = slot else {
            return;
        };
        let Some(inner) = slot.link.get() else {
            return;
        };
        if !inner.delivering.load(Ordering::Acquire) {
            return;
        }
        let handler = lock(&inner.output).clone();
        if let Some(handler) = handler {
            if inner.delivering.load(Ordering::Acquire) {
                handler(now, output_time);
            }
        }
    }));
    0
}

#[must_use]
pub fn current_host_time() -> u64 {
    unsafe { apple_cf::raw::CVGetCurrentHostTime() }
}

#[must_use]
pub fn host_clock_frequency() -> f64 {
    unsafe { apple_cf::raw::CVGetHostClockFrequency() }
}

#[cfg(test)]
mod immortal_tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::mpsc;
    use std::time::Duration;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    unsafe extern "C" {
        fn CGMainDisplayID() -> u32;
    }

    #[test]
    fn start_stop_restart_and_deliver_from_core_video_thread() {
        fn assert_send_sync<T: Send + Sync>() {}

        let _guard = lock(&TEST_LOCK);
        assert_send_sync::<ImmortalDisplayLink>();

        let display_id = unsafe { CGMainDisplayID() };
        if display_id == 0 {
            return;
        }
        let link = ImmortalDisplayLink::for_display(display_id).expect("create display link");
        let same_link = ImmortalDisplayLink::for_display(display_id).expect("reuse display link");
        assert!(Arc::ptr_eq(&link.inner, &same_link.inner));
        assert_eq!(link.display_id(), display_id);
        assert!(!link.is_running());

        let (sender, receiver) = mpsc::channel();
        let test_thread = std::thread::current().id();
        link.set_output_handler(move |now, output| {
            let _ = sender.send((now.hostTime, output.hostTime, std::thread::current().id()));
        });
        for _ in 0..2 {
            link.start().expect("start display link");
            assert!(link.is_running());
            let (now, output, callback_thread) = receiver
                .recv_timeout(Duration::from_secs(5))
                .expect("receive display link callback");
            assert!(now > 0);
            assert!(output > 0);
            assert_ne!(callback_thread, test_thread);
            link.stop().expect("stop display link");
            assert!(!link.is_running());
        }
        link.clear_output_handler();
        assert!(current_host_time() > 0);
        assert!(host_clock_frequency().is_finite());
        assert!(host_clock_frequency() > 0.0);
    }

    #[test]
    fn callback_panic_does_not_cross_c_boundary() {
        let _guard = lock(&TEST_LOCK);
        let display_id = unsafe { CGMainDisplayID() };
        if display_id == 0 {
            return;
        }
        let link = ImmortalDisplayLink::for_display(display_id).expect("create display link");
        link.stop().expect("stop display link");
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        link.set_output_handler(move |_, _| {
            counter.fetch_add(1, Ordering::SeqCst);
            panic!("output callback panic");
        });
        let timestamp = unsafe { core::mem::zeroed::<CVTimeStamp>() };
        let late_status = unsafe {
            immortal_display_link_output(
                core::ptr::null_mut(),
                &raw const timestamp,
                &raw const timestamp,
                0,
                core::ptr::null_mut(),
                display_id as usize as *mut c_void,
            )
        };
        assert_eq!(late_status, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        link.inner.delivering.store(true, Ordering::Release);
        let status = unsafe {
            immortal_display_link_output(
                core::ptr::null_mut(),
                &raw const timestamp,
                &raw const timestamp,
                0,
                core::ptr::null_mut(),
                display_id as usize as *mut c_void,
            )
        };
        link.inner.delivering.store(false, Ordering::Release);
        link.clear_output_handler();
        assert_eq!(status, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
