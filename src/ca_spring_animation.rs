use crate::animation::{AnimationLike, SpringAnimation};

impl SpringAnimation {
    pub fn configure(&self, mass: f64, stiffness: f64, damping: f64, initial_velocity: f64) {
        unsafe {
            crate::ffi::ca_spring_animation_configure(
                self.as_animation_ptr(),
                mass,
                stiffness,
                damping,
                initial_velocity,
            )
        };
    }
}
