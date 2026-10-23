use core::cell::{Cell, RefCell};
use core::ffi::c_void;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::Duration;

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
const CALLBACK_EXIT_TIMEOUT: Duration = Duration::from_secs(5);
const START_RESULT_TIMEOUT: Duration = Duration::from_secs(10);

struct ImmortalCVDisplayLink(NonNull<c_void>);

unsafe impl Send for ImmortalCVDisplayLink {}
unsafe impl Sync for ImmortalCVDisplayLink {}

struct ImmortalDisplayLinkInner {
    ptr: ImmortalCVDisplayLink,
    display_id: u32,
    delivering: AtomicBool,
    transition: Mutex<LinkTransition>,
    output: Mutex<Option<Arc<ImmortalOutput>>>,
    callbacks: CallbackGate,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LinkPhase {
    Stopped,
    Starting,
    Running,
    Stopping,
}

struct LinkTransition {
    phase: LinkPhase,
    desired_running: bool,
    pending_starts: Vec<mpsc::Sender<Result<(), CVReturn>>>,
}

#[derive(Debug)]
#[must_use]
pub enum StartOutcome {
    Started,
    Pending(mpsc::Receiver<Result<(), CVReturn>>),
}

std::thread_local! {
    static IN_OUTPUT_CALLBACK: Cell<bool> = const { Cell::new(false) };
}

#[derive(Default)]
struct CallbackGate {
    active: Mutex<usize>,
    idle: Condvar,
}

struct OutputCallbackGuard<'a> {
    gate: &'a CallbackGate,
    previous: bool,
}

impl<'a> OutputCallbackGuard<'a> {
    fn enter(gate: &'a CallbackGate) -> Self {
        *lock(&gate.active) += 1;
        Self {
            gate,
            previous: IN_OUTPUT_CALLBACK.with(|active| active.replace(true)),
        }
    }
}

impl Drop for OutputCallbackGuard<'_> {
    fn drop(&mut self) {
        IN_OUTPUT_CALLBACK.with(|active| active.set(self.previous));
        *lock(&self.gate.active) -= 1;
        self.gate.idle.notify_all();
    }
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
                delivering: AtomicBool::new(false),
                transition: Mutex::new(LinkTransition {
                    phase: LinkPhase::Stopped,
                    desired_running: false,
                    pending_starts: Vec::new(),
                }),
                output: Mutex::new(None),
                callbacks: CallbackGate::default(),
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

    pub fn start(&self) -> Result<StartOutcome, CVReturn> {
        let in_callback = IN_OUTPUT_CALLBACK.with(Cell::get);
        let mut transition = lock(&self.inner.transition);
        transition.desired_running = true;
        if transition.phase == LinkPhase::Running {
            self.inner.delivering.store(true, Ordering::Release);
            return Ok(StartOutcome::Started);
        }

        let (sender, receiver) = mpsc::channel();
        transition.pending_starts.push(sender);
        let needs_start = transition.phase == LinkPhase::Stopped;
        if needs_start {
            transition.phase = LinkPhase::Starting;
        }
        drop(transition);

        if needs_start {
            if in_callback {
                self.spawn_start();
            } else {
                self.complete_start();
            }
        }
        if in_callback {
            Ok(StartOutcome::Pending(receiver))
        } else {
            receiver
                .recv_timeout(START_RESULT_TIMEOUT)
                .unwrap_or(Err(CV_RETURN_ERROR))
                .map(|()| StartOutcome::Started)
        }
    }

    pub fn stop(&self) -> Result<(), CVReturn> {
        let mut transition = lock(&self.inner.transition);
        transition.desired_running = false;
        self.inner.delivering.store(false, Ordering::Release);
        if transition.phase == LinkPhase::Running {
            transition.phase = LinkPhase::Stopping;
            drop(transition);
            self.complete_stop()
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        matches!(
            lock(&self.inner.transition).phase,
            LinkPhase::Running | LinkPhase::Stopping
        )
    }

    fn spawn_start(&self) {
        let link = self.clone();
        if std::thread::Builder::new()
            .name("coreanimation-display-link-start".to_owned())
            .spawn(move || link.complete_start())
            .is_err()
        {
            let mut transition = lock(&self.inner.transition);
            transition.phase = LinkPhase::Stopped;
            self.inner.delivering.store(false, Ordering::Release);
            let pending = core::mem::take(&mut transition.pending_starts);
            drop(transition);
            for sender in pending {
                let _ = sender.send(Err(CV_RETURN_ERROR));
            }
        }
    }

    fn complete_start(&self) {
        let (active, _) = self
            .inner
            .callbacks
            .idle
            .wait_timeout_while(
                lock(&self.inner.callbacks.active),
                CALLBACK_EXIT_TIMEOUT,
                |active| *active != 0,
            )
            .unwrap_or_else(PoisonError::into_inner);
        if *active != 0 {
            drop(active);
            let mut transition = lock(&self.inner.transition);
            transition.phase = LinkPhase::Stopped;
            let pending = core::mem::take(&mut transition.pending_starts);
            drop(transition);
            for sender in pending {
                let _ = sender.send(Err(CV_RETURN_ERROR));
            }
            return;
        }
        drop(active);

        let mut transition = lock(&self.inner.transition);
        if !transition.desired_running {
            transition.phase = LinkPhase::Stopped;
            let pending = core::mem::take(&mut transition.pending_starts);
            drop(transition);
            for sender in pending {
                let _ = sender.send(Err(CV_RETURN_ERROR));
            }
            return;
        }
        drop(transition);

        let status = unsafe { crate::ffi::CVDisplayLinkStart(self.inner.ptr.0.as_ptr()) };
        let native_running =
            status == 0 || unsafe { crate::ffi::CVDisplayLinkIsRunning(self.inner.ptr.0.as_ptr()) };
        let mut transition = lock(&self.inner.transition);
        let stop_after_start = if native_running {
            if transition.desired_running {
                transition.phase = LinkPhase::Running;
                self.inner.delivering.store(true, Ordering::Release);
                false
            } else {
                transition.phase = LinkPhase::Stopping;
                self.inner.delivering.store(false, Ordering::Release);
                true
            }
        } else {
            transition.phase = LinkPhase::Stopped;
            self.inner.delivering.store(false, Ordering::Release);
            false
        };
        let pending = core::mem::take(&mut transition.pending_starts);
        drop(transition);
        for sender in pending {
            let _ = sender.send(cv_result(status));
        }
        if stop_after_start {
            let _ = self.complete_stop();
        }
    }

    fn complete_stop(&self) -> Result<(), CVReturn> {
        let status = unsafe { crate::ffi::CVDisplayLinkStop(self.inner.ptr.0.as_ptr()) };
        let native_running =
            status != 0 && unsafe { crate::ffi::CVDisplayLinkIsRunning(self.inner.ptr.0.as_ptr()) };
        #[cfg(test)]
        immortal_tests::after_native_stop();

        let mut transition = lock(&self.inner.transition);
        if native_running {
            transition.phase = LinkPhase::Running;
            let wants_running = transition.desired_running;
            self.inner
                .delivering
                .store(wants_running, Ordering::Release);
            let pending = core::mem::take(&mut transition.pending_starts);
            drop(transition);
            for sender in pending {
                let _ = sender.send(if wants_running {
                    Ok(())
                } else {
                    Err(CV_RETURN_ERROR)
                });
            }
        } else if transition.desired_running {
            transition.phase = LinkPhase::Starting;
            drop(transition);
            self.spawn_start();
        } else {
            transition.phase = LinkPhase::Stopped;
            let pending = core::mem::take(&mut transition.pending_starts);
            drop(transition);
            for sender in pending {
                let _ = sender.send(Err(CV_RETURN_ERROR));
            }
        }
        cv_result(status)
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
                let _guard = OutputCallbackGuard::enter(&inner.callbacks);
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
    use std::time::{Duration, Instant};

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    std::thread_local! {
        static AFTER_NATIVE_STOP: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
    }

    pub(super) fn after_native_stop() {
        AFTER_NATIVE_STOP.with(|hook| {
            if let Some(hook) = hook.borrow().as_ref() {
                hook();
            }
        });
    }

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
            assert!(matches!(
                link.start().expect("start display link"),
                StartOutcome::Started
            ));
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

    fn callback_reenters_during_concurrent_stop(restart: bool) {
        let _guard = lock(&TEST_LOCK);
        let display_id = unsafe { CGMainDisplayID() };
        if display_id == 0 {
            return;
        }

        let link = ImmortalDisplayLink::for_display(display_id).expect("create display link");
        link.stop().expect("stop display link");
        let (entered_sender, entered_receiver) = mpsc::channel();
        let (callback_sender, callback_receiver) = mpsc::channel();
        let released = Arc::new(AtomicBool::new(false));
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_link = link.clone();
        let handler_released = Arc::clone(&released);
        let handler_calls = Arc::clone(&calls);
        link.set_output_handler(move |_, _| {
            if handler_calls.fetch_add(1, Ordering::SeqCst) != 0 {
                return;
            }
            let _ = entered_sender.send(());
            let deadline = Instant::now() + Duration::from_secs(5);
            while !handler_released.load(Ordering::Acquire) {
                if Instant::now() >= deadline {
                    return;
                }
                std::thread::sleep(Duration::from_millis(1));
            }
            let result = if restart {
                handler_link.start().map(Some)
            } else {
                handler_link.stop().map(|()| None)
            };
            let _ = callback_sender.send(result);
        });
        assert!(matches!(
            link.start().expect("start display link"),
            StartOutcome::Started
        ));
        entered_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("callback entered before stop");

        let (stop_sender, stop_receiver) = mpsc::channel();
        let stopping_link = link.clone();
        let stopper = std::thread::spawn(move || {
            let _ = stop_sender.send(stopping_link.stop());
        });

        let deadline = Instant::now() + Duration::from_secs(5);
        let mut stopping = false;
        while Instant::now() < deadline {
            if link
                .inner
                .transition
                .try_lock()
                .is_ok_and(|state| state.phase == LinkPhase::Stopping)
            {
                stopping = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        if stopping {
            std::thread::sleep(Duration::from_millis(50));
        }
        let stopped_before_callback_returned = stop_receiver.try_recv().is_ok();
        released.store(true, Ordering::Release);
        assert!(stopping, "stop never entered the Stopping state");
        assert!(
            !stopped_before_callback_returned,
            "stop returned before the callback finished"
        );

        let callback_outcome = callback_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("callback re-entry completed")
            .expect("callback re-entry succeeded");
        if restart {
            let StartOutcome::Pending(receiver) =
                callback_outcome.expect("callback requested a restart")
            else {
                panic!("callback restart must be pending");
            };
            assert_eq!(receiver.recv_timeout(Duration::from_secs(5)), Ok(Ok(())));
        } else {
            assert!(callback_outcome.is_none());
        }
        stop_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("concurrent stop completed")
            .expect("concurrent stop succeeded");
        stopper.join().expect("stop thread completed");
        assert_eq!(link.is_running(), restart);
        if restart {
            let deadline = Instant::now() + Duration::from_secs(5);
            while calls.load(Ordering::SeqCst) < 2 && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(1));
            }
            assert!(calls.load(Ordering::SeqCst) >= 2);
        }
        link.stop().expect("stop display link after test");
        link.clear_output_handler();
    }

    #[test]
    fn callback_stops_while_another_thread_stops() {
        callback_reenters_during_concurrent_stop(false);
    }

    #[test]
    fn callback_starts_while_another_thread_stops() {
        callback_reenters_during_concurrent_stop(true);
    }

    #[test]
    fn callback_stops_then_starts_in_one_frame() {
        let _guard = lock(&TEST_LOCK);
        let display_id = unsafe { CGMainDisplayID() };
        if display_id == 0 {
            return;
        }

        let link = ImmortalDisplayLink::for_display(display_id).expect("create display link");
        link.stop().expect("stop display link");
        let (sender, receiver) = mpsc::channel();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let callback_link = link.clone();
        link.set_output_handler(move |_, _| {
            if callback_calls.fetch_add(1, Ordering::SeqCst) == 0 {
                let stopped = callback_link.stop();
                let started = callback_link.start();
                let _ = sender.send((stopped, started));
            }
        });

        assert!(matches!(
            link.start().expect("start display link"),
            StartOutcome::Started
        ));
        let (stopped, started) = receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("callback stopped and requested a start");
        stopped.expect("callback stopped the link");
        let StartOutcome::Pending(pending) = started.expect("callback requested a start") else {
            panic!("callback start must be pending");
        };
        assert_eq!(pending.recv_timeout(Duration::from_secs(5)), Ok(Ok(())));

        let deadline = Instant::now() + Duration::from_secs(5);
        while calls.load(Ordering::SeqCst) < 2 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(calls.load(Ordering::SeqCst) >= 2);
        assert!(link.is_running());
        link.stop().expect("stop display link after test");
        link.clear_output_handler();
    }

    #[test]
    fn external_start_during_callback_stop_receives_native_result() {
        let _guard = lock(&TEST_LOCK);
        let display_id = unsafe { CGMainDisplayID() };
        if display_id == 0 {
            return;
        }

        let link = ImmortalDisplayLink::for_display(display_id).expect("create display link");
        link.stop().expect("stop display link");
        let (paused_sender, paused_receiver) = mpsc::channel();
        let (callback_sender, callback_receiver) = mpsc::channel();
        let released = Arc::new(AtomicBool::new(false));
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_released = Arc::clone(&released);
        let callback_calls = Arc::clone(&calls);
        let callback_link = link.clone();
        link.set_output_handler(move |_, _| {
            if callback_calls.fetch_add(1, Ordering::SeqCst) != 0 {
                return;
            }
            let paused_sender = paused_sender.clone();
            let released = Arc::clone(&callback_released);
            AFTER_NATIVE_STOP.with(|hook| {
                *hook.borrow_mut() = Some(Box::new(move || {
                    let _ = paused_sender.send(());
                    let deadline = Instant::now() + Duration::from_secs(5);
                    while !released.load(Ordering::Acquire) {
                        if Instant::now() >= deadline {
                            return;
                        }
                        std::thread::sleep(Duration::from_millis(1));
                    }
                }));
            });
            let stopped = callback_link.stop();
            AFTER_NATIVE_STOP.with(|hook| *hook.borrow_mut() = None);
            let _ = callback_sender.send(stopped);
        });

        assert!(matches!(
            link.start().expect("start display link"),
            StartOutcome::Started
        ));
        paused_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("callback stopped before the state update");

        let (start_sender, start_receiver) = mpsc::channel();
        let starting_link = link.clone();
        let worker = std::thread::spawn(move || {
            let _ = start_sender.send(starting_link.start());
        });
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut merged = false;
        while Instant::now() < deadline {
            if link.inner.transition.try_lock().is_ok_and(|transition| {
                transition.phase == LinkPhase::Stopping
                    && transition.desired_running
                    && transition.pending_starts.len() == 1
            }) {
                merged = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        released.store(true, Ordering::Release);
        assert!(merged, "external start did not merge with callback stop");

        callback_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("callback stop completed")
            .expect("callback stop succeeded");
        let result = start_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("merged start completed")
            .expect("merged native start succeeded");
        assert!(matches!(result, StartOutcome::Started));
        worker.join().expect("start thread completed");

        let deadline = Instant::now() + Duration::from_secs(5);
        while calls.load(Ordering::SeqCst) < 2 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(calls.load(Ordering::SeqCst) >= 2);
        assert!(link.is_running());
        link.stop().expect("stop display link after test");
        link.clear_output_handler();
    }
}
