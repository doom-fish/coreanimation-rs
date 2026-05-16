# Coverage

`coreanimation-rs` v0.2.1 keeps the original broad modules (`animation`, `layer`, `display_link`, `emitter`, `renderer`, and `transaction`) and layers additive `ca_*` extensions on top. The goal is a safe, tested QuartzCore subset for macOS rather than exhaustive header parity.

Legend:

- ✅ implemented and exercised by at least one example and one integration test
- 🟡 partial subset exposed
- ⏭️ not wrapped yet

## Requested v0.2.0 logical areas

| Area | Status | Current Rust surface | Evidence | Notes |
| --- | --- | --- | --- | --- |
| `CALayer` | ✅ | `z_position`, `anchor_point_z`, `sublayer_transform`, `double_sided`, `geometry_flipped` | `examples/02_ca_layer_roundtrip.rs`, `tests/ca_layer_tests.rs` | Additive methods on the existing `Layer` type |
| `CAAnimation` | ✅ | `timing_function_name`, `set_timing_function_name` | `examples/03_ca_animation_timing_function.rs`, `tests/ca_animation_tests.rs` | Uses the new `TimingFunctionName` enum |
| `CAKeyframeAnimation` | ✅ | `timing_function_names`, `tension_values`, `continuity_values`, `bias_values` | `examples/04_ca_keyframe_animation_arrays.rs`, `tests/ca_keyframe_animation_tests.rs` | Extends the existing `KeyframeAnimation` wrapper |
| `CABasicAnimation` | ✅ | Numeric `from` / `to` / `by` helpers plus `additive` and `cumulative` | `examples/05_ca_basic_animation_values.rs`, `tests/ca_basic_animation_tests.rs` | Focused on number-valued animations |
| `CAAnimationGroup` | ✅ | `push`, `clear`, `len`, `is_empty` | `examples/06_ca_animation_group_collection.rs`, `tests/ca_animation_group_tests.rs` | Preserves the original `AnimationGroup` type |
| `CASpringAnimation` | ✅ | `configure` convenience plus existing spring accessors | `examples/07_ca_spring_animation_configure.rs`, `tests/ca_spring_animation_tests.rs` | Useful configuration helper on top of the existing wrapper |
| `CATransition` | ✅ | `has_subtype`, `clear_subtype` | `examples/08_ca_transition_subtype.rs`, `tests/ca_transition_tests.rs` | Complements the existing transition type/subtype APIs |
| `CAMediaTiming` | ✅ | `begin_time`, `speed`, `time_offset`, `repeat_duration`, `fill_mode` | `examples/09_ca_media_timing_roundtrip.rs`, `tests/ca_media_timing_tests.rs` | Uses the new `MediaTimingFillMode` enum |
| `CATransaction` | ✅ | `lock`, `unlock`, `lock_guard`, animation timing-function helpers | `examples/10_ca_transaction_defaults.rs`, `tests/ca_transaction_tests.rs` | Adds `TransactionLockGuard` without changing the existing transaction API |
| `CADisplayLink` | ✅ | `QuartzDisplayLink::new_main_screen`, run-loop add/remove, `invalidate`, paused state, timestamps | `examples/11_ca_display_link_smoke.rs`, `tests/ca_display_link_tests.rs` | Headless-safe and returns `None` when unavailable |
| `CAMetalLayer` | ✅ | `framebuffer_only`, `maximum_drawable_count`, `presents_with_transaction`, `display_sync_enabled`, `allows_next_drawable_timeout` | `examples/12_ca_metal_layer_properties.rs`, `tests/ca_metal_layer_tests.rs` | Extends the existing `MetalLayer` type |
| `CAGradientLayer` | ✅ | `color_components_at` | `examples/13_ca_gradient_layer_colors.rs`, `tests/ca_gradient_layer_tests.rs` | Works alongside the existing gradient color/location setters |
| `CATextLayer` | ✅ | `wrapped`, `allows_font_subpixel_quantization` | `examples/14_ca_text_layer_flags.rs`, `tests/ca_text_layer_tests.rs` | Additive methods on the existing `TextLayer` type |
| `CAShapeLayer` | ✅ | `fill_rule`, `stroke_start`, `stroke_end`, `line_dash_phase` | `examples/15_ca_shape_layer_stroke.rs`, `tests/ca_shape_layer_tests.rs` | Adds the `ShapeFillRule` enum |
| `CATransformLayer` | ✅ | `TransformLayer::new` | `examples/16_ca_transform_layer_smoke.rs`, `tests/ca_transform_layer_tests.rs` | New wrapper type that derefs to `Layer` |
| `CAReplicatorLayer` | ✅ | `instance_count`, `preserves_depth`, `instance_delay`, `instance_transform`, `instance_color`, RGBA offsets | `examples/17_ca_replicator_layer_properties.rs`, `tests/ca_replicator_layer_tests.rs` | New wrapper type that derefs to `Layer` |
| `CAEmitterLayer` | ✅ | `emitter_z_position`, `emitter_depth`, `preserves_depth`, `spin`, `seed` | `examples/18_ca_emitter_layer_properties.rs`, `tests/ca_emitter_layer_tests.rs` | Extends the existing `EmitterLayer` type |
| `CAScrollLayer` | ✅ | `new`, `scroll_mode`, `visible_rect`, `scroll_to_point`, `scroll_to_rect` | `examples/19_ca_scroll_layer_scroll_mode.rs`, `tests/ca_scroll_layer_tests.rs` | New wrapper type that derefs to `Layer` |
| `CATiledLayer` | ✅ | `new`, `levels_of_detail`, `levels_of_detail_bias`, `tile_size` | `examples/20_ca_tiled_layer_properties.rs`, `tests/ca_tiled_layer_tests.rs` | New wrapper type that derefs to `Layer` |

## Additive v0.2.1 logical areas

| Area | Status | Current Rust surface | Evidence | Notes |
| --- | --- | --- | --- | --- |
| `CAPropertyAnimation` / `CAValueFunction` | ✅ | `PropertyAnimation`, `ValueFunction`, `ValueFunctionName`, additive/cumulative/value-function helpers | `examples/21_ca_property_animation_value_function.rs`, `tests/ca_property_animation_tests.rs` | Adds first-class property-animation and value-function wrappers |
| `CAMediaTimingFunction` | ✅ | `TimingFunction`, `Animation::timing_function`, `Transaction::animation_timing_function` | `examples/22_ca_timing_function_objects.rs`, `tests/ca_timing_function_tests.rs` | Supports named and custom cubic Bezier timing functions |
| `CALayer` / `CAMetalLayer` tone mapping | ✅ | `ToneMapMode`, `Layer::supports_tone_map_mode`, `tone_map_mode`, `set_tone_map_mode` | `examples/23_ca_tone_map_mode.rs`, `tests/ca_tone_map_mode_tests.rs` | `MetalLayer` inherits the new accessors via `Deref<Target = Layer>` |
| `CAMetalDisplayLink` | ✅ | `MetalDisplayLink`, `MetalDisplayLinkUpdate`, delegate closure bridge, current-run-loop helper | `examples/24_ca_metal_display_link.rs`, `tests/ca_metal_display_link_tests.rs` | Offscreen-safe with a plain `CAMetalLayer` |

## Existing v0.1.0 surface retained

- `CARenderer` offscreen rendering remains available and is still covered by `examples/01_layer_render_smoke.rs`.
- `DisplayLink` continues to wrap `CVDisplayLink`; `QuartzDisplayLink` and `MetalDisplayLink` are additive rather than breaking replacements.
- Existing `Color`, `Path`, `EmitterCell`, `GradientLayer`, `TextLayer`, `ShapeLayer`, `MetalLayer`, and transaction completion helpers remain part of the public API.

## Known partial and skipped QuartzCore areas

| Area | Status | Current limitation |
| --- | --- | --- |
| `CALayer` deep surface | 🟡 | No delegate/action callback model, filter wrappers, layout/constraint APIs, or newer dynamic-range properties yet |
| `CAPropertyAnimation` / object-valued `CABasicAnimation` | 🟡 | The crate now exposes the first-class property-animation/value-function surface, but higher-level typed key-path helpers and non-number `CABasicAnimation` values are still missing |
| `CAKeyframeAnimation` advanced surface | 🟡 | The crate exposes the requested timing-function and T/C/B arrays, but not every QuartzCore path / rotation / interpolation feature |
| `CADisplayLink` advanced configuration | 🟡 | The current wrapper focuses on lifecycle/state and timestamps; construction is main-screen/main-thread and `macOS 14+` only |
| `CAMetalDisplayLink` advanced configuration | 🟡 | Delegate callbacks, paused state, and frame latency are wrapped, but `preferredFrameRateRange` still depends on the unwrapped `CAFrameRateRange` value type |
| `CAMetalLayer` HDR / EDR / colorspace / residency / developer-HUD APIs | 🟡 | `toneMapMode` is wrapped, but preferred dynamic range, colorspace, residency, and developer-HUD APIs remain unwrapped |
| `CATextLayer` attributed string / font management and `CAShapeLayer` full dash-pattern editing | 🟡 | Only a focused slice is exposed here; older content APIs remain unchanged |
| `CAEmitterLayer` / `CAEmitterCell` advanced particle tuning | 🟡 | Existing particle support is useful but still far from complete QuartzCore coverage |
