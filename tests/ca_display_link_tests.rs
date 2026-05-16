use coreanimation::QuartzDisplayLink;

#[test]
fn cadisplaylink_is_headless_safe() {
    if let Some(link) = QuartzDisplayLink::new_main_screen() {
        assert!(!link.is_paused());
        link.set_paused(true);
        assert!(link.is_paused());
        let _ = link.timestamp();
        let _ = link.duration();
        let _ = link.target_timestamp();
        link.invalidate();
    }
}
