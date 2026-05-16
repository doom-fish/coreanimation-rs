use core::ffi::{c_char, c_void};

#[allow(clippy::upper_case_acronyms)]
pub type CVDisplayLinkOutputCallback = Option<
    unsafe extern "C" fn(
        display_link: *mut c_void,
        in_now: *const CVTimeStamp,
        in_output_time: *const CVTimeStamp,
        flags_in: u64,
        flags_out: *mut u64,
        display_link_context: *mut c_void,
    ) -> i32,
>;

#[allow(clippy::upper_case_acronyms)]
pub type TransactionCompletionCallback = Option<unsafe extern "C" fn(context: *mut c_void)>;

#[allow(clippy::upper_case_acronyms)]
pub type MetalDisplayLinkUpdateCallback =
    Option<unsafe extern "C" fn(context: *mut c_void, update_handle: *mut c_void)>;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CVSMPTETime {
    pub subframes: i16,
    pub subframe_divisor: i16,
    pub counter: u32,
    pub type_: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CVTime {
    pub time_value: i64,
    pub time_scale: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CVTimeStamp {
    pub version: u32,
    pub video_time_scale: i32,
    pub video_time: i64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub video_refresh_period: i64,
    pub smpte_time: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}

unsafe extern "C" {
    pub fn ca_retain(handle: *mut c_void) -> *mut c_void;
    pub fn ca_release(handle: *mut c_void);

    pub fn ca_color_new_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> *mut c_void;
    pub fn ca_color_get_components(handle: *mut c_void, out_components: *mut c_void) -> bool;

    pub fn ca_path_new_mutable() -> *mut c_void;
    pub fn ca_path_move_to(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_path_add_line_to(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_path_add_rect(handle: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn ca_path_add_ellipse(handle: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn ca_path_close_subpath(handle: *mut c_void);
    pub fn ca_path_get_bounding_box(handle: *mut c_void, out_rect: *mut c_void) -> bool;

    pub fn ca_layer_new() -> *mut c_void;
    pub fn ca_layer_get_frame(handle: *mut c_void, out_rect: *mut c_void) -> bool;
    pub fn ca_layer_set_frame(handle: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn ca_layer_get_bounds(handle: *mut c_void, out_rect: *mut c_void) -> bool;
    pub fn ca_layer_set_bounds(handle: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn ca_layer_get_position(handle: *mut c_void, out_point: *mut c_void) -> bool;
    pub fn ca_layer_set_position(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_layer_get_anchor_point(handle: *mut c_void, out_point: *mut c_void) -> bool;
    pub fn ca_layer_set_anchor_point(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_layer_get_transform(handle: *mut c_void, out_transform: *mut c_void) -> bool;
    pub fn ca_layer_set_transform(handle: *mut c_void, transform: *const c_void);
    pub fn ca_layer_sublayer_count(handle: *mut c_void) -> usize;
    pub fn ca_layer_sublayer_at(handle: *mut c_void, index: usize) -> *mut c_void;
    pub fn ca_layer_add_sublayer(handle: *mut c_void, child: *mut c_void);
    pub fn ca_layer_remove_from_superlayer(handle: *mut c_void);
    pub fn ca_layer_set_contents(handle: *mut c_void, image_handle: *mut c_void);
    pub fn ca_layer_get_contents(handle: *mut c_void) -> *mut c_void;
    pub fn ca_layer_get_contents_scale(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_contents_scale(handle: *mut c_void, scale: f64);
    pub fn ca_layer_set_background_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_layer_get_background_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_layer_set_border_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_layer_get_border_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_layer_get_border_width(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_border_width(handle: *mut c_void, value: f64);
    pub fn ca_layer_get_corner_radius(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_corner_radius(handle: *mut c_void, value: f64);
    pub fn ca_layer_get_opacity(handle: *mut c_void) -> f32;
    pub fn ca_layer_set_opacity(handle: *mut c_void, value: f32);
    pub fn ca_layer_is_hidden(handle: *mut c_void) -> bool;
    pub fn ca_layer_set_hidden(handle: *mut c_void, hidden: bool);
    pub fn ca_layer_set_mask(handle: *mut c_void, mask_handle: *mut c_void);
    pub fn ca_layer_get_mask(handle: *mut c_void) -> *mut c_void;
    pub fn ca_layer_get_masks_to_bounds(handle: *mut c_void) -> bool;
    pub fn ca_layer_set_masks_to_bounds(handle: *mut c_void, value: bool);
    pub fn ca_layer_get_shadow_offset(handle: *mut c_void, out_size: *mut c_void) -> bool;
    pub fn ca_layer_set_shadow_offset(handle: *mut c_void, width: f64, height: f64);
    pub fn ca_layer_get_shadow_radius(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_shadow_radius(handle: *mut c_void, value: f64);
    pub fn ca_layer_get_shadow_opacity(handle: *mut c_void) -> f32;
    pub fn ca_layer_set_shadow_opacity(handle: *mut c_void, value: f32);
    pub fn ca_layer_set_shadow_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_layer_get_shadow_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_layer_get_contents_gravity(handle: *mut c_void) -> i32;
    pub fn ca_layer_set_contents_gravity(handle: *mut c_void, gravity: i32);
    pub fn ca_layer_add_animation(
        handle: *mut c_void,
        animation_handle: *mut c_void,
        key: *const c_char,
    );
    pub fn ca_layer_remove_animation(handle: *mut c_void, key: *const c_char);

    pub fn ca_shape_layer_new() -> *mut c_void;
    pub fn ca_shape_layer_set_path(handle: *mut c_void, path_handle: *mut c_void);
    pub fn ca_shape_layer_get_path(handle: *mut c_void) -> *mut c_void;
    pub fn ca_shape_layer_set_fill_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_shape_layer_get_fill_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_shape_layer_set_stroke_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_shape_layer_get_stroke_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_shape_layer_get_line_width(handle: *mut c_void) -> f64;
    pub fn ca_shape_layer_set_line_width(handle: *mut c_void, value: f64);
    pub fn ca_shape_layer_get_line_cap(handle: *mut c_void) -> i32;
    pub fn ca_shape_layer_set_line_cap(handle: *mut c_void, value: i32);
    pub fn ca_shape_layer_get_line_join(handle: *mut c_void) -> i32;
    pub fn ca_shape_layer_set_line_join(handle: *mut c_void, value: i32);
    pub fn ca_shape_layer_set_line_dash_pattern(
        handle: *mut c_void,
        pattern: *const f64,
        length: usize,
    );
    pub fn ca_shape_layer_line_dash_pattern_count(handle: *mut c_void) -> usize;
    pub fn ca_shape_layer_line_dash_pattern_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_shape_layer_get_miter_limit(handle: *mut c_void) -> f64;
    pub fn ca_shape_layer_set_miter_limit(handle: *mut c_void, value: f64);

    pub fn ca_text_layer_new() -> *mut c_void;
    pub fn ca_text_layer_set_string(handle: *mut c_void, value: *const c_char);
    pub fn ca_text_layer_get_string(handle: *mut c_void) -> *mut c_char;
    pub fn ca_text_layer_set_font_name(handle: *mut c_void, value: *const c_char);
    pub fn ca_text_layer_get_font_name(handle: *mut c_void) -> *mut c_char;
    pub fn ca_text_layer_get_font_size(handle: *mut c_void) -> f64;
    pub fn ca_text_layer_set_font_size(handle: *mut c_void, value: f64);
    pub fn ca_text_layer_set_foreground_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_text_layer_get_foreground_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_text_layer_get_alignment_mode(handle: *mut c_void) -> i32;
    pub fn ca_text_layer_set_alignment_mode(handle: *mut c_void, value: i32);
    pub fn ca_text_layer_get_truncation_mode(handle: *mut c_void) -> i32;
    pub fn ca_text_layer_set_truncation_mode(handle: *mut c_void, value: i32);

    pub fn ca_gradient_layer_new() -> *mut c_void;
    pub fn ca_gradient_layer_set_colors(
        handle: *mut c_void,
        colors: *const *mut c_void,
        count: usize,
    );
    pub fn ca_gradient_layer_color_count(handle: *mut c_void) -> usize;
    pub fn ca_gradient_layer_color_at(handle: *mut c_void, index: usize) -> *mut c_void;
    pub fn ca_gradient_layer_set_locations(handle: *mut c_void, values: *const f64, count: usize);
    pub fn ca_gradient_layer_location_count(handle: *mut c_void) -> usize;
    pub fn ca_gradient_layer_location_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_gradient_layer_get_start_point(handle: *mut c_void, out_point: *mut c_void) -> bool;
    pub fn ca_gradient_layer_set_start_point(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_gradient_layer_get_end_point(handle: *mut c_void, out_point: *mut c_void) -> bool;
    pub fn ca_gradient_layer_set_end_point(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_gradient_layer_get_type(handle: *mut c_void) -> i32;
    pub fn ca_gradient_layer_set_type(handle: *mut c_void, value: i32);

    pub fn ca_emitter_layer_new() -> *mut c_void;
    pub fn ca_emitter_layer_set_emitter_cells(
        handle: *mut c_void,
        cells: *const *mut c_void,
        count: usize,
    );
    pub fn ca_emitter_layer_emitter_cell_count(handle: *mut c_void) -> usize;
    pub fn ca_emitter_layer_emitter_cell_at(handle: *mut c_void, index: usize) -> *mut c_void;
    pub fn ca_emitter_layer_get_birth_rate(handle: *mut c_void) -> f32;
    pub fn ca_emitter_layer_set_birth_rate(handle: *mut c_void, value: f32);
    pub fn ca_emitter_layer_get_lifetime(handle: *mut c_void) -> f32;
    pub fn ca_emitter_layer_set_lifetime(handle: *mut c_void, value: f32);
    pub fn ca_emitter_layer_get_emitter_position(
        handle: *mut c_void,
        out_point: *mut c_void,
    ) -> bool;
    pub fn ca_emitter_layer_set_emitter_position(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_emitter_layer_get_emitter_size(handle: *mut c_void, out_size: *mut c_void) -> bool;
    pub fn ca_emitter_layer_set_emitter_size(handle: *mut c_void, width: f64, height: f64);
    pub fn ca_emitter_layer_get_emitter_shape(handle: *mut c_void) -> i32;
    pub fn ca_emitter_layer_set_emitter_shape(handle: *mut c_void, value: i32);
    pub fn ca_emitter_layer_get_emitter_mode(handle: *mut c_void) -> i32;
    pub fn ca_emitter_layer_set_emitter_mode(handle: *mut c_void, value: i32);
    pub fn ca_emitter_layer_get_render_mode(handle: *mut c_void) -> i32;
    pub fn ca_emitter_layer_set_render_mode(handle: *mut c_void, value: i32);
    pub fn ca_emitter_layer_get_velocity(handle: *mut c_void) -> f32;
    pub fn ca_emitter_layer_set_velocity(handle: *mut c_void, value: f32);
    pub fn ca_emitter_layer_get_scale(handle: *mut c_void) -> f32;
    pub fn ca_emitter_layer_set_scale(handle: *mut c_void, value: f32);

    pub fn ca_emitter_cell_new() -> *mut c_void;
    pub fn ca_emitter_cell_set_name(handle: *mut c_void, value: *const c_char);
    pub fn ca_emitter_cell_get_name(handle: *mut c_void) -> *mut c_char;
    pub fn ca_emitter_cell_get_enabled(handle: *mut c_void) -> bool;
    pub fn ca_emitter_cell_set_enabled(handle: *mut c_void, value: bool);
    pub fn ca_emitter_cell_get_birth_rate(handle: *mut c_void) -> f32;
    pub fn ca_emitter_cell_set_birth_rate(handle: *mut c_void, value: f32);
    pub fn ca_emitter_cell_get_lifetime(handle: *mut c_void) -> f32;
    pub fn ca_emitter_cell_set_lifetime(handle: *mut c_void, value: f32);
    pub fn ca_emitter_cell_get_velocity(handle: *mut c_void) -> f64;
    pub fn ca_emitter_cell_set_velocity(handle: *mut c_void, value: f64);
    pub fn ca_emitter_cell_get_scale(handle: *mut c_void) -> f64;
    pub fn ca_emitter_cell_set_scale(handle: *mut c_void, value: f64);
    pub fn ca_emitter_cell_get_emission_range(handle: *mut c_void) -> f64;
    pub fn ca_emitter_cell_set_emission_range(handle: *mut c_void, value: f64);
    pub fn ca_emitter_cell_get_emission_longitude(handle: *mut c_void) -> f64;
    pub fn ca_emitter_cell_set_emission_longitude(handle: *mut c_void, value: f64);
    pub fn ca_emitter_cell_set_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_emitter_cell_get_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_emitter_cell_set_contents(handle: *mut c_void, image_handle: *mut c_void);
    pub fn ca_emitter_cell_get_contents(handle: *mut c_void) -> *mut c_void;
    pub fn ca_emitter_cell_get_alpha_speed(handle: *mut c_void) -> f32;
    pub fn ca_emitter_cell_set_alpha_speed(handle: *mut c_void, value: f32);

    pub fn ca_metal_layer_new() -> *mut c_void;
    pub fn ca_metal_layer_set_device(handle: *mut c_void, device_handle: *mut c_void);
    pub fn ca_metal_layer_get_pixel_format(handle: *mut c_void) -> usize;
    pub fn ca_metal_layer_set_pixel_format(handle: *mut c_void, pixel_format: usize);
    pub fn ca_metal_layer_get_drawable_size(handle: *mut c_void, out_size: *mut c_void) -> bool;
    pub fn ca_metal_layer_set_drawable_size(handle: *mut c_void, width: f64, height: f64);
    pub fn ca_metal_layer_next_drawable(handle: *mut c_void) -> *mut c_void;
    pub fn ca_metal_drawable_get_texture(handle: *mut c_void) -> *mut c_void;
    pub fn ca_metal_drawable_present(handle: *mut c_void);

    pub fn ca_animation_new() -> *mut c_void;
    pub fn ca_animation_get_duration(handle: *mut c_void) -> f64;
    pub fn ca_animation_set_duration(handle: *mut c_void, value: f64);
    pub fn ca_animation_get_repeat_count(handle: *mut c_void) -> f32;
    pub fn ca_animation_set_repeat_count(handle: *mut c_void, value: f32);
    pub fn ca_animation_get_autoreverses(handle: *mut c_void) -> bool;
    pub fn ca_animation_set_autoreverses(handle: *mut c_void, value: bool);
    pub fn ca_animation_get_removed_on_completion(handle: *mut c_void) -> bool;
    pub fn ca_animation_set_removed_on_completion(handle: *mut c_void, value: bool);

    pub fn ca_property_animation_new(key_path: *const c_char) -> *mut c_void;
    pub fn ca_basic_animation_new(key_path: *const c_char) -> *mut c_void;
    pub fn ca_property_animation_get_key_path(handle: *mut c_void) -> *mut c_char;
    pub fn ca_property_animation_set_key_path(handle: *mut c_void, value: *const c_char);
    pub fn ca_property_animation_get_value_function(handle: *mut c_void) -> *mut c_void;
    pub fn ca_property_animation_set_value_function(handle: *mut c_void, value_handle: *mut c_void);
    pub fn ca_basic_animation_set_from_number(handle: *mut c_void, value: f64);
    pub fn ca_basic_animation_set_to_number(handle: *mut c_void, value: f64);
    pub fn ca_basic_animation_set_by_number(handle: *mut c_void, value: f64);

    pub fn ca_keyframe_animation_new(key_path: *const c_char) -> *mut c_void;
    pub fn ca_keyframe_animation_set_values(handle: *mut c_void, values: *const f64, count: usize);
    pub fn ca_keyframe_animation_value_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_value_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_keyframe_animation_set_path(handle: *mut c_void, path_handle: *mut c_void);
    pub fn ca_keyframe_animation_get_path(handle: *mut c_void) -> *mut c_void;
    pub fn ca_keyframe_animation_set_key_times(
        handle: *mut c_void,
        values: *const f64,
        count: usize,
    );
    pub fn ca_keyframe_animation_key_time_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_key_time_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_keyframe_animation_get_calculation_mode(handle: *mut c_void) -> i32;
    pub fn ca_keyframe_animation_set_calculation_mode(handle: *mut c_void, value: i32);
    pub fn ca_keyframe_animation_get_rotation_mode(handle: *mut c_void) -> i32;
    pub fn ca_keyframe_animation_set_rotation_mode(handle: *mut c_void, value: i32);

    pub fn ca_spring_animation_new(key_path: *const c_char) -> *mut c_void;
    pub fn ca_spring_animation_get_mass(handle: *mut c_void) -> f64;
    pub fn ca_spring_animation_set_mass(handle: *mut c_void, value: f64);
    pub fn ca_spring_animation_get_stiffness(handle: *mut c_void) -> f64;
    pub fn ca_spring_animation_set_stiffness(handle: *mut c_void, value: f64);
    pub fn ca_spring_animation_get_damping(handle: *mut c_void) -> f64;
    pub fn ca_spring_animation_set_damping(handle: *mut c_void, value: f64);
    pub fn ca_spring_animation_get_initial_velocity(handle: *mut c_void) -> f64;
    pub fn ca_spring_animation_set_initial_velocity(handle: *mut c_void, value: f64);
    pub fn ca_spring_animation_get_settling_duration(handle: *mut c_void) -> f64;

    pub fn ca_animation_group_new() -> *mut c_void;
    pub fn ca_animation_group_set_animations(
        handle: *mut c_void,
        animations: *const *mut c_void,
        count: usize,
    );
    pub fn ca_animation_group_animation_count(handle: *mut c_void) -> usize;
    pub fn ca_animation_group_animation_at(handle: *mut c_void, index: usize) -> *mut c_void;

    pub fn ca_transition_new() -> *mut c_void;
    pub fn ca_transition_get_type(handle: *mut c_void) -> i32;
    pub fn ca_transition_set_type(handle: *mut c_void, value: i32);
    pub fn ca_transition_get_subtype(handle: *mut c_void) -> i32;
    pub fn ca_transition_set_subtype(handle: *mut c_void, value: i32);
    pub fn ca_transition_get_start_progress(handle: *mut c_void) -> f32;
    pub fn ca_transition_set_start_progress(handle: *mut c_void, value: f32);
    pub fn ca_transition_get_end_progress(handle: *mut c_void) -> f32;
    pub fn ca_transition_set_end_progress(handle: *mut c_void, value: f32);

    pub fn ca_renderer_new(texture_handle: *mut c_void, queue_handle: *mut c_void) -> *mut c_void;
    pub fn ca_renderer_set_layer(handle: *mut c_void, layer_handle: *mut c_void);
    pub fn ca_renderer_get_bounds(handle: *mut c_void, out_rect: *mut c_void) -> bool;
    pub fn ca_renderer_set_bounds(handle: *mut c_void, x: f64, y: f64, width: f64, height: f64);
    pub fn ca_renderer_begin_frame(handle: *mut c_void, time: f64, timestamp: *mut c_void);
    pub fn ca_renderer_update_bounds(handle: *mut c_void, out_rect: *mut c_void) -> bool;
    pub fn ca_renderer_render(handle: *mut c_void);
    pub fn ca_renderer_end_frame(handle: *mut c_void);
    pub fn ca_renderer_next_frame_time(handle: *mut c_void) -> f64;
    pub fn ca_renderer_set_destination(handle: *mut c_void, texture_handle: *mut c_void);
    pub fn ca_renderer_render_at_time(handle: *mut c_void, time: f64);
    pub fn ca_texture_copy_bytes(
        texture_handle: *mut c_void,
        out_bytes: *mut c_void,
        bytes_per_row: usize,
    ) -> bool;

    pub fn ca_transaction_begin();
    pub fn ca_transaction_commit();
    pub fn ca_transaction_flush();
    pub fn ca_transaction_get_animation_duration() -> f64;
    pub fn ca_transaction_set_animation_duration(value: f64);
    pub fn ca_transaction_get_disable_actions() -> bool;
    pub fn ca_transaction_set_disable_actions(value: bool);
    pub fn ca_transaction_set_completion_block(
        callback: TransactionCompletionCallback,
        context: *mut c_void,
    );
    pub fn ca_run_current_run_loop(seconds: f64);

    pub fn ca_layer_get_z_position(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_z_position(handle: *mut c_void, value: f64);
    pub fn ca_layer_get_anchor_point_z(handle: *mut c_void) -> f64;
    pub fn ca_layer_set_anchor_point_z(handle: *mut c_void, value: f64);
    pub fn ca_layer_get_sublayer_transform(handle: *mut c_void, out_transform: *mut c_void)
        -> bool;
    pub fn ca_layer_set_sublayer_transform(handle: *mut c_void, transform: *const c_void);
    pub fn ca_layer_get_double_sided(handle: *mut c_void) -> bool;
    pub fn ca_layer_set_double_sided(handle: *mut c_void, value: bool);
    pub fn ca_layer_get_geometry_flipped(handle: *mut c_void) -> bool;
    pub fn ca_layer_set_geometry_flipped(handle: *mut c_void, value: bool);
    pub fn ca_layer_supports_tone_map_mode() -> bool;
    pub fn ca_layer_get_tone_map_mode(handle: *mut c_void) -> i32;
    pub fn ca_layer_set_tone_map_mode(handle: *mut c_void, value: i32);

    pub fn ca_animation_get_timing_function(handle: *mut c_void) -> *mut c_void;
    pub fn ca_animation_set_timing_function(handle: *mut c_void, value_handle: *mut c_void);
    pub fn ca_animation_get_timing_function_name(handle: *mut c_void) -> i32;
    pub fn ca_animation_set_timing_function_name(handle: *mut c_void, value: i32);
    pub fn ca_animation_get_begin_time(handle: *mut c_void) -> f64;
    pub fn ca_animation_set_begin_time(handle: *mut c_void, value: f64);
    pub fn ca_animation_get_speed(handle: *mut c_void) -> f32;
    pub fn ca_animation_set_speed(handle: *mut c_void, value: f32);
    pub fn ca_animation_get_time_offset(handle: *mut c_void) -> f64;
    pub fn ca_animation_set_time_offset(handle: *mut c_void, value: f64);
    pub fn ca_animation_get_repeat_duration(handle: *mut c_void) -> f64;
    pub fn ca_animation_set_repeat_duration(handle: *mut c_void, value: f64);
    pub fn ca_animation_get_fill_mode(handle: *mut c_void) -> i32;
    pub fn ca_animation_set_fill_mode(handle: *mut c_void, value: i32);

    pub fn ca_basic_animation_get_from_number(handle: *mut c_void, out_value: *mut c_void) -> bool;
    pub fn ca_basic_animation_get_to_number(handle: *mut c_void, out_value: *mut c_void) -> bool;
    pub fn ca_basic_animation_get_by_number(handle: *mut c_void, out_value: *mut c_void) -> bool;
    pub fn ca_property_animation_get_additive(handle: *mut c_void) -> bool;
    pub fn ca_property_animation_set_additive(handle: *mut c_void, value: bool);
    pub fn ca_property_animation_get_cumulative(handle: *mut c_void) -> bool;
    pub fn ca_property_animation_set_cumulative(handle: *mut c_void, value: bool);
    pub fn ca_value_function_new(value: i32) -> *mut c_void;
    pub fn ca_value_function_get_name(handle: *mut c_void) -> i32;
    pub fn ca_timing_function_new_named(value: i32) -> *mut c_void;
    pub fn ca_timing_function_new_control_points(
        c1x: f32,
        c1y: f32,
        c2x: f32,
        c2y: f32,
    ) -> *mut c_void;
    pub fn ca_timing_function_get_name(handle: *mut c_void) -> i32;
    pub fn ca_timing_function_get_control_point(
        handle: *mut c_void,
        index: usize,
        out_values: *mut c_void,
    ) -> bool;

    pub fn ca_keyframe_animation_set_timing_function_names(
        handle: *mut c_void,
        values: *const i32,
        count: usize,
    );
    pub fn ca_keyframe_animation_timing_function_name_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_timing_function_name_at(handle: *mut c_void, index: usize) -> i32;
    pub fn ca_keyframe_animation_set_tension_values(
        handle: *mut c_void,
        values: *const f64,
        count: usize,
    );
    pub fn ca_keyframe_animation_tension_value_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_tension_value_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_keyframe_animation_set_continuity_values(
        handle: *mut c_void,
        values: *const f64,
        count: usize,
    );
    pub fn ca_keyframe_animation_continuity_value_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_continuity_value_at(handle: *mut c_void, index: usize) -> f64;
    pub fn ca_keyframe_animation_set_bias_values(
        handle: *mut c_void,
        values: *const f64,
        count: usize,
    );
    pub fn ca_keyframe_animation_bias_value_count(handle: *mut c_void) -> usize;
    pub fn ca_keyframe_animation_bias_value_at(handle: *mut c_void, index: usize) -> f64;

    pub fn ca_animation_group_append_animation(handle: *mut c_void, animation_handle: *mut c_void);
    pub fn ca_animation_group_clear_animations(handle: *mut c_void);
    pub fn ca_spring_animation_configure(
        handle: *mut c_void,
        mass: f64,
        stiffness: f64,
        damping: f64,
        initial_velocity: f64,
    );
    pub fn ca_transition_has_subtype(handle: *mut c_void) -> bool;
    pub fn ca_transition_clear_subtype(handle: *mut c_void);

    pub fn ca_transaction_lock();
    pub fn ca_transaction_unlock();
    pub fn ca_transaction_get_animation_timing_function() -> *mut c_void;
    pub fn ca_transaction_set_animation_timing_function(value_handle: *mut c_void);
    pub fn ca_transaction_get_animation_timing_function_name() -> i32;
    pub fn ca_transaction_set_animation_timing_function_name(value: i32);

    pub fn ca_quartz_display_link_new_main_screen() -> *mut c_void;
    pub fn ca_quartz_display_link_add_to_main_run_loop(handle: *mut c_void);
    pub fn ca_quartz_display_link_remove_from_main_run_loop(handle: *mut c_void);
    pub fn ca_quartz_display_link_invalidate(handle: *mut c_void);
    pub fn ca_quartz_display_link_is_paused(handle: *mut c_void) -> bool;
    pub fn ca_quartz_display_link_set_paused(handle: *mut c_void, paused: bool);
    pub fn ca_quartz_display_link_get_timestamp(handle: *mut c_void) -> f64;
    pub fn ca_quartz_display_link_get_duration(handle: *mut c_void) -> f64;
    pub fn ca_quartz_display_link_get_target_timestamp(handle: *mut c_void) -> f64;

    pub fn ca_metal_display_link_is_available() -> bool;
    pub fn ca_metal_display_link_new(layer_handle: *mut c_void) -> *mut c_void;
    pub fn ca_metal_display_link_add_to_current_run_loop(handle: *mut c_void);
    pub fn ca_metal_display_link_remove_from_current_run_loop(handle: *mut c_void);
    pub fn ca_metal_display_link_invalidate(handle: *mut c_void);
    pub fn ca_metal_display_link_is_paused(handle: *mut c_void) -> bool;
    pub fn ca_metal_display_link_set_paused(handle: *mut c_void, value: bool);
    pub fn ca_metal_display_link_get_preferred_frame_latency(handle: *mut c_void) -> f32;
    pub fn ca_metal_display_link_set_preferred_frame_latency(handle: *mut c_void, value: f32);
    pub fn ca_metal_display_link_set_delegate(
        handle: *mut c_void,
        callback: MetalDisplayLinkUpdateCallback,
        context: *mut c_void,
    );
    pub fn ca_metal_display_link_update_get_drawable(handle: *mut c_void) -> *mut c_void;
    pub fn ca_metal_display_link_update_get_target_timestamp(handle: *mut c_void) -> f64;
    pub fn ca_metal_display_link_update_get_target_presentation_timestamp(
        handle: *mut c_void,
    ) -> f64;

    pub fn ca_metal_layer_get_framebuffer_only(handle: *mut c_void) -> bool;
    pub fn ca_metal_layer_set_framebuffer_only(handle: *mut c_void, value: bool);
    pub fn ca_metal_layer_get_maximum_drawable_count(handle: *mut c_void) -> usize;
    pub fn ca_metal_layer_set_maximum_drawable_count(handle: *mut c_void, value: usize);
    pub fn ca_metal_layer_get_presents_with_transaction(handle: *mut c_void) -> bool;
    pub fn ca_metal_layer_set_presents_with_transaction(handle: *mut c_void, value: bool);
    pub fn ca_metal_layer_get_display_sync_enabled(handle: *mut c_void) -> bool;
    pub fn ca_metal_layer_set_display_sync_enabled(handle: *mut c_void, value: bool);
    pub fn ca_metal_layer_get_allows_next_drawable_timeout(handle: *mut c_void) -> bool;
    pub fn ca_metal_layer_set_allows_next_drawable_timeout(handle: *mut c_void, value: bool);

    pub fn ca_gradient_layer_get_color_components_at(
        handle: *mut c_void,
        index: usize,
        out_components: *mut c_void,
    ) -> bool;

    pub fn ca_text_layer_get_wrapped(handle: *mut c_void) -> bool;
    pub fn ca_text_layer_set_wrapped(handle: *mut c_void, value: bool);
    pub fn ca_text_layer_get_allows_font_subpixel_quantization(handle: *mut c_void) -> bool;
    pub fn ca_text_layer_set_allows_font_subpixel_quantization(handle: *mut c_void, value: bool);

    pub fn ca_shape_layer_get_fill_rule(handle: *mut c_void) -> i32;
    pub fn ca_shape_layer_set_fill_rule(handle: *mut c_void, value: i32);
    pub fn ca_shape_layer_get_stroke_start(handle: *mut c_void) -> f64;
    pub fn ca_shape_layer_set_stroke_start(handle: *mut c_void, value: f64);
    pub fn ca_shape_layer_get_stroke_end(handle: *mut c_void) -> f64;
    pub fn ca_shape_layer_set_stroke_end(handle: *mut c_void, value: f64);
    pub fn ca_shape_layer_get_line_dash_phase(handle: *mut c_void) -> f64;
    pub fn ca_shape_layer_set_line_dash_phase(handle: *mut c_void, value: f64);

    pub fn ca_transform_layer_new() -> *mut c_void;

    pub fn ca_replicator_layer_new() -> *mut c_void;
    pub fn ca_replicator_layer_get_instance_count(handle: *mut c_void) -> isize;
    pub fn ca_replicator_layer_set_instance_count(handle: *mut c_void, value: isize);
    pub fn ca_replicator_layer_get_preserves_depth(handle: *mut c_void) -> bool;
    pub fn ca_replicator_layer_set_preserves_depth(handle: *mut c_void, value: bool);
    pub fn ca_replicator_layer_get_instance_delay(handle: *mut c_void) -> f64;
    pub fn ca_replicator_layer_set_instance_delay(handle: *mut c_void, value: f64);
    pub fn ca_replicator_layer_get_instance_transform(
        handle: *mut c_void,
        out_transform: *mut c_void,
    ) -> bool;
    pub fn ca_replicator_layer_set_instance_transform(
        handle: *mut c_void,
        transform: *const c_void,
    );
    pub fn ca_replicator_layer_set_instance_color(handle: *mut c_void, color_handle: *mut c_void);
    pub fn ca_replicator_layer_get_instance_color(handle: *mut c_void) -> *mut c_void;
    pub fn ca_replicator_layer_get_instance_red_offset(handle: *mut c_void) -> f32;
    pub fn ca_replicator_layer_set_instance_red_offset(handle: *mut c_void, value: f32);
    pub fn ca_replicator_layer_get_instance_green_offset(handle: *mut c_void) -> f32;
    pub fn ca_replicator_layer_set_instance_green_offset(handle: *mut c_void, value: f32);
    pub fn ca_replicator_layer_get_instance_blue_offset(handle: *mut c_void) -> f32;
    pub fn ca_replicator_layer_set_instance_blue_offset(handle: *mut c_void, value: f32);
    pub fn ca_replicator_layer_get_instance_alpha_offset(handle: *mut c_void) -> f32;
    pub fn ca_replicator_layer_set_instance_alpha_offset(handle: *mut c_void, value: f32);

    pub fn ca_emitter_layer_get_emitter_z_position(handle: *mut c_void) -> f64;
    pub fn ca_emitter_layer_set_emitter_z_position(handle: *mut c_void, value: f64);
    pub fn ca_emitter_layer_get_emitter_depth(handle: *mut c_void) -> f64;
    pub fn ca_emitter_layer_set_emitter_depth(handle: *mut c_void, value: f64);
    pub fn ca_emitter_layer_get_preserves_depth(handle: *mut c_void) -> bool;
    pub fn ca_emitter_layer_set_preserves_depth(handle: *mut c_void, value: bool);
    pub fn ca_emitter_layer_get_spin(handle: *mut c_void) -> f32;
    pub fn ca_emitter_layer_set_spin(handle: *mut c_void, value: f32);
    pub fn ca_emitter_layer_get_seed(handle: *mut c_void) -> u32;
    pub fn ca_emitter_layer_set_seed(handle: *mut c_void, value: u32);

    pub fn ca_scroll_layer_new() -> *mut c_void;
    pub fn ca_scroll_layer_get_scroll_mode(handle: *mut c_void) -> i32;
    pub fn ca_scroll_layer_set_scroll_mode(handle: *mut c_void, value: i32);
    pub fn ca_scroll_layer_get_visible_rect(handle: *mut c_void, out_rect: *mut c_void) -> bool;
    pub fn ca_scroll_layer_scroll_to_point(handle: *mut c_void, x: f64, y: f64);
    pub fn ca_scroll_layer_scroll_to_rect(
        handle: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );

    pub fn ca_tiled_layer_new() -> *mut c_void;
    pub fn ca_tiled_layer_get_levels_of_detail(handle: *mut c_void) -> usize;
    pub fn ca_tiled_layer_set_levels_of_detail(handle: *mut c_void, value: usize);
    pub fn ca_tiled_layer_get_levels_of_detail_bias(handle: *mut c_void) -> usize;
    pub fn ca_tiled_layer_set_levels_of_detail_bias(handle: *mut c_void, value: usize);
    pub fn ca_tiled_layer_get_tile_size(handle: *mut c_void, out_size: *mut c_void) -> bool;
    pub fn ca_tiled_layer_set_tile_size(handle: *mut c_void, width: f64, height: f64);

    pub fn CVDisplayLinkCreateWithActiveCGDisplays(display_link_out: *mut *mut c_void) -> i32;
    pub fn CVDisplayLinkSetCurrentCGDisplay(display_link: *mut c_void, display_id: u32) -> i32;
    pub fn CVDisplayLinkGetCurrentCGDisplay(display_link: *mut c_void) -> u32;
    pub fn CVDisplayLinkSetOutputCallback(
        display_link: *mut c_void,
        callback: CVDisplayLinkOutputCallback,
        user_info: *mut c_void,
    ) -> i32;
    pub fn CVDisplayLinkStart(display_link: *mut c_void) -> i32;
    pub fn CVDisplayLinkStop(display_link: *mut c_void) -> i32;
    pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(display_link: *mut c_void) -> CVTime;
    pub fn CVDisplayLinkGetActualOutputVideoRefreshPeriod(display_link: *mut c_void) -> f64;
    pub fn CVDisplayLinkIsRunning(display_link: *mut c_void) -> bool;
    pub fn CVDisplayLinkGetCurrentTime(
        display_link: *mut c_void,
        out_time: *mut CVTimeStamp,
    ) -> i32;
    pub fn CVDisplayLinkTranslateTime(
        display_link: *mut c_void,
        in_time: *const CVTimeStamp,
        out_time: *mut CVTimeStamp,
    ) -> i32;
    pub fn CVDisplayLinkRetain(display_link: *mut c_void) -> *mut c_void;
    pub fn CVDisplayLinkRelease(display_link: *mut c_void);
}
