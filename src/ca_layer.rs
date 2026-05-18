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
    /// Returns the layer's z position.
    pub fn z_position(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_z_position(self.as_ptr()) }
    }

    /// Sets the layer's z position.
    pub fn set_z_position(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_z_position(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's anchor point z.
    pub fn anchor_point_z(&self) -> f64 {
        unsafe { crate::ffi::ca_layer_get_anchor_point_z(self.as_ptr()) }
    }

    /// Sets the layer's anchor point z.
    pub fn set_anchor_point_z(&self, value: f64) {
        unsafe { crate::ffi::ca_layer_set_anchor_point_z(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns the layer's sublayer transform.
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

    /// Sets the layer's sublayer transform.
    pub fn set_sublayer_transform(&self, transform: Transform3D) {
        unsafe {
            crate::ffi::ca_layer_set_sublayer_transform(
                self.as_ptr(),
                (&transform as *const Transform3D).cast::<core::ffi::c_void>(),
            )
        };
    }

    #[must_use]
    /// Returns whether the layer double sided.
    pub fn is_double_sided(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_double_sided(self.as_ptr()) }
    }

    /// Sets the layer's double sided.
    pub fn set_double_sided(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_double_sided(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns whether the layer geometry flipped.
    pub fn is_geometry_flipped(&self) -> bool {
        unsafe { crate::ffi::ca_layer_get_geometry_flipped(self.as_ptr()) }
    }

    /// Sets the layer's geometry flipped.
    pub fn set_geometry_flipped(&self, value: bool) {
        unsafe { crate::ffi::ca_layer_set_geometry_flipped(self.as_ptr(), value) };
    }

    #[must_use]
    /// Returns whether tone-map mode is supported.
    pub fn supports_tone_map_mode() -> bool {
        unsafe { crate::ffi::ca_layer_supports_tone_map_mode() }
    }

    #[must_use]
    /// Returns the layer's tone map mode.
    pub fn tone_map_mode(&self) -> ToneMapMode {
        ToneMapMode::from_raw(unsafe { crate::ffi::ca_layer_get_tone_map_mode(self.as_ptr()) })
    }

    /// Sets the layer's tone map mode.
    pub fn set_tone_map_mode(&self, value: ToneMapMode) {
        unsafe { crate::ffi::ca_layer_set_tone_map_mode(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns whether preferred dynamic range is supported.
    pub fn supports_preferred_dynamic_range() -> bool {
        unsafe { crate::ffi::ca_layer_supports_preferred_dynamic_range() }
    }

    #[must_use]
    /// Returns the layer's preferred dynamic range.
    pub fn preferred_dynamic_range(&self) -> DynamicRange {
        DynamicRange::from_raw(unsafe {
            crate::ffi::ca_layer_get_preferred_dynamic_range(self.as_ptr())
        })
    }

    /// Sets the layer's preferred dynamic range.
    pub fn set_preferred_dynamic_range(&self, value: DynamicRange) {
        unsafe { crate::ffi::ca_layer_set_preferred_dynamic_range(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the layer's contents format.
    pub fn contents_format(&self) -> ContentsFormat {
        ContentsFormat::from_raw(unsafe { crate::ffi::ca_layer_get_contents_format(self.as_ptr()) })
    }

    /// Sets the layer's contents format.
    pub fn set_contents_format(&self, value: ContentsFormat) {
        unsafe { crate::ffi::ca_layer_set_contents_format(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the layer's minification filter.
    pub fn minification_filter(&self) -> ContentsFilter {
        ContentsFilter::from_raw(unsafe {
            crate::ffi::ca_layer_get_minification_filter(self.as_ptr())
        })
    }

    /// Sets the layer's minification filter.
    pub fn set_minification_filter(&self, value: ContentsFilter) {
        unsafe { crate::ffi::ca_layer_set_minification_filter(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the layer's magnification filter.
    pub fn magnification_filter(&self) -> ContentsFilter {
        ContentsFilter::from_raw(unsafe {
            crate::ffi::ca_layer_get_magnification_filter(self.as_ptr())
        })
    }

    /// Sets the layer's magnification filter.
    pub fn set_magnification_filter(&self, value: ContentsFilter) {
        unsafe { crate::ffi::ca_layer_set_magnification_filter(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the layer's edge antialiasing mask.
    pub fn edge_antialiasing_mask(&self) -> EdgeAntialiasingMask {
        EdgeAntialiasingMask::from(unsafe {
            crate::ffi::ca_layer_get_edge_antialiasing_mask(self.as_ptr())
        })
    }

    /// Sets the layer's edge antialiasing mask.
    pub fn set_edge_antialiasing_mask(&self, value: EdgeAntialiasingMask) {
        unsafe { crate::ffi::ca_layer_set_edge_antialiasing_mask(self.as_ptr(), value.bits()) };
    }

    #[must_use]
    /// Returns the layer's masked corners.
    pub fn masked_corners(&self) -> CornerMask {
        CornerMask::from(unsafe { crate::ffi::ca_layer_get_masked_corners(self.as_ptr()) })
    }

    /// Sets the layer's masked corners.
    pub fn set_masked_corners(&self, value: CornerMask) {
        unsafe { crate::ffi::ca_layer_set_masked_corners(self.as_ptr(), value.bits()) };
    }

    #[must_use]
    /// Returns the layer's corner curve.
    pub fn corner_curve(&self) -> CornerCurve {
        CornerCurve::from_raw(unsafe { crate::ffi::ca_layer_get_corner_curve(self.as_ptr()) })
    }

    /// Sets the layer's corner curve.
    pub fn set_corner_curve(&self, value: CornerCurve) {
        unsafe { crate::ffi::ca_layer_set_corner_curve(self.as_ptr(), value as i32) };
    }

    #[must_use]
    /// Returns the expansion factor for a corner curve.
    pub fn corner_curve_expansion_factor(curve: CornerCurve) -> f64 {
        unsafe { crate::ffi::ca_layer_corner_curve_expansion_factor(curve as i32) }
    }

    #[must_use]
    /// Returns the layer's autoresizing mask.
    pub fn autoresizing_mask(&self) -> AutoresizingMask {
        AutoresizingMask::from(unsafe { crate::ffi::ca_layer_get_autoresizing_mask(self.as_ptr()) })
    }

    /// Sets the layer's autoresizing mask.
    pub fn set_autoresizing_mask(&self, value: AutoresizingMask) {
        unsafe { crate::ffi::ca_layer_set_autoresizing_mask(self.as_ptr(), value.bits()) };
    }

    /// Sets the layer delegate.
    pub fn set_delegate(&self, delegate: Option<&LayerDelegate>) {
        unsafe {
            crate::ffi::ca_layer_set_delegate(
                self.as_ptr(),
                delegate.map_or(core::ptr::null_mut(), LayerDelegate::as_ptr),
            )
        };
    }

    /// Forces the layer to redraw immediately.
    pub fn display(&self) {
        unsafe { crate::ffi::ca_layer_display(self.as_ptr()) };
    }

    /// Marks the layer as needing display.
    pub fn set_needs_display(&self) {
        unsafe { crate::ffi::ca_layer_set_needs_display(self.as_ptr()) };
    }

    /// Displays the layer if it is marked dirty.
    pub fn display_if_needed(&self) {
        unsafe { crate::ffi::ca_layer_display_if_needed(self.as_ptr()) };
    }

    /// Marks the layer as needing layout.
    pub fn set_needs_layout(&self) {
        unsafe { crate::ffi::ca_layer_set_needs_layout(self.as_ptr()) };
    }

    /// Lays out the layer immediately if needed.
    pub fn layout_if_needed(&self) {
        unsafe { crate::ffi::ca_layer_layout_if_needed(self.as_ptr()) };
    }

    /// Lays out the layer sublayers immediately.
    pub fn layout_sublayers(&self) {
        unsafe { crate::ffi::ca_layer_layout_sublayers(self.as_ptr()) };
    }

    #[must_use]
    /// Returns the layer's name.
    pub fn name(&self) -> Option<String> {
        take_c_string(unsafe { crate::ffi::ca_layer_get_name(self.as_ptr()) })
    }

    /// Sets the layer's name.
    pub fn set_name(&self, value: &str) {
        if let Some(value) = cstring_from_str(value) {
            unsafe { crate::ffi::ca_layer_set_name(self.as_ptr(), value.as_ptr()) };
        }
    }

    #[must_use]
    /// Returns the default animation for a layer action key.
    pub fn default_action_for_key(event: &str) -> Option<Animation> {
        let event = cstring_from_str(event)?;
        unsafe { Animation::from_raw(crate::ffi::ca_layer_default_action_for_key(event.as_ptr())) }
    }

    #[must_use]
    /// Returns the default action handle for a layer action key.
    pub fn default_action_handle_for_key(event: &str) -> Option<Action> {
        let event = cstring_from_str(event)?;
        unsafe {
            Action::from_raw(crate::ffi::ca_layer_default_action_handle_for_key(
                event.as_ptr(),
            ))
        }
    }

    #[must_use]
    /// Returns the animation registered for a layer action key.
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
    /// Returns the action handle registered for a layer action key.
    pub fn action_handle_for_key(&self, event: &str) -> Option<Action> {
        let event = cstring_from_str(event)?;
        unsafe {
            Action::from_raw(crate::ffi::ca_layer_action_handle_for_key(
                self.as_ptr(),
                event.as_ptr(),
            ))
        }
    }

    /// Associates an animation with a layer action key.
    pub fn set_action_for_key<A: AnimationLike>(&self, event: &str, action: Option<&A>) {
        self.set_action_handle_for_key(event, action);
    }

    /// Associates an action handle with a layer action key.
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

    /// Clears the action registered for a layer action key.
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
    /// Returns the layer layout manager.
    pub fn layout_manager(&self) -> Option<LayoutManager> {
        unsafe { LayoutManager::from_raw(crate::ffi::ca_layer_get_layout_manager(self.as_ptr())) }
    }

    /// Sets the layer layout manager.
    pub fn set_layout_manager(&self, value: Option<&LayoutManager>) {
        unsafe {
            crate::ffi::ca_layer_set_layout_manager(
                self.as_ptr(),
                value.map_or(core::ptr::null_mut(), LayoutManager::as_ptr),
            )
        };
    }

    #[must_use]
    /// Returns the layer constraints.
    pub fn constraints(&self) -> Vec<Constraint> {
        let count = unsafe { crate::ffi::ca_layer_constraint_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                Constraint::from_raw(crate::ffi::ca_layer_constraint_at(self.as_ptr(), index))
            })
            .collect()
    }

    /// Sets the layer constraints.
    pub fn set_constraints(&self, constraints: &[&Constraint]) {
        let raw: Vec<*mut core::ffi::c_void> =
            constraints.iter().map(|value| value.as_ptr()).collect();
        unsafe { crate::ffi::ca_layer_set_constraints(self.as_ptr(), raw.as_ptr(), raw.len()) };
    }

    /// Adds a constraint to the layer.
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
