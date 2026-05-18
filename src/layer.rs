use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref};

use apple_cf::cg::CGImage;
use apple_cf::cg::{CGPoint, CGRect, CGSize};

use crate::animation::{Animation, AnimationLike};
use crate::ca_action::Action;
use crate::color::Color;
use crate::path::Path;
use crate::private::{cstring_from_str, handle_type};
use crate::transform::Transform3D;

handle_type!(Layer);

/// Trait for wrappers that can yield a `CALayer` handle.
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
        /// Safe wrapper around the corresponding `Core Animation` layer type.
        pub struct $name {
            inner: Layer,
        }

        impl $name {
            #[must_use]
            /// Creates a new wrapper instance.
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
/// Mirrors `CALayerContentsGravity` values. See <https://developer.apple.com/documentation/quartzcore/calayercontentsgravity>.
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
/// Mirrors `Core Animation` tone-mapping mode values.
pub enum ToneMapMode {
    Automatic = 0,
    Never = 1,
    IfSupported = 2,
}

impl ToneMapMode {
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Never,
            2 => Self::IfSupported,
            _ => Self::Automatic,
        }
    }
}

macro_rules! bitmask_type {
    ($name:ident, $ty:ty, { $($const_name:ident = $value:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        #[repr(transparent)]
        /// Bitmask wrapper for a `Core Animation` option set.
        pub struct $name($ty);

        impl $name {
            /// Empty bitmask value.
            pub const NONE: Self = Self(0);
            $(pub const $const_name: Self = Self($value);)+

            #[must_use]
            /// Returns an empty bitmask.
            pub const fn empty() -> Self {
                Self::NONE
            }

            #[must_use]
            /// Returns the raw bit pattern.
            pub const fn bits(self) -> $ty {
                self.0
            }

            #[must_use]
            /// Returns whether all bits in `other` are set.
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
    };
}

bitmask_type!(AutoresizingMask, u32, {
    MIN_X_MARGIN = 1_u32 << 0,
    WIDTH_SIZABLE = 1_u32 << 1,
    MAX_X_MARGIN = 1_u32 << 2,
    MIN_Y_MARGIN = 1_u32 << 3,
    HEIGHT_SIZABLE = 1_u32 << 4,
    MAX_Y_MARGIN = 1_u32 << 5,
});

bitmask_type!(EdgeAntialiasingMask, u32, {
    LEFT_EDGE = 1_u32 << 0,
    RIGHT_EDGE = 1_u32 << 1,
    BOTTOM_EDGE = 1_u32 << 2,
    TOP_EDGE = 1_u32 << 3,
});

bitmask_type!(CornerMask, u64, {
    MIN_X_MIN_Y = 1_u64 << 0,
    MAX_X_MIN_Y = 1_u64 << 1,
    MIN_X_MAX_Y = 1_u64 << 2,
    MAX_X_MAX_Y = 1_u64 << 3,
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CALayerContentsFormat` values. See <https://developer.apple.com/documentation/quartzcore/calayercontentsformat>.
pub enum ContentsFormat {
    RGBA8Uint = 0,
    RGBA16Float = 1,
    Gray8Uint = 2,
    Automatic = 3,
}

impl ContentsFormat {
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::RGBA16Float,
            2 => Self::Gray8Uint,
            3 => Self::Automatic,
            _ => Self::RGBA8Uint,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CALayerContentsFilter` values. See <https://developer.apple.com/documentation/quartzcore/calayercontentsfilter>.
pub enum ContentsFilter {
    Nearest = 0,
    Linear = 1,
    Trilinear = 2,
}

impl ContentsFilter {
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Linear,
            2 => Self::Trilinear,
            _ => Self::Nearest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CALayerCornerCurve` values. See <https://developer.apple.com/documentation/quartzcore/calayercornercurve>.
pub enum CornerCurve {
    Circular = 0,
    Continuous = 1,
}

impl CornerCurve {
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Continuous,
            _ => Self::Circular,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `Core Animation` preferred dynamic-range values.
pub enum DynamicRange {
    Automatic = 0,
    Standard = 1,
    ConstrainedHigh = 2,
    High = 3,
}

impl DynamicRange {
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) const fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Standard,
            2 => Self::ConstrainedHigh,
            3 => Self::High,
            _ => Self::Automatic,
        }
    }
}

/// Well-known `CALayer` action dictionary keys.
pub struct LayerActionKeys;

impl LayerActionKeys {
    /// Action key for layer insertion events.
    pub const ON_ORDER_IN: &str = "onOrderIn";
    /// Action key for layer removal events.
    pub const ON_ORDER_OUT: &str = "onOrderOut";
    /// Action key for transition animations.
    pub const TRANSITION: &str = "transition";
}

struct LayerDisplayContext {
    callback: Box<dyn FnMut(Layer)>,
}

struct LayerLayoutContext {
    callback: Box<dyn FnMut(Layer)>,
}

type RawLayerActionCallback = dyn FnMut(Layer, &str) -> *mut c_void;

struct LayerActionContext {
    callback: Box<RawLayerActionCallback>,
}

/// Safe callback bridge for `CALayerDelegate`. See <https://developer.apple.com/documentation/quartzcore/calayerdelegate>.
pub struct LayerDelegate {
    ptr: *mut c_void,
    display_context: Option<*mut LayerDisplayContext>,
    layout_context: Option<*mut LayerLayoutContext>,
    action_context: Option<*mut LayerActionContext>,
}

impl core::fmt::Debug for LayerDelegate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LayerDelegate")
            .field("ptr", &self.ptr)
            .field("has_display", &self.display_context.is_some())
            .field("has_layout", &self.layout_context.is_some())
            .field("has_action", &self.action_context.is_some())
            .finish()
    }
}

impl LayerDelegate {
    #[must_use]
    /// Creates a `CALayerDelegate` callback bridge.
    pub fn new() -> Option<Self> {
        let ptr = unsafe { crate::ffi::ca_layer_delegate_new() };
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                display_context: None,
                layout_context: None,
                action_context: None,
            })
        }
    }

    /// Installs a callback for layer display requests.
    pub fn set_display_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Layer) + 'static,
    {
        self.clear_display_callback();
        let context = Box::into_raw(Box::new(LayerDisplayContext {
            callback: Box::new(callback),
        }));
        unsafe {
            crate::ffi::ca_layer_delegate_set_display_callback(
                self.ptr,
                Some(layer_delegate_display_trampoline),
                context.cast(),
            )
        };
        self.display_context = Some(context);
    }

    /// Clears the layer display callback.
    pub fn clear_display_callback(&mut self) {
        unsafe {
            crate::ffi::ca_layer_delegate_set_display_callback(
                self.ptr,
                None,
                core::ptr::null_mut(),
            )
        };
        if let Some(context) = self.display_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    /// Installs a callback for layer layout requests.
    pub fn set_layout_sublayers_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Layer) + 'static,
    {
        self.clear_layout_sublayers_callback();
        let context = Box::into_raw(Box::new(LayerLayoutContext {
            callback: Box::new(callback),
        }));
        unsafe {
            crate::ffi::ca_layer_delegate_set_layout_callback(
                self.ptr,
                Some(layer_delegate_layout_trampoline),
                context.cast(),
            )
        };
        self.layout_context = Some(context);
    }

    /// Clears the layer layout callback.
    pub fn clear_layout_sublayers_callback(&mut self) {
        unsafe {
            crate::ffi::ca_layer_delegate_set_layout_callback(self.ptr, None, core::ptr::null_mut())
        };
        if let Some(context) = self.layout_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    /// Installs a callback that returns animations for layer action keys.
    pub fn set_action_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Layer, &str) -> Option<Animation> + 'static,
    {
        self.clear_action_callback();
        let mut callback = callback;
        let context = Box::into_raw(Box::new(LayerActionContext {
            callback: Box::new(move |layer, key| {
                callback(layer, key).map_or(core::ptr::null_mut(), |animation| unsafe {
                    crate::ffi::ca_retain(animation.as_ptr())
                })
            }),
        }));
        unsafe {
            crate::ffi::ca_layer_delegate_set_action_callback(
                self.ptr,
                Some(layer_delegate_action_trampoline),
                context.cast(),
            )
        };
        self.action_context = Some(context);
    }

    /// Installs a callback that returns action handles for layer action keys.
    pub fn set_action_provider<F>(&mut self, callback: F)
    where
        F: FnMut(Layer, &str) -> Option<Action> + 'static,
    {
        self.clear_action_callback();
        let mut callback = callback;
        let context = Box::into_raw(Box::new(LayerActionContext {
            callback: Box::new(move |layer, key| {
                callback(layer, key).map_or(core::ptr::null_mut(), |action| unsafe {
                    crate::ffi::ca_retain(action.as_ptr())
                })
            }),
        }));
        unsafe {
            crate::ffi::ca_layer_delegate_set_action_callback(
                self.ptr,
                Some(layer_delegate_action_trampoline),
                context.cast(),
            )
        };
        self.action_context = Some(context);
    }

    /// Clears the layer action callback.
    pub fn clear_action_callback(&mut self) {
        unsafe {
            crate::ffi::ca_layer_delegate_set_action_callback(self.ptr, None, core::ptr::null_mut())
        };
        if let Some(context) = self.action_context.take() {
            unsafe { drop(Box::from_raw(context)) };
        }
    }

    /// Returns the underlying raw pointer.
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for LayerDelegate {
    fn drop(&mut self) {
        self.clear_display_callback();
        self.clear_layout_sublayers_callback();
        self.clear_action_callback();
        if !self.ptr.is_null() {
            unsafe { crate::ffi::ca_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

unsafe extern "C" fn layer_delegate_display_trampoline(
    context: *mut c_void,
    layer_handle: *mut c_void,
) {
    if context.is_null() || layer_handle.is_null() {
        return;
    }

    let context = unsafe { &mut *context.cast::<LayerDisplayContext>() };
    let layer = unsafe { Layer::from_raw_unchecked(layer_handle) };
    (context.callback)(layer);
}

unsafe extern "C" fn layer_delegate_layout_trampoline(
    context: *mut c_void,
    layer_handle: *mut c_void,
) {
    if context.is_null() || layer_handle.is_null() {
        return;
    }

    let context = unsafe { &mut *context.cast::<LayerLayoutContext>() };
    let layer = unsafe { Layer::from_raw_unchecked(layer_handle) };
    (context.callback)(layer);
}

unsafe extern "C" fn layer_delegate_action_trampoline(
    context: *mut c_void,
    layer_handle: *mut c_void,
    key: *const c_char,
) -> *mut c_void {
    if context.is_null() || layer_handle.is_null() {
        return core::ptr::null_mut();
    }

    let context = unsafe { &mut *context.cast::<LayerActionContext>() };
    let layer = unsafe { Layer::from_raw_unchecked(layer_handle) };
    let key = if key.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(key) }.to_str().unwrap_or_default()
    };
    (context.callback)(layer, key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors `CAShapeLayerLineCap` values. See <https://developer.apple.com/documentation/quartzcore/cashapelayerlinecap>.
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
/// Mirrors `CAShapeLayerLineJoin` values. See <https://developer.apple.com/documentation/quartzcore/cashapelayerlinejoin>.
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
/// Mirrors `CATextLayerAlignmentMode` values. See <https://developer.apple.com/documentation/quartzcore/catextlayeralignmentmode>.
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
/// Mirrors `CATextLayerTruncationMode` values. See <https://developer.apple.com/documentation/quartzcore/catextlayertruncationmode>.
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
/// Mirrors `CAGradientLayerType` values. See <https://developer.apple.com/documentation/quartzcore/cagradientlayertype>.
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
/// Safe wrapper around `CAMetalDrawable`. See <https://developer.apple.com/documentation/quartzcore/cametaldrawable>.
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
    /// Converts a raw `Core Animation` value into this type.
    pub(crate) unsafe fn from_raw(ptr: *mut core::ffi::c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, owned: true })
        }
    }

    #[must_use]
    /// Returns the drawable Metal texture.
    pub fn texture(&self) -> Option<apple_metal::MetalTexture> {
        let ptr = unsafe { crate::ffi::ca_metal_drawable_get_texture(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { apple_metal::MetalTexture::from_raw(ptr) })
        }
    }

    /// Presents the drawable to the display.
    pub fn present(&self) {
        unsafe { crate::ffi::ca_metal_drawable_present(self.ptr) };
    }
}

impl Layer {
    #[must_use]
    /// Creates a new `CALayer` wrapper.
    pub fn new() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_layer_new()) }
    }

    #[must_use]
    /// Returns the layer's frame.
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

    /// Sets the layer's frame.
    pub fn set_frame(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_layer_set_frame(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    #[must_use]
    /// Returns the layer's bounds.
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

    /// Sets the layer's bounds.
    pub fn set_bounds(&self, rect: CGRect) {
        unsafe {
            crate::ffi::ca_layer_set_bounds(
                self.as_ptr(),
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
        };
    }

    #[must_use]
    /// Returns the layer's position.
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

    /// Sets the layer's position.
    pub fn set_position(&self, point: CGPoint) {
        unsafe { crate::ffi::ca_layer_set_position(self.as_ptr(), point.x, point.y) };
    }

    #[must_use]
    /// Returns the layer's anchor point.
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

    /// Sets the layer's anchor point.
    pub fn set_anchor_point(&self, point: CGPoint) {
        unsafe { crate::ffi::ca_layer_set_anchor_point(self.as_ptr(), point.x, point.y) };
    }

    #[must_use]
    /// Returns the layer's transform.
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

    /// Sets the layer's transform.
    pub fn set_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_layer_set_transform(
                self.as_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    #[must_use]
    /// Returns the layer sublayers.
    pub fn sublayers(&self) -> Vec<Layer> {
        let count = unsafe { crate::ffi::ca_layer_sublayer_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Layer::from_raw(crate::ffi::ca_layer_sublayer_at(self.as_ptr(), index))
            })
            .collect()
    }

    /// Adds a child layer as a sublayer.
    pub fn add_sublayer<L: LayerLike>(&self, child: &L) {
        unsafe { crate::ffi::ca_layer_add_sublayer(self.as_ptr(), child.as_layer_ptr()) };
    }

    /// Removes the layer from its superlayer.
    pub fn remove_from_superlayer(&self) {
        unsafe { crate::ffi::ca_layer_remove_from_superlayer(self.as_ptr()) };
    }

    /// Sets the layer contents image.
    pub fn set_contents(&self, image: Option<&CGImage>) {
        let ptr = image.map_or(core::ptr::null_mut(), |image| image.as_ptr());
        unsafe { crate::ffi::ca_layer_set_contents(self.as_ptr(), ptr) };
    }

    #[must_use]
    /// Returns the layer contents image.
    pub fn contents(&self) -> Option<CGImage> {
        let ptr = unsafe { crate::ffi::ca_layer_get_contents(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CGImage::from_raw(ptr) })
        }
    }

    #[must_use]
    /// Returns the layer's contents scale.
    pub fn contents_scale(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_contents_scale(self.as_ptr()) }
    }

    /// Sets the layer's contents scale.
    pub fn set_contents_scale(&self, scale: f64) {
        unsafe { crate::ffi::ca_layer_set_contents_scale(self.as_ptr(), scale) };
    }

    /// Sets the layer background color.
    pub fn set_background_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_background_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the layer background color.
    pub fn background_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_background_color(self.as_ptr())) }
    }

    /// Sets the layer border color.
    pub fn set_border_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_border_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the layer border color.
    pub fn border_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_border_color(self.as_ptr())) }
    }

    #[must_use]
    /// Returns the layer's border width.
    pub fn border_width(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_border_width(self.as_ptr()) }
    }

    /// Sets the layer's border width.
    pub fn set_border_width(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_border_width(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's corner radius.
    pub fn corner_radius(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_corner_radius(self.as_ptr()) }
    }

    /// Sets the layer's corner radius.
    pub fn set_corner_radius(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_corner_radius(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's opacity.
    pub fn opacity(&self) -> f32 {
        unsafe { crate::ffi::ca_layer_get_opacity(self.as_ptr()) }
    }

    /// Sets the layer's opacity.
    pub fn set_opacity(&self, value: f32) {
        unsafe { crate::ffi::ca_layer_set_opacity(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the layer hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe { crate::ffi::ca_layer_is_hidden(self.as_ptr()) }
    }

    /// Sets the layer's hidden.
    pub fn set_hidden(&self, hidden: bool) {
        unsafe { crate::ffi::ca_layer_set_hidden(self.as_ptr(), hidden) };
    }

    /// Sets the layer mask.
    pub fn set_mask<L: LayerLike>(&self, mask: Option<&L>) {
        unsafe {
            crate::ffi::ca_layer_set_mask(
                self.as_ptr(),
                mask.map_or(core::ptr::null_mut(), LayerLike::as_layer_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the layer mask.
    pub fn mask(&self) -> Option<Layer> {
        unsafe { Layer::from_raw(crate::ffi::ca_layer_get_mask(self.as_ptr())) }
    }

    #[must_use]
    /// Returns whether the layer masks to bounds.
    pub fn masks_to_bounds(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_masks_to_bounds(self.as_ptr()) }
    }

    /// Sets the layer's masks to bounds.
    pub fn set_masks_to_bounds(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_masks_to_bounds(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's shadow offset.
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

    /// Sets the layer's shadow offset.
    pub fn set_shadow_offset(&self, value: CGSize) {
        unsafe { crate::ffi::ca_layer_set_shadow_offset(self.as_ptr(), value.width, value.height) };
    }

    #[must_use]
    /// Returns the layer's shadow radius.
    pub fn shadow_radius(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_shadow_radius(self.as_ptr()) }
    }

    /// Sets the layer's shadow radius.
    pub fn set_shadow_radius(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_shadow_radius(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's shadow opacity.
    pub fn shadow_opacity(&self) -> f32 {
        unsafe { crate::ffi::ca_layer_get_shadow_opacity(self.as_ptr()) }
    }

    /// Sets the layer's shadow opacity.
    pub fn set_shadow_opacity(&self, value: f32) {
        unsafe { crate::ffi::ca_layer_set_shadow_opacity(self.as_ptr(), value) };
    }

    /// Sets the layer shadow color.
    pub fn set_shadow_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_layer_set_shadow_color(
                self.as_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the layer shadow color.
    pub fn shadow_color(&self) -> Option<Color> {
        unsafe { Color::from_raw(crate::ffi::ca_layer_get_shadow_color(self.as_ptr())) }
    }

    #[must_use]
    /// Returns the layer's contents gravity.
    pub fn contents_gravity(&self) -> ContentsGravity {
        ContentsGravity::from_raw(unsafe {
            crate::ffi::ca_layer_get_contents_gravity(self.as_ptr())
        })
    }

    /// Sets the layer's contents gravity.
    pub fn set_contents_gravity(&self, gravity: ContentsGravity) {
        unsafe { crate::ffi::ca_layer_set_contents_gravity(self.as_ptr(), gravity as i32) };
    }

    /// Adds an animation to the layer under an optional key.
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

    /// Removes the animation registered for a key.
    pub fn remove_animation(&self, key: &str) {
        if let Some(key) = cstring_from_str(key) {
            unsafe { crate::ffi::ca_layer_remove_animation(self.as_ptr(), key.as_ptr()) };
        }
    }
}

impl ShapeLayer {
    /// Sets the shape layer's path.
    pub fn set_path(&self, path: Option<&Path>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_path(
                self.as_layer_ptr(),
                path.map_or(core::ptr::null_mut(), Path::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the shape layer's path.
    pub fn path(&self) -> Option<Path> {
        unsafe { Path::from_raw(crate::ffi::ca_shape_layer_get_path(self.as_layer_ptr())) }
    }

    /// Sets the shape layer's fill color.
    pub fn set_fill_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_fill_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the shape layer's fill color.
    pub fn fill_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_shape_layer_get_fill_color(
                self.as_layer_ptr(),
            ))
        }
    }

    /// Sets the shape layer's stroke color.
    pub fn set_stroke_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_shape_layer_set_stroke_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the shape layer's stroke color.
    pub fn stroke_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_shape_layer_get_stroke_color(
                self.as_layer_ptr(),
            ))
        }
    }

    #[must_use]
    /// Returns the shape layer's line width.
    pub fn line_width(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_line_width(self.as_layer_ptr()) }
    }

    /// Sets the shape layer's line width.
    pub fn set_line_width(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_line_width(self.as_layer_ptr(), value) };
    }

    #[must_use]
    /// Returns the shape layer's line cap.
    pub fn line_cap(&self) -> LineCap {
        LineCap::from_raw(unsafe { crate::ffi::ca_shape_layer_get_line_cap(self.as_layer_ptr()) })
    }

    /// Sets the shape layer's line cap.
    pub fn set_line_cap(&self, value: LineCap) {
        unsafe { crate::ffi::ca_shape_layer_set_line_cap(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the shape layer's line join.
    pub fn line_join(&self) -> LineJoin {
        LineJoin::from_raw(unsafe { crate::ffi::ca_shape_layer_get_line_join(self.as_layer_ptr()) })
    }

    /// Sets the shape layer's line join.
    pub fn set_line_join(&self, value: LineJoin) {
        unsafe { crate::ffi::ca_shape_layer_set_line_join(self.as_layer_ptr(), value as i32) };
    }

    /// Sets the shape layer line-dash pattern.
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
    /// Returns the shape layer line-dash pattern.
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
    /// Returns the shape layer's miter limit.
    pub fn miter_limit(&self) -> f64 {
        unsafe { crate::ffi::ca_shape_layer_get_miter_limit(self.as_layer_ptr()) }
    }

    /// Sets the shape layer's miter limit.
    pub fn set_miter_limit(&self, value: f64) {
        unsafe { crate::ffi::ca_shape_layer_set_miter_limit(self.as_layer_ptr(), value) };
    }
}

impl TextLayer {
    /// Sets the text layer string.
    pub fn set_string(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_text_layer_set_string(self.as_layer_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    /// Returns the text layer string.
    pub fn string(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_text_layer_get_string(self.as_layer_ptr()) })
    }

    /// Sets the text layer's font name.
    pub fn set_font_name(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_text_layer_set_font_name(self.as_layer_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    /// Returns the text layer's font name.
    pub fn font_name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_text_layer_get_font_name(self.as_layer_ptr()) })
    }

    #[must_use]
    /// Returns the text layer's font size.
    pub fn font_size(&self) -> f64 {
        unsafe { crate::ffi::ca_text_layer_get_font_size(self.as_layer_ptr()) }
    }

    /// Sets the text layer's font size.
    pub fn set_font_size(&self, value: f64) {
        unsafe { crate::ffi::ca_text_layer_set_font_size(self.as_layer_ptr(), value) };
    }

    /// Sets the text layer's foreground color.
    pub fn set_foreground_color(&self, color: Option<&Color>) {
        unsafe {
            crate::ffi::ca_text_layer_set_foreground_color(
                self.as_layer_ptr(),
                color.map_or(core::ptr::null_mut(), Color::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the text layer's foreground color.
    pub fn foreground_color(&self) -> Option<Color> {
        unsafe {
            Color::from_raw(crate::ffi::ca_text_layer_get_foreground_color(
                self.as_layer_ptr(),
            ))
        }
    }

    #[must_use]
    /// Returns the text layer's alignment mode.
    pub fn alignment_mode(&self) -> TextAlignmentMode {
        TextAlignmentMode::from_raw(unsafe {
            crate::ffi::ca_text_layer_get_alignment_mode(self.as_layer_ptr())
        })
    }

    /// Sets the text layer's alignment mode.
    pub fn set_alignment_mode(&self, value: TextAlignmentMode) {
        unsafe { crate::ffi::ca_text_layer_set_alignment_mode(self.as_layer_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the text layer's truncation mode.
    pub fn truncation_mode(&self) -> TextTruncationMode {
        TextTruncationMode::from_raw(unsafe {
            crate::ffi::ca_text_layer_get_truncation_mode(self.as_layer_ptr())
        })
    }

    /// Sets the text layer's truncation mode.
    pub fn set_truncation_mode(&self, value: TextTruncationMode) {
        unsafe { crate::ffi::ca_text_layer_set_truncation_mode(self.as_layer_ptr(), value as i32) };
    }
}

impl GradientLayer {
    /// Sets the gradient layer colors.
    pub fn set_colors(&self, colors: &[&Color]) {
        let raw: Vec<*mut core::ffi::c_void> = colors.iter().map(|color| color.as_ptr()).collect();
        unsafe {
            crate::ffi::ca_gradient_layer_set_colors(self.as_layer_ptr(), raw.as_ptr(), raw.len())
        };
    }

    #[must_use]
    /// Returns the gradient layer colors.
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

    /// Sets the gradient stop locations.
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
    /// Returns the gradient stop locations.
    pub fn locations(&self) -> Vec<f64> {
        let count = unsafe { crate::ffi::ca_gradient_layer_location_count(self.as_layer_ptr()) };
        (0..count)
            .map(|index| unsafe {
                crate::ffi::ca_gradient_layer_location_at(self.as_layer_ptr(), index)
            })
            .collect()
    }

    #[must_use]
    /// Returns the gradient layer's start point.
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

    /// Sets the gradient layer's start point.
    pub fn set_start_point(&self, value: CGPoint) {
        unsafe {
            crate::ffi::ca_gradient_layer_set_start_point(self.as_layer_ptr(), value.x, value.y)
        };
    }

    #[must_use]
    /// Returns the gradient layer's end point.
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

    /// Sets the gradient layer's end point.
    pub fn set_end_point(&self, value: CGPoint) {
        unsafe {
            crate::ffi::ca_gradient_layer_set_end_point(self.as_layer_ptr(), value.x, value.y)
        };
    }

    #[must_use]
    /// Returns the gradient layer's gradient type.
    pub fn gradient_type(&self) -> GradientType {
        GradientType::from_raw(unsafe {
            crate::ffi::ca_gradient_layer_get_type(self.as_layer_ptr())
        })
    }

    /// Sets the gradient layer's gradient type.
    pub fn set_gradient_type(&self, value: GradientType) {
        unsafe { crate::ffi::ca_gradient_layer_set_type(self.as_layer_ptr(), value as i32) };
    }
}

impl MetalLayer {
    /// Sets the Metal device used by the layer.
    pub fn set_device(&self, device: Option<&apple_metal::MetalDevice>) {
        unsafe {
            crate::ffi::ca_metal_layer_set_device(
                self.as_layer_ptr(),
                device.map_or(core::ptr::null_mut(), apple_metal::MetalDevice::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the Metal layer's pixel format.
    pub fn pixel_format(&self) -> usize {
        unsafe { crate::ffi::ca_metal_layer_get_pixel_format(self.as_layer_ptr()) }
    }

    /// Sets the Metal layer's pixel format.
    pub fn set_pixel_format(&self, pixel_format: usize) {
        unsafe { crate::ffi::ca_metal_layer_set_pixel_format(self.as_layer_ptr(), pixel_format) };
    }

    #[must_use]
    /// Returns the Metal layer's drawable size.
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

    /// Sets the Metal layer's drawable size.
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
    /// Returns the next available Metal drawable.
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
