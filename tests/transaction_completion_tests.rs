use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use coreanimation::{MetalDisplayLink, Transaction};

fn on_main_thread() -> bool {
    unsafe { libc::pthread_main_np() != 0 }
}

fn spin_main_run_loop_until(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        MetalDisplayLink::run_current_run_loop_for(0.01);
    }
    condition()
}

fn completion_runs_once_on_the_main_thread_and_is_released() {
    let calls = Arc::new(AtomicUsize::new(0));
    let ran_on_main = Arc::new(AtomicBool::new(false));
    let (counter, main_flag) = (Arc::clone(&calls), Arc::clone(&ran_on_main));

    Transaction::begin();
    Transaction::set_completion_handler(move || {
        main_flag.store(on_main_thread(), Ordering::SeqCst);
        counter.fetch_add(1, Ordering::SeqCst);
    });
    Transaction::commit();

    assert!(spin_main_run_loop_until(
        || calls.load(Ordering::SeqCst) == 1
    ));
    assert!(ran_on_main.load(Ordering::SeqCst));
    assert!(spin_main_run_loop_until(|| Arc::strong_count(&calls) == 1));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

fn replaced_completion_is_released() {
    let first = Arc::new(AtomicUsize::new(0));
    let second = Arc::new(AtomicUsize::new(0));
    let (first_counter, second_counter) = (Arc::clone(&first), Arc::clone(&second));

    Transaction::begin();
    Transaction::set_completion_handler(move || {
        first_counter.fetch_add(1, Ordering::SeqCst);
    });
    Transaction::set_completion_handler(move || {
        second_counter.fetch_add(1, Ordering::SeqCst);
    });
    Transaction::commit();

    assert!(spin_main_run_loop_until(
        || second.load(Ordering::SeqCst) == 1
    ));
    assert!(spin_main_run_loop_until(|| {
        Arc::strong_count(&first) == 1 && Arc::strong_count(&second) == 1
    }));
    assert!(first.load(Ordering::SeqCst) <= 1);
}

fn panicking_completion_is_contained() {
    let after = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&after);

    Transaction::begin();
    Transaction::set_completion_handler(|| panic!("completion panic"));
    Transaction::commit();
    Transaction::begin();
    Transaction::set_completion_handler(move || flag.store(true, Ordering::SeqCst));
    Transaction::commit();

    assert!(spin_main_run_loop_until(|| after.load(Ordering::SeqCst)));
}

fn main() {
    assert!(on_main_thread());
    completion_runs_once_on_the_main_thread_and_is_released();
    replaced_completion_is_released();
    panicking_completion_is_contained();
    println!("transaction completion handlers: 3 checks passed");
}
