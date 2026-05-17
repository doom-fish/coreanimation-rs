use std::ffi::CStr;

use crate::animation::{Animation, AnimationLike};
use crate::ca_action::{Action, ActionLike};
use crate::ca_constraint::{Constraint, LayoutManager};
use crate::layer::{
    AutoresizingMask, ContentsFilter, ContentsFormat, CornerCurve, CornerMask, DynamicRange,
    EdgeAntialiasingMask, Layer, LayerDelegate, ToneMapMode,
};
use crate::private::cstring_from_str;
use crate::transform::Transform3D;

impl Layer {
    #[must_use]
    pub fn z_position(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_z_position(self.as_ptr()) }
    }

    pub fn set_z_position(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_z_position(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn anchor_point_z(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_anchor_point_z(self.as_ptr()) }
    }

    pub fn set_anchor_point_z(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_anchor_point_z(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn sublayer_transform(&self) -> Transform3D {
        let mut transform = Transform3D::identity();
        let ok = unsafe {
            crate::ffi::ca_layer_get_sublayer_transform(
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

    pub fn set_sublayer_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_layer_set_sublayer_transform(
                self.as_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    #[must_use]
    pub fn is_double_sided(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_double_sided(self.as_ptr()) }
    }

    pub fn set_double_sided(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_double_sided(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn is_geometry_flipped(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_geometry_flipped(self.as_ptr()) }
    }

    pub fn set_geometry_flipped(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_geometry_flipped(self.as_ptr(), value) };
    }

    #[must_use]
    pub fn supports_tone_map_mode() -> bool {
        unsafe { crate::ffi::ca_layer_supports_tone_map_mode() }
    }

    #[must_use]
    pub fn tone_map_mode(&self) -> ToneMapMode {
        ToneMapMode::from_raw(unsafe { crate::ffi::ca_layer_get_tone_map_mode(self.as_ptr()) })
    }

    pub fn set_tone_map_mode(&self, value: ToneMapMode) {
        unsafe { crate::ffi::ca_layer_set_tone_map_mode(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn supports_preferred_dynamic_range() -> bool {
        unsafe { crate::ffi::ca_layer_supports_preferred_dynamic_range() }
    }

    #[must_use]
    pub fn preferred_dynamic_range(&self) -> DynamicRange {
        DynamicRange::from_raw(unsafe {
            crate::ffi::ca_layer_get_preferred_dynamic_range(self.as_ptr())
        })
    }

    pub fn set_preferred_dynamic_range(&self, value: DynamicRange) {
        unsafe { crate::ffi::ca_layer_set_preferred_dynamic_range(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn contents_format(&self) -> ContentsFormat {
        ContentsFormat::from_raw(unsafe { crate::ffi::ca_layer_get_contents_format(self.as_ptr()) })
    }

    pub fn set_contents_format(&self, value: ContentsFormat) {
        unsafe { crate::ffi::ca_layer_set_contents_format(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn minification_filter(&self) -> ContentsFilter {
        ContentsFilter::from_raw(unsafe {
            crate::ffi::ca_layer_get_minification_filter(self.as_ptr())
        })
    }

    pub fn set_minification_filter(&self, value: ContentsFilter) {
        unsafe { crate::ffi::ca_layer_set_minification_filter(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn magnification_filter(&self) -> ContentsFilter {
        ContentsFilter::from_raw(unsafe {
            crate::ffi::ca_layer_get_magnification_filter(self.as_ptr())
        })
    }

    pub fn set_magnification_filter(&self, value: ContentsFilter) {
        unsafe { crate::ffi::ca_layer_set_magnification_filter(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn edge_antialiasing_mask(&self) -> EdgeAntialiasingMask {
        EdgeAntialiasingMask::from(unsafe {
            crate::ffi::ca_layer_get_edge_antialiasing_mask(self.as_ptr())
        })
    }

    pub fn set_edge_antialiasing_mask(&self, value: EdgeAntialiasingMask) {
        unsafe {
            crate::ffi::ca_layer_set_edge_antialiasing_mask(self.as_ptr(), value.bits())
        };
    }

    #[must_use]
    pub fn masked_corners(&self) -> CornerMask {
        CornerMask::from(unsafe { crate::ffi::ca_layer_get_masked_corners(self.as_ptr()) })
    }

    pub fn set_masked_corners(&self, value: CornerMask) {
        unsafe { crate::ffi::ca_layer_set_masked_corners(self.as_ptr(), value.bits()) };
    }

    #[must_use]
    pub fn corner_curve(&self) -> CornerCurve {
        CornerCurve::from_raw(unsafe { crate::ffi::ca_layer_get_corner_curve(self.as_ptr()) })
    }

    pub fn set_corner_curve(&self, value: CornerCurve) {
        unsafe { crate::ffi::ca_layer_set_corner_curve(self.as_ptr(), value as i32) };
    }

    #[must_use]
    pub fn corner_curve_expansion_factor(curve: CornerCurve) -> f64 {
        unsafe { crate::ffi::ca_layer_corner_curve_expansion_factor(curve as i32) }
    }

    #[must_use]
    pub fn autoresizing_mask(&self) -> AutoresizingMask {
        AutoresizingMask::from(unsafe { crate::ffi::ca_layer_get_autoresizing_mask(self.as_ptr()) })
    }

    pub fn set_autoresizing_mask(&self, value: AutoresizingMask) {
        unsafe { crate::ffi::ca_layer_set_autoresizing_mask(self.as_ptr(), value.bits()) };
    }

    pub fn set_delegate(&self, delegate: Option<&LayerDelegate>) {
        unsafe {
            crate::ffi::ca_layer_set_delegate(
                self.as_ptr(),
                delegate.map_or(core::ptr::null_mut(), LayerDelegate::as_ptr),
            )
        };
    }

    pub fn display(&self) {
        unsafe { crate::ffi::ca_layer_display(self.as_ptr()) };
    }

    pub fn set_needs_display(&self) {
        unsafe { crate::ffi::ca_layer_set_needs_display(self.as_ptr()) };
    }

    pub fn display_if_needed(&self) {
        unsafe { crate::ffi::ca_layer_display_if_needed(self.as_ptr()) };
    }

    pub fn set_needs_layout(&self) {
        unsafe { crate::ffi::ca_layer_set_needs_layout(self.as_ptr()) };
    }

    pub fn layout_if_needed(&self) {
        unsafe { crate::ffi::ca_layer_layout_if_needed(self.as_ptr()) };
    }

    pub fn layout_sublayers(&self) {
        unsafe { crate::ffi::ca_layer_layout_sublayers(self.as_ptr()) };
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_layer_get_name(self.as_ptr()) })
    }

    pub fn set_name(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_layer_set_name(self.as_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    pub fn default_action_for_key(event: &str) -> Option<Animation> {
        let event = cstring_from_str(event)?;
        unsafe { Animation::from_raw(crate::ffi::ca_layer_default_action_for_key(event.as_ptr())) }
    }

    #[must_use]
    pub fn default_action_handle_for_key(event: &str) -> Option<Action> {
        let event = cstring_from_str(event)?;
        unsafe { Action::from_raw(crate::ffi::ca_layer_default_action_handle_for_key(event.as_ptr())) }
    }

    #[must_use]
    pub fn action_for_key(&self, event: &str) -> Option<Animation> {
        let event = cstring_from_str(event)?;
        unsafe {
            Animation::from_raw(crate::ffi::ca_layer_action_for_key(
                self.as_ptr(),
                event.as_ptr(),
            ))
        }
    }

    #[must_use]
    pub fn action_handle_for_key(&self, event: &str) -> Option<Action> {
        let event = cstring_from_str(event)?;
        unsafe {
            Action::from_raw(crate::ffi::ca_layer_action_handle_for_key(
                self.as_ptr(),
                event.as_ptr(),
            ))
        }
    }

    pub fn set_action_for_key<A: AnimationLike>(&self, event: &str, action: Option<&A>) {
        self.set_action_handle_for_key(event, action);
    }

    pub fn set_action_handle_for_key<A: ActionLike>(&self, event: &str, action: Option<&A>) {
        if let Some(event) = cstring_from_str(event) {
            unsafe {
                crate::ffi::ca_layer_set_action_for_key(
                    self.as_ptr(),
                    event.as_ptr(),
                    action.map_or(core::ptr::null_mut(), ActionLike::as_action_ptr),
                )
            };
        }
    }

    pub fn clear_action_for_key(&self, event: &str) {
        if let Some(event) = cstring_from_str(event) {
            unsafe {
                crate::ffi::ca_layer_set_action_for_key(
                    self.as_ptr(),
                    event.as_ptr(),
                    core::ptr::null_mut(),
                )
            };
        }
    }

    #[must_use]
    pub fn layout_manager(&self) -> Option<LayoutManager> {
        unsafe { LayoutManager::from_raw(crate::ffi::ca_layer_get_layout_manager(self.as_ptr())) }
    }

    pub fn set_layout_manager(&self, value: Option<&LayoutManager>) {
        unsafe {
            crate::ffi::ca_layer_set_layout_manager(
                self.as_ptr(),
                value.map_or(core::ptr::null_mut(), LayoutManager::as_ptr),
            )
        };
    }

    #[must_use]
    pub fn constraints(&self) -> Vec<Constraint> {
        let count = unsafe { crate::ffi::ca_layer_constraint_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Constraint::from_raw(crate::ffi::ca_layer_constraint_at(self.as_ptr(), index))
            })
            .collect()
    }

    pub fn set_constraints(&self, constraints: &[&Constraint]) {
        let raw: Vec<*mut core::ffi::c_void> = constraints.iter().map(|value| value.as_ptr()).collect();
        unsafe {
            crate::ffi::ca_layer_set_constraints(self.as_ptr(), raw.as_ptr(), raw.len())
        };
    }

    pub fn add_constraint(&self, constraint: &Constraint) {
        unsafe { crate::ffi::ca_layer_add_constraint(self.as_ptr(), constraint.as_ptr()) };
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
