use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use apple_cf::raw::CVDisplayLinkRef;
use coreanimation::{CVReturn, CVTimeStamp, DisplayLink};

fn link() -> Option<DisplayLink> {
    match DisplayLink::with_active_displays() {
        Ok(link) => Some(link),
        Err(status) => {
            eprintln!("skipping: no active display ({status})");
            None
        }
    }
}

fn wait_until(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    condition()
}

fn counting_handler(
    counter: &Arc<AtomicUsize>,
) -> impl FnMut(&CVTimeStamp, &CVTimeStamp) + Send + 'static {
    let counter = Arc::clone(counter);
    move |_, _| {
        counter.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn output_handler_receives_frames_and_is_freed_on_drop() {
    let Some(link) = link() else {
        return;
    };
    let frames = Arc::new(AtomicUsize::new(0));
    let saw_future_output = Arc::new(AtomicBool::new(false));
    let counter = Arc::clone(&frames);
    let future = Arc::clone(&saw_future_output);
    link.set_output_handler(move |now, output_time| {
        if output_time.hostTime > now.hostTime {
            future.store(true, Ordering::SeqCst);
        }
        counter.fetch_add(1, Ordering::SeqCst);
    })
    .expect("install handler");
    link.start().expect("start");

    assert!(wait_until(|| frames.load(Ordering::SeqCst) >= 3));
    drop(link);

    assert_eq!(Arc::strong_count(&frames), 1);
    assert!(saw_future_output.load(Ordering::SeqCst));
    let after_drop = frames.load(Ordering::SeqCst);
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(frames.load(Ordering::SeqCst), after_drop);
}

#[test]
fn replacing_a_running_handler_frees_the_old_one() {
    let Some(link) = link() else {
        return;
    };
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    link.set_output_handler(counting_handler(&first))
        .expect("first handler");
    link.start().expect("start");
    assert!(wait_until(|| first.load(Ordering::SeqCst) >= 1));

    link.set_output_handler(counting_handler(&second))
        .expect("second handler");

    assert_eq!(Arc::strong_count(&first), 1);
    assert!(link.is_running());
    let first_total = first.load(Ordering::SeqCst);
    assert!(wait_until(|| second.load(Ordering::SeqCst) >= 2));
    assert_eq!(first.load(Ordering::SeqCst), first_total);

    link.clear_output_handler().expect("clear handler");
    assert_eq!(Arc::strong_count(&second), 1);
    assert!(!link.is_running());
}

#[test]
fn panicking_handler_keeps_the_link_running() {
    let Some(link) = link() else {
        return;
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    link.set_output_handler(move |_, _| {
        assert_ne!(
            counter.fetch_add(1, Ordering::SeqCst),
            0,
            "output handler panic"
        );
    })
    .expect("install handler");
    link.start().expect("start");

    assert!(wait_until(|| calls.load(Ordering::SeqCst) >= 3));
    assert!(link.is_running());
}

#[test]
fn dropping_the_link_waits_for_the_running_handler() {
    let Some(link) = link() else {
        return;
    };
    let entered = Arc::new(AtomicBool::new(false));
    let finished = Arc::new(AtomicBool::new(false));
    let (entry, exit) = (Arc::clone(&entered), Arc::clone(&finished));
    link.set_output_handler(move |_, _| {
        if !entry.swap(true, Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(200));
            exit.store(true, Ordering::SeqCst);
        }
    })
    .expect("install handler");
    link.start().expect("start");
    assert!(wait_until(|| entered.load(Ordering::SeqCst)));

    drop(link);

    assert!(finished.load(Ordering::SeqCst));
    assert_eq!(Arc::strong_count(&finished), 1);
}

#[test]
fn only_the_last_handle_stops_the_link() {
    let Some(link) = link() else {
        return;
    };
    let frames = Arc::new(AtomicUsize::new(0));
    link.set_output_handler(counting_handler(&frames))
        .expect("install handler");
    link.start().expect("start");
    let clone = link.clone();

    drop(link);

    assert!(clone.is_running());
    let before = frames.load(Ordering::SeqCst);
    assert!(wait_until(|| frames.load(Ordering::SeqCst) > before + 1));
    drop(clone);
    assert_eq!(Arc::strong_count(&frames), 1);
}

static RAW_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn raw_output(
    _display_link: CVDisplayLinkRef,
    _now: *const CVTimeStamp,
    _output_time: *const CVTimeStamp,
    _flags_in: u64,
    _flags_out: *mut u64,
    _context: *mut core::ffi::c_void,
) -> CVReturn {
    RAW_CALLS.fetch_add(1, Ordering::SeqCst);
    0
}

#[test]
fn raw_callback_replaces_the_handler() {
    let Some(link) = link() else {
        return;
    };
    let frames = Arc::new(AtomicUsize::new(0));
    link.set_output_handler(counting_handler(&frames))
        .expect("install handler");

    unsafe { link.set_output_callback(Some(raw_output), core::ptr::null_mut()) }
        .expect("install raw callback");

    assert_eq!(Arc::strong_count(&frames), 1);
    link.start().expect("start");
    assert!(wait_until(|| RAW_CALLS.load(Ordering::SeqCst) >= 1));
    assert_eq!(frames.load(Ordering::SeqCst), 0);
}
