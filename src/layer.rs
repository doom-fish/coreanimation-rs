use std::ffi::CStr;
use std::ops::Deref;

use apple_cf::cg::CGImage;
use apple_cf::cg::{CGPoint, CGRect, CGSize};

use crate::animation::AnimationLike;
use crate::color::Color;
use crate::path::Path;
use crate::private::{cstring_from_str, handle_type};
use crate::transform::Transform3D;

handle_type!(Layer);

pub trait LayerLike {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void;
}

impl LayerLike for Layer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.as_ptr()
    }
}

macro_rules! layer_wrapper {
    ($name:ident, $ctor:path) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            inner: Layer,
        }

        impl $name {
            #[must_use]
            pub fn new() -> Option<Self> {
                unsafe { Layer::from_raw($ctor()) }.map(|inner| Self { inner })
            }
        }

        impl Deref for $name {
            type Target = Layer;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl LayerLike for $name {
            fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
                self.inner.as_ptr()
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ContentsGravity {
    Center = 0,
    Top = 1,
    Bottom = 2,
    Left = 3,
    Right = 4,
    TopLeft = 5,
    TopRight = 6,
    BottomLeft = 7,
    BottomRight = 8,
    Resize = 9,
    ResizeAspect = 10,
    ResizeAspectFill = 11,
}

impl ContentsGravity {
    const fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Center,
            1 => Self::Top,
            2 => Self::Bottom,
            3 => Self::Left,
            4 => Self::Right,
            5 => Self::TopLeft,
            6 => Self::TopRight,
            7 => Self::BottomLeft,
            8 => Self::BottomRight,
            10 => Self::ResizeAspect,
            11 => Self::ResizeAspectFill,
            _ => Self::Resize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ToneMapMode {
    Automatic = 0,
    Never = 1,
    IfSupported = 2,
}

impl ToneMapMode {
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Never,
            2 => Self::IfSupported,
            _ => Self::Automatic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LineCap {
    Butt = 0,
    Round = 1,
    Square = 2,
}

impl LineCap {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Round,
            2 => Self::Square,
            _ => Self::Butt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LineJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}

impl LineJoin {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Round,
            2 => Self::Bevel,
            _ => Self::Miter,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextAlignmentMode {
    Natural = 0,
    Left = 1,
    Right = 2,
    Center = 3,
    Justified = 4,
}

impl TextAlignmentMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Left,
            2 => Self::Right,
            3 => Self::Center,
            4 => Self::Justified,
            _ => Self::Natural,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextTruncationMode {
    None = 0,
    Start = 1,
    Middle = 2,
    End = 3,
}

impl TextTruncationMode {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Start,
            2 => Self::Middle,
            3 => Self::End,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GradientType {
    Axial = 0,
    Radial = 1,
    Conic = 2,
}

impl GradientType {
    const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Radial,
            2 => Self::Conic,
            _ => Self::Axial,
        }
    }
}

layer_wrapper!(ShapeLayer, crate::ffi::ca_shape_layer_new);
layer_wrapper!(TextLayer, crate::ffi::ca_text_layer_new);
layer_wrapper!(GradientLayer, crate::ffi::ca_gradient_layer_new);
layer_wrapper!(MetalLayer, crate::ffi::ca_metal_layer_new);

#[derive(Debug, Clone)]
pub struct MetalDrawable {
    ptr: *mut core::ffi::c_void,
    owned: bool,
}

impl Drop for MetalDrawable {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe { crate::ffi::ca_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl MetalDrawable {
    pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, owned: true })
        }
    }

    #[must_use]
    pub fn texture(&self) -> Option<apple_metal::MetalTexture> {
        let ptr = unsafe { crate::ffi::ca_metal_drawable_get_texture(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { apple_metal::MetalTexture::from_raw(ptr) })
        }
    }

    pub fn present(&self) {
        unsafe { crate::ffi::ca_metal_drawable_present(self.ptr) };
    }
}

impl Layer {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_layer_new()) }
    }

    #[must_use]
    pub fn frame(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_layer_get_frame(
                self.as_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }

    pub fn set_frame(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_layer_set_frame(self.as_ptr(), rect.x, rect.y, rect.width, rect.height)
        };
    }

    #[must_use]
    pub fn bounds(&self) -> CGRect {
        let mut rect = CGRect::zero();
        let ok = unsafe {
            crate::ffi::ca_layer_get_bounds(
                self.as_ptr(),
                (&mut rect as *mut CGRect).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            rect
        } else {
            CGRect::zero()
        }
    }

    pub fn set_bounds(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_layer_set_bounds(self.as_ptr(), rect.x, rect.y, rect.width, rect.height)
        };
    }

    #[must_use]
    pub fn position(&self) -> CGPoint {
        let mut point = CGPoint::zero();
        let ok = unsafe {
            crate::ffi::ca_layer_get_position(
                self.as_ptr(),
                (&mut point as *mut CGPoint).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            point
        } else {
            CGPoint::zero()
        }
    }

    pub fn set_position(&self, point: CGPoint) {
        unsafe { crate::ffi::ca_layer_set_position(self.as_ptr(), point.x, point.y) };
    }

    #[must_use]
    pub fn anchor_point(&self) -> CGPoint {
        let mut point = CGPoint::zero();
        let ok = unsafe {
            crate::ffi::ca_layer_get_anchor_point(
                self.as_ptr(),
                (&mut point as *mut CGPoint).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            point
        } else {
            CGPoint::zero()
        }
    }

    pub fn set_anchor_point(&self, point: CGPoint) {
        unsafe { crate::ffi::ca_layer_set_anchor_point(self.as_ptr(), point.x, point.y) };
    }

    #[must_use]
    pub fn transform(&self) -> Transform3D {
        let mut transform = Transform3D::identity();
        let ok = unsafe {
            crate::ffi::ca_layer_get_transform(
                self.as_ptr(),
                (&mut transform as *mut Transform3D).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            transform
        } else {
            Transform3D::identity()
        }
    }

    pub fn set_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_layer_set_transform(
                self.as_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    #[must_use]
    pub fn sublayers(&self) -> Vec<Layer> {
        let count = unsafe { crate::ffi::ca_layer_sublayer_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Layer::from_raw(crate::ffi::ca_layer_sublayer_at(self.as_ptr(), index))
            })
            .collect()
    }

    pub fn add_sublayer<L: LayerLike>(&self, child: &L) {
        unsafe { crate::ffi::ca_layer_add_sublayer(self.as_ptr(), child.as_layer_ptr()) };
    }

    pub fn remove_from_superlayer(&self) {
        unsafe { crate::ffi::ca_layer_remove_from_superlayer(self.as_ptr()) };
    }

    pub fn set_contents(&self, image: Option<&CGImage>) {
        let ptr = image.map_or(core::ptr::null_mut(), |image| image.as_ptr());
        unsafe { crate::ffi::ca_layer_set_contents(self.as_ptr(), ptr) };
    }

    #[must_use]
    pub fn contents(&self) -> Option<CGImage> {
        let ptr = unsafe { crate::ffi::ca_layer_get_contents(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CGImage::from_raw(ptr) })
        }
    }

    #[must_use]
    pub fn contents_scale(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_contents_scale(self.as_ptr()) }
    }

    pub fn set_contents_scale(&self, scale: f64) {
        unsafe { crate::ffi::ca_layer_set_contents_scale(self.as_ptr(), scale) };
    }

    pub fn set_background_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_background_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn background_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_background_color(self.as_ptr())) }
    }

    pub fn set_border_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_border_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn border_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_border_color(self.as_ptr())) }
    }

    #[must_use]
    pub fn border_width(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_border_width(self.as_ptr()) }
    }

    pub fn set_border_width(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_border_width(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn corner_radius(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_corner_radius(self.as_ptr()) }
    }

    pub fn set_corner_radius(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_corner_radius(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn opacity(&self) -> f32 {
        unsafe { crate::ffi::ca_layer_get_opacity(self.as_ptr()) }
    }

    pub fn set_opacity(&self, value: f32) {
        unsafe { crate::ffi::ca_layer_set_opacity(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn is_hidden(&self) -> bool {
        unsafe { crate::ffi::ca_layer_is_hidden(self.as_ptr()) }
    }

    pub fn set_hidden(&self, hidden: bool) {
        unsafe { crate::ffi::ca_layer_set_hidden(self.as_ptr(), hidden) };
    }

    pub fn set_mask<L: LayerLike>(&self, mask: Option<&L>) {
        unsafe {
            crate::ffi::ca_layer_set_mask(
                self.as_ptr(),
                mask.map_or(core::ptr::null_mut(), LayerLike::as_layer_ptr),
            )
        };
    }

    #[must_use]
    pub fn mask(&self) -> Option<Layer> {
        unsafe { Layer::from_raw(crate::ffi::ca_layer_get_mask(self.as_ptr())) }
    }

    #[must_use]
    pub fn masks_to_bounds(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_masks_to_bounds(self.as_ptr()) }
    }

    pub fn set_masks_to_bounds(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_masks_to_bounds(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn shadow_offset(&self) -> CGSize {
        let mut size = CGSize::zero();
        let ok = unsafe {
            crate::ffi::ca_layer_get_shadow_offset(
                self.as_ptr(),
                (&mut size as *mut CGSize).cast::<core::ffi::c_void>(),
            )
        };
        if ok {
            size
        } else {
            CGSize::zero()
        }
    }

    pub fn set_shadow_offset(&self, value: CGSize) {
        unsafe { crate::ffi::ca_layer_set_shadow_offset(self.as_ptr(), value.width, value.height) };
    }

    #[must_use]
    pub fn shadow_radius(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_shadow_radius(self.as_ptr()) }
    }

    pub fn set_shadow_radius(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_shadow_radius(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn shadow_opacity(&self) -> f32 {
        unsafe { crate::ffi::ca_layer_get_shadow_opacity(self.as_ptr()) }
    }

    pub fn set_shadow_opacity(&self, value: f32) {
        unsafe { crate::ffi::ca_layer_set_shadow_opacity(self.as_ptr(), value) };
    }

    pub fn set_shadow_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_shadow_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn shadow_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_shadow_color(self.as_ptr())) }
    }

    #[must_use]
    pub fn contents_gravity(&self) -> ContentsGravity {
        ContentsGravity::from_raw(unsafe {
            crate::ffi::ca_layer_get_contents_gravity(self.as_ptr())
        })
    }

    pub fn set_contents_gravity(&self, gravity: ContentsGravity) {
        unsafe { crate::ffi::ca_layer_set_contents_gravity(self.as_ptr(), gravity as i32) };
    }

    pub fn add_animation<A: AnimationLike>(&self, animation: &A, key: Option<&str>) {
        let key = key.and_then(cstring_from_str);
        unsafe {
            crate::ffi::ca_layer_add_animation(
                self.as_ptr(),
                animation.as_animation_ptr(),
                key.as_ref().map_or(core::ptr::null(), |key| key.as_ptr()),
            )
        };
    }

    pub fn remove_animation(&self, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { crate::ffi::ca_layer_remove_animation(self.as_ptr(), key.as_ptr()) };
        }
    }
}

impl ShapeLayer {
    pub fn set_path(&self, path: Option<&Path>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_path(
                self.as_layer_ptr(),
                path.map_or(core::ptr::null_mut(), Path::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn path(&self) -> Option<Path> {
        unsafe { Path::from_raw(crate::ffi::ca_shape_layer_get_path(self.as_layer_ptr())) }
    }

    pub fn set_fill_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_fill_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn fill_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_shape_layer_get_fill_color(
                self.as_layer_ptr(),
            ))
        }
    }

    pub fn set_stroke_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_stroke_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn stroke_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_shape_layer_get_stroke_color(
                self.as_layer_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn line_width(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_line_width(self.as_layer_ptr()) }
    }

    pub fn set_line_width(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_line_width(self.as_layer_ptr(), value) };
    }

    #[must_use]
    pub fn line_cap(&self) -> LineCap {
        LineCap::from_raw(unsafe { crate::ffi::ca_shape_layer_get_line_cap(self.as_layer_ptr()) })
    }

    pub fn set_line_cap(&self, value: LineCap) {
        unsafe { crate::ffi::ca_shape_layer_set_line_cap(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn line_join(&self) -> LineJoin {
        LineJoin::from_raw(unsafe { crate::ffi::ca_shape_layer_get_line_join(self.as_layer_ptr()) })
    }

    pub fn set_line_join(&self, value: LineJoin) {
        unsafe { crate::ffi::ca_shape_layer_set_line_join(self.as_layer_ptr(), value as i32) };
    }

    pub fn set_line_dash_pattern(&self, pattern: &[f64]) {
        unsafe {
            crate::ffi::ca_shape_layer_set_line_dash_pattern(
                self.as_layer_ptr(),
                pattern.as_ptr(),
                pattern.len(),
            )
        };
    }

    #[must_use]
    pub fn line_dash_pattern(&self) -> Vec<f64> {
        let count =
            unsafe { crate::ffi::ca_shape_layer_line_dash_pattern_count(self.as_layer_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_shape_layer_line_dash_pattern_at(self.as_layer_ptr(), index)
            })
            .collect()
    }

    #[must_use]
    pub fn miter_limit(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_miter_limit(self.as_layer_ptr()) }
    }

    pub fn set_miter_limit(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_miter_limit(self.as_layer_ptr(), value) };
    }
}

impl TextLayer {
    pub fn set_string(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_text_layer_set_string(self.as_layer_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    pub fn string(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_text_layer_get_string(self.as_layer_ptr()) })
    }

    pub fn set_font_name(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_text_layer_set_font_name(self.as_layer_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    pub fn font_name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_text_layer_get_font_name(self.as_layer_ptr()) })
    }

    #[must_use]
    pub fn font_size(&self) -> f64 {
        unsafe { crate::ffi::ca_text_layer_get_font_size(self.as_layer_ptr()) }
    }

    pub fn set_font_size(&self, value: f64) {
        unsafe { crate::ffi::ca_text_layer_set_font_size(self.as_layer_ptr(), value) };
    }

    pub fn set_foreground_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_text_layer_set_foreground_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn foreground_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_text_layer_get_foreground_color(
                self.as_layer_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn alignment_mode(&self) -> TextAlignmentMode {
        TextAlignmentMode::from_raw(unsafe {
            crate::ffi::ca_text_layer_get_alignment_mode(self.as_layer_ptr())
        })
    }

    pub fn set_alignment_mode(&self, value: TextAlignmentMode) {
        unsafe { crate::ffi::ca_text_layer_set_alignment_mode(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    pub fn truncation_mode(&self) -> TextTruncationMode {
        TextTruncationMode::from_raw(unsafe {
            crate::ffi::ca_text_layer_get_truncation_mode(self.as_layer_ptr())
        })
    }

    pub fn set_truncation_mode(&self, value: TextTruncationMode) {
        unsafe { crate::ffi::ca_text_layer_set_truncation_mode(self.as_layer_ptr(), value as i32) };
    }
}

impl GradientLayer {
    pub fn set_colors(&self, colors: &[&Color]) {
        let raw: Vec<*mut core::ffi::c_void> = colors.iter().map(|color| color.as_ptr()).collect();
        unsafe {
            crate::ffi::ca_gradient_layer_set_colors(self.as_layer_ptr(), raw.as_ptr(), raw.len())
        };
    }

    #[must_use]
    pub fn colors(&self) -> Vec<Color> {
        let count = unsafe { crate::ffi::ca_gradient_layer_color_count(self.as_layer_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Color::from_raw(crate::ffi::ca_gradient_layer_color_at(
                    self.as_layer_ptr(),
                    index,
                ))
            })
            .collect()
    }

    pub fn set_locations(&self, values: &[f64]) {
        unsafe {
            crate::ffi::ca_gradient_layer_set_locations(
                self.as_layer_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
    }

    #[must_use]
    pub fn locations(&self) -> Vec<f64> {
        let count = unsafe { crate::ffi::ca_gradient_layer_location_count(self.as_layer_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_gradient_layer_location_at(self.as_layer_ptr(), index)
            })
            .collect()
    }

    #[must_use]
    pub fn start_point(&self) -> CGPoint {
        let mut point = CGPoint::zero();
        let ok = unsafe {
            crate::ffi::ca_gradient_layer_get_start_point(
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

    pub fn set_start_point(&self, value: CGPoint) {
        unsafe {
            crate::ffi::ca_gradient_layer_set_start_point(self.as_layer_ptr(), value.x, value.y)
        };
    }

    #[must_use]
    pub fn end_point(&self) -> CGPoint {
        let mut point = CGPoint::zero();
        let ok = unsafe {
            crate::ffi::ca_gradient_layer_get_end_point(
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

    pub fn set_end_point(&self, value: CGPoint) {
        unsafe {
            crate::ffi::ca_gradient_layer_set_end_point(self.as_layer_ptr(), value.x, value.y)
        };
    }

    #[must_use]
    pub fn gradient_type(&self) -> GradientType {
        GradientType::from_raw(unsafe {
            crate::ffi::ca_gradient_layer_get_type(self.as_layer_ptr())
        })
    }

    pub fn set_gradient_type(&self, value: GradientType) {
        unsafe { crate::ffi::ca_gradient_layer_set_type(self.as_layer_ptr(), value as i32) };
    }
}

impl MetalLayer {
    pub fn set_device(&self, device: Option<&apple_metal::MetalDevice>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_device(
                self.as_layer_ptr(),
                device.map_or(core::ptr::null_mut(), apple_metal::MetalDevice::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn pixel_format(&self) -> usize {
        unsafe { crate::ffi::ca_metal_layer_get_pixel_format(self.as_layer_ptr()) }
    }

    pub fn set_pixel_format(&self, pixel_format: usize) {
        unsafe { crate::ffi::ca_metal_layer_set_pixel_format(self.as_layer_ptr(), pixel_format) };
    }

    #[must_use]
    pub fn drawable_size(&self) -> CGSize {
        let mut size = CGSize::zero();
        let ok = unsafe {
            crate::ffi::ca_metal_layer_get_drawable_size(
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

    pub fn set_drawable_size(&self, size: CGSize) {
        unsafe {
            crate::ffi::ca_metal_layer_set_drawable_size(
                self.as_layer_ptr(),
                size.width,
                size.height,
            )
        };
    }

    #[must_use]
    pub fn next_drawable(&self) -> Option<MetalDrawable> {
        unsafe {
            MetalDrawable::from_raw(crate::ffi::ca_metal_layer_next_drawable(
                self.as_layer_ptr(),
            ))
        }
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
