use std::ffi::CStr;
use std::ops::Deref;

use apple_cf::cg::CGImage;
use apple_cf::cg::{CGPoint, CGSize};

use crate::color::Color;
use crate::layer::{Layer, LayerLike};
use crate::private::{cstring_from_str, handle_type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterShape {
    Point = 0,
    Line = 1,
    Rectangle = 2,
    Cuboid = 3,
    Circle = 4,
    Sphere = 5,
}

impl EmitterShape {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Line,
            2 => Self::Rectangle,
            3 => Self::Cuboid,
            4 => Self::Circle,
            5 => Self::Sphere,
            _ => Self::Point,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterMode {
    Points = 0,
    Outline = 1,
    Surface = 2,
    Volume = 3,
}

impl EmitterMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Outline,
            2 => Self::Surface,
            _ => Self::Volume,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterRenderMode {
    Unordered = 0,
    OldestFirst = 1,
    OldestLast = 2,
    BackToFront = 3,
    Additive = 4,
}

impl EmitterRenderMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::OldestFirst,
            2 => Self::OldestLast,
            3 => Self::BackToFront,
            4 => Self::Additive,
            _ => Self::Unordered,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmitterLayer {
    inner: Layer,
}

impl EmitterLayer {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Layer::from_raw(crate::ffi::ca_emitter_layer_new()) }.map(|inner| Self { inner })
    }

    pub fn set_emitter_cells(&self, cells: &[&EmitterCell]) {
        let raw: Vec<*mut core::ffi::c_void> = cells.iter().map(|cell| cell.as_ptr()).collect();
        unsafe {
            crate::ffi::ca_emitter_layer_set_emitter_cells(
                self.as_layer_ptr(),
                raw.as_ptr(),
                raw.len(),
            )
        };
    }

    #[must_use]
    pub fn emitter_cells(&self) -> Vec<EmitterCell> {
        let count = unsafe { crate::ffi::ca_emitter_layer_emitter_cell_count(self.as_layer_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                EmitterCell::from_raw(crate::ffi::ca_emitter_layer_emitter_cell_at(
                    self.as_layer_ptr(),
                    index,
                ))
            })
            .collect()
    }

    #[must_use]
    pub fn birth_rate(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_layer_get_birth_rate(self.as_layer_ptr()) }
    }

    pub fn set_birth_rate(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_layer_set_birth_rate(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn lifetime(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_layer_get_lifetime(self.as_layer_ptr()) }
    }

    pub fn set_lifetime(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_layer_set_lifetime(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn emitter_position(&self) -> CGPoint {
        let mut point = CGPoint::zero();
        let ok = unsafe {
            crate::ffi::ca_emitter_layer_get_emitter_position(
                self.as_layer_ptr(),
                (&mut point as *mut CGPoint).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            point
        } else {
            CGPoint::zero()
        }
    }

    pub fn set_emitter_position(&self, value: CGPoint) {
        unsafe {
            crate::ffi::ca_emitter_layer_set_emitter_position(self.as_layer_ptr(), value.x, value.y)
        };
    }

    #[must_use]
    pub fn emitter_size(&self) -> CGSize {
        let mut size = CGSize::zero();
        let ok = unsafe {
            crate::ffi::ca_emitter_layer_get_emitter_size(
                self.as_layer_ptr(),
                (&mut size as *mut CGSize).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            size
        } else {
            CGSize::zero()
        }
    }

    pub fn set_emitter_size(&self, value: CGSize) {
        unsafe {
            crate::ffi::ca_emitter_layer_set_emitter_size(
                self.as_layer_ptr(),
                value.width,
                value.height,
            )
        };
    }

    #[must_use]
    pub fn emitter_shape(&self) -> EmitterShape {
        EmitterShape::from_raw(unsafe {
            crate::ffi::ca_emitter_layer_get_emitter_shape(self.as_layer_ptr())
        })
    }

    pub fn set_emitter_shape(&self, value: EmitterShape) {
        unsafe {
            crate::ffi::ca_emitter_layer_set_emitter_shape(self.as_layer_ptr(), value as i32)
        };
    }

    #[must_use]
    pub fn emitter_mode(&self) -> EmitterMode {
        EmitterMode::from_raw(unsafe {
            crate::ffi::ca_emitter_layer_get_emitter_mode(self.as_layer_ptr())
        })
    }

    pub fn set_emitter_mode(&self, value: EmitterMode) {
        unsafe { crate::ffi::ca_emitter_layer_set_emitter_mode(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn render_mode(&self) -> EmitterRenderMode {
        EmitterRenderMode::from_raw(unsafe {
            crate::ffi::ca_emitter_layer_get_render_mode(self.as_layer_ptr())
        })
    }

    pub fn set_render_mode(&self, value: EmitterRenderMode) {
        unsafe { crate::ffi::ca_emitter_layer_set_render_mode(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn velocity(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_layer_get_velocity(self.as_layer_ptr()) }
    }

    pub fn set_velocity(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_layer_set_velocity(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn scale(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_layer_get_scale(self.as_layer_ptr()) }
    }

    pub fn set_scale(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_layer_set_scale(self.as_layer_ptr(), value) };
    }
}

impl Deref for EmitterLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LayerLike for EmitterLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}

handle_type!(EmitterCell);

impl EmitterCell {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_emitter_cell_new()) }
    }

    pub fn set_name(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_emitter_cell_set_name(self.as_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_emitter_cell_get_name(self.as_ptr()) })
    }

    #[must_use]
    pub fn enabled(&self) -> bool {
        unsafe { crate::ffi::ca_emitter_cell_get_enabled(self.as_ptr()) }
    }

    pub fn set_enabled(&self, value: bool) {
        unsafe { crate::ffi::ca_emitter_cell_set_enabled(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn birth_rate(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_cell_get_birth_rate(self.as_ptr()) }
    }

    pub fn set_birth_rate(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_cell_set_birth_rate(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn lifetime(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_cell_get_lifetime(self.as_ptr()) }
    }

    pub fn set_lifetime(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_cell_set_lifetime(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn velocity(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_cell_get_velocity(self.as_ptr()) }
    }

    pub fn set_velocity(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_cell_set_velocity(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn scale(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_cell_get_scale(self.as_ptr()) }
    }

    pub fn set_scale(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_cell_set_scale(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn emission_range(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_cell_get_emission_range(self.as_ptr()) }
    }

    pub fn set_emission_range(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_cell_set_emission_range(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn emission_longitude(&self) -> f64 {
        unsafe { crate::ffi::ca_emitter_cell_get_emission_longitude(self.as_ptr()) }
    }

    pub fn set_emission_longitude(&self, value: f64) {
        unsafe { crate::ffi::ca_emitter_cell_set_emission_longitude(self.as_ptr(), value) };
    }

    pub fn set_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_emitter_cell_set_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_emitter_cell_get_color(self.as_ptr())) }
    }

    pub fn set_contents(&self, image: Option<&CGImage>) {
        unsafe {
            crate::ffi::ca_emitter_cell_set_contents(
                self.as_ptr(),
                image.map_or(core::ptr::null_mut(), CGImage::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn contents(&self) -> Option<CGImage> {
        let ptr = unsafe { crate::ffi::ca_emitter_cell_get_contents(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CGImage::from_raw(ptr) })
        }
    }

    #[must_use]
    pub fn alpha_speed(&self) -> f32 {
        unsafe { crate::ffi::ca_emitter_cell_get_alpha_speed(self.as_ptr()) }
    }

    pub fn set_alpha_speed(&self, value: f32) {
        unsafe { crate::ffi::ca_emitter_cell_set_alpha_speed(self.as_ptr(), value) };
    }
}

fn take_c_string(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { libc::free(ptr.cast()) };
    Some(value)
}
