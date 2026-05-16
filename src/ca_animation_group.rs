use crate::animation::{AnimationGroup, AnimationLike};

impl AnimationGroup {
    pub fn push<A: AnimationLike>(&self, animation: &A) {
        unsafe {
            crate::ffi::ca_animation_group_append_animation(
                self.as_animation_ptr(),
                animation.as_animation_ptr(),
            )
        };
    }

    pub fn clear(&self) {
        unsafe { crate::ffi::ca_animation_group_clear_animations(self.as_animation_ptr()) };
    }

    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { crate::ffi::ca_animation_group_animation_count(self.as_animation_ptr()) }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
