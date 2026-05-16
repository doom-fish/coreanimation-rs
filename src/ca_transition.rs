use crate::animation::{AnimationLike, Transition};

impl Transition {
    #[must_use]
    pub fn has_subtype(&self) -> bool {
        unsafe { crate::ffi::ca_transition_has_subtype(self.as_animation_ptr()) }
    }

    pub fn clear_subtype(&self) {
        unsafe { crate::ffi::ca_transition_clear_subtype(self.as_animation_ptr()) };
    }
}
