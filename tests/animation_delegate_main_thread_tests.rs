use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use coreanimation::{
    AnimationDelegate, BasicAnimation, CGRect, Layer, MetalDisplayLink, Transaction,
};

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

fn animated_layer(delegate: &AnimationDelegate, duration: f64) -> Layer {
    let layer = Layer::new().expect("layer");
    layer
        .set_frame(CGRect::new(0.0, 0.0, 10.0, 10.0))
        .expect("frame");
    let animation = BasicAnimation::new(Some("opacity")).expect("animation");
    animation.set_from_number(0.0);
    animation.set_to_number(1.0);
    animation.set_duration(duration);
    animation.set_delegate(Some(delegate));
    Transaction::begin();
    layer.add_animation(&animation, Some("fade"));
    Transaction::commit();
    Transaction::flush();
    layer
}

fn callbacks_from_a_worker_run_on_the_main_thread() {
    let started = Arc::new(AtomicBool::new(false));
    let stopped = Arc::new(AtomicBool::new(false));
    let all_on_main = Arc::new(AtomicBool::new(true));
    let (committed_tx, committed_rx) = mpsc::channel();
    let (finish_tx, finish_rx) = mpsc::channel::<()>();
    let worker = {
        let (started, stopped, all_on_main) = (
            Arc::clone(&started),
            Arc::clone(&stopped),
            Arc::clone(&all_on_main),
        );
        std::thread::spawn(move || {
            let mut delegate = AnimationDelegate::new().expect("delegate");
            let start_main = Arc::clone(&all_on_main);
            delegate.set_did_start(move |_animation| {
                start_main.fetch_and(on_main_thread(), Ordering::SeqCst);
                started.store(true, Ordering::SeqCst);
            });
            delegate.set_did_stop(move |_animation, _finished| {
                all_on_main.fetch_and(on_main_thread(), Ordering::SeqCst);
                stopped.store(true, Ordering::SeqCst);
            });
            let layer = animated_layer(&delegate, 0.05);
            committed_tx.send(()).expect("committed");
            finish_rx.recv().expect("finish");
            drop(layer);
            drop(delegate);
        })
    };
    committed_rx.recv().expect("worker committed");

    assert!(spin_main_run_loop_until(|| {
        started.load(Ordering::SeqCst) && stopped.load(Ordering::SeqCst)
    }));
    assert!(all_on_main.load(Ordering::SeqCst));
    finish_tx.send(()).expect("finish worker");
    worker.join().expect("worker");
}

fn dropping_the_delegate_before_callbacks_arrive_is_safe() {
    let probe = Arc::new(AtomicBool::new(false));
    let (dropped_tx, dropped_rx) = mpsc::channel();
    let worker = {
        let probe = Arc::clone(&probe);
        std::thread::spawn(move || {
            let mut delegate = AnimationDelegate::new().expect("delegate");
            let start_probe = Arc::clone(&probe);
            delegate.set_did_start(move |_animation| start_probe.store(true, Ordering::SeqCst));
            delegate.set_did_stop(move |_animation, _finished| probe.store(true, Ordering::SeqCst));
            let layer = animated_layer(&delegate, 0.3);
            drop(delegate);
            drop(layer);
            dropped_tx.send(()).expect("dropped");
        })
    };
    dropped_rx.recv().expect("worker dropped the delegate");
    worker.join().expect("worker");

    assert!(spin_main_run_loop_until(|| Arc::strong_count(&probe) == 1));
    assert!(!probe.load(Ordering::SeqCst));
}

fn main() {
    assert!(on_main_thread());
    callbacks_from_a_worker_run_on_the_main_thread();
    dropping_the_delegate_before_callbacks_arrive_is_safe();
    println!("animation delegate: 2 checks passed");
}
