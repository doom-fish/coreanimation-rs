use coreanimation::QuartzDisplayLink;

fn main() {
    if let Some(link) = QuartzDisplayLink::new_main_screen() {
        assert!(!link.is_paused());
        link.set_paused(true);
        assert!(link.is_paused());
        link.add_to_main_run_loop();
        link.remove_from_main_run_loop();
        let _ = link.timestamp();
        let _ = link.duration();
        let _ = link.target_timestamp();
        link.invalidate();
        println!("✅ CADisplayLink smoke OK");
    } else {
        println!("ℹ️ CADisplayLink unavailable on this runner");
    }
}
