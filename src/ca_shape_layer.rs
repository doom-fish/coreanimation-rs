use crate::layer::{LayerLike, ShapeLayer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShapeFillRule {
    NonZero = 0,
    EvenOdd = 1,
}

impl ShapeFillRule {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::EvenOdd,
            _ => Self::NonZero,
        }
    }
}

impl ShapeLayer {
    #[must_use]
    pub fn fill_rule(&self) -> ShapeFillRule {
        ShapeFillRule::from_raw(unsafe {
            crate::ffi::ca_shape_layer_get_fill_rule(self.as_layer_ptr())
        })
    }

    pub fn set_fill_rule(&self, value: ShapeFillRule) {
        unsafe { crate::ffi::ca_shape_layer_set_fill_rule(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn stroke_start(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_stroke_start(self.as_layer_ptr()) }
    }

    pub fn set_stroke_start(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_stroke_start(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn stroke_end(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_stroke_end(self.as_layer_ptr()) }
    }

    pub fn set_stroke_end(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_stroke_end(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn line_dash_phase(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_line_dash_phase(self.as_layer_ptr()) }
    }

    pub fn set_line_dash_phase(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_line_dash_phase(self.as_layer_ptr(), value) };
    }
}
