use std::ops::Deref;

use crate::color::Color;
use crate::layer::{Layer, LayerLike};
use crate::transform::Transform3D;

#[derive(Debug, Clone)]
/// Safe wrapper around `CAReplicatorLayer`. See <https://developer.apple.com/documentation/quartzcore/careplicatorlayer>.
pub struct ReplicatorLayer {
    inner: Layer,
}

impl ReplicatorLayer {
    #[must_use]
    /// Creates a new replicator layer wrapper.
    pub fn new() -> Option<Self> {
        unsafe { Layer::from_raw(crate::ffi::ca_replicator_layer_new()) }
            .map(|inner| Self { inner })
    }

    #[must_use]
    /// Returns the replicator layer's instance count.
    pub fn instance_count(&self) -> usize {
        usize::try_from(unsafe {
            crate::ffi::ca_replicator_layer_get_instance_count(self.as_layer_ptr())
        })
        .unwrap_or_default()
    }

    /// Sets the replicator layer's instance count.
    pub fn set_instance_count(&self, value: usize) {
        let value = isize::try_from(value).unwrap_or(isize::MAX);
        unsafe { crate::ffi::ca_replicator_layer_set_instance_count(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the replicator layer preserves depth.
    pub fn preserves_depth(&self) -> bool {
        unsafe { crate::ffi::ca_replicator_layer_get_preserves_depth(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's preserves depth.
    pub fn set_preserves_depth(&self, value: bool) {
        unsafe { crate::ffi::ca_replicator_layer_set_preserves_depth(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the replicator layer's instance delay.
    pub fn instance_delay(&self) -> f64 {
        unsafe { crate::ffi::ca_replicator_layer_get_instance_delay(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's instance delay.
    pub fn set_instance_delay(&self, value: f64) {
        unsafe { crate::ffi::ca_replicator_layer_set_instance_delay(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the replicator layer's instance transform.
    pub fn instance_transform(&self) -> Transform3D {
        let mut transform = Transform3D::identity();
        let ok = unsafe {
            crate::ffi::ca_replicator_layer_get_instance_transform(
                self.as_layer_ptr(),
                (&mut transform as *mut Transform3D).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            transform
        } else {
            Transform3D::identity()
        }
    }

    /// Sets the replicator layer's instance transform.
    pub fn set_instance_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_transform(
                self.as_layer_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    /// Sets the replicator layer's instance color.
    pub fn set_instance_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the replicator layer's instance color.
    pub fn instance_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_replicator_layer_get_instance_color(
                self.as_layer_ptr(),
            ))
        }
    }

    #[must_use]
    /// Returns the replicator layer's instance red offset.
    pub fn instance_red_offset(&self) -> f32 {
        unsafe { crate::ffi::ca_replicator_layer_get_instance_red_offset(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's instance red offset.
    pub fn set_instance_red_offset(&self, value: f32) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_red_offset(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    /// Returns the replicator layer's instance green offset.
    pub fn instance_green_offset(&self) -> f32 {
        unsafe { crate::ffi::ca_replicator_layer_get_instance_green_offset(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's instance green offset.
    pub fn set_instance_green_offset(&self, value: f32) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_green_offset(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    /// Returns the replicator layer's instance blue offset.
    pub fn instance_blue_offset(&self) -> f32 {
        unsafe { crate::ffi::ca_replicator_layer_get_instance_blue_offset(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's instance blue offset.
    pub fn set_instance_blue_offset(&self, value: f32) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_blue_offset(self.as_layer_ptr(), value)
        };
    }

    #[must_use]
    /// Returns the replicator layer's instance alpha offset.
    pub fn instance_alpha_offset(&self) -> f32 {
        unsafe { crate::ffi::ca_replicator_layer_get_instance_alpha_offset(self.as_layer_ptr()) }
    }

    /// Sets the replicator layer's instance alpha offset.
    pub fn set_instance_alpha_offset(&self, value: f32) {
        unsafe {
            crate::ffi::ca_replicator_layer_set_instance_alpha_offset(self.as_layer_ptr(), value)
        };
    }
}

impl Deref for ReplicatorLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LayerLike for ReplicatorLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}
