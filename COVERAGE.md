# Coverage

`coreanimation-rs` v0.2.2 keeps the original broad modules (`animation`, `layer`, `display_link`, `emitter`, `renderer`, and `transaction`) and layers additive `ca_*` extensions on top. The current release closes the full non-exempt QuartzCore `CA*.h` audit surface from `MacOSX26.2.sdk` while keeping the earlier API stable.

Legend:

- ✅ implemented and exercised by at least one example and one integration test
- 🟡 broader ergonomic surface still available for future work
- ⏭️ intentionally skipped

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

## Additive v0.2.2 logical areas

| Area | Status | Current Rust surface | Evidence | Notes |
| --- | --- | --- | --- | --- |
| `CAAction` / `CAAnimationDelegate` / `CAFrameRateRange` / `CACurrentMediaTime` | ✅ | `Action`, `ActionLike`, `AnimationDelegate`, `FrameRateRange`, `current_media_time`, `Animation::{set_delegate, preferred_frame_rate_range}`, `MetalDisplayLink::preferred_frame_rate_range` | `examples/25_ca_animation_delegate.rs`, `tests/ca_animation_delegate_tests.rs`, `src/ca_animation_delegate.rs` | Adds explicit frame-rate default/make/equality helpers and generic action handles |
| `CALayer` advanced surface / constraints / layout managers | ✅ | `AutoresizingMask`, `EdgeAntialiasingMask`, `CornerMask`, `ContentsFormat`, `ContentsFilter`, `CornerCurve`, `DynamicRange`, `LayerActionKeys`, `LayerDelegate`, `Constraint`, `ConstraintLayoutManager`, `LayoutManager`, plus additive `Layer` APIs | `examples/26_ca_layer_advanced.rs`, `tests/ca_layer_advanced_tests.rs` | Non-breaking additive layer, action-dictionary, constraint, and delegate APIs |
| `CAEDRMetadata` / `CAMetalLayer` HDR surface | ✅ | `EDRMetadata`, `MetalLayer::{colorspace, set_colorspace, edr_metadata, set_edr_metadata}`, inherited `preferred_dynamic_range` | `examples/27_ca_edr_metadata.rs`, `tests/ca_edr_metadata_tests.rs` | Covers HDR metadata creation plus colorspace / dynamic-range configuration |
| `CARemoteLayerClient` / `CARemoteLayerServer` / `CARenderer` color space | ✅ | `RemoteLayerClient`, `RemoteLayerServer`, `Renderer::new_with_color_space`, `CGColorSpace` | `examples/28_ca_remote_layer_renderer.rs`, `tests/ca_remote_layer_renderer_tests.rs` | Keeps the existing renderer constructor intact while adding the color-space option |
| `CATransform3D` advanced helpers | ✅ | `Transform3D::{rotation, translated, scaled, rotated, concat, inverted, from_affine, is_affine, to_affine}`, `CGAffineTransform` | `examples/29_ca_transform3d.rs`, `tests/ca_transform3d_tests.rs` | Completes the remaining audited transform helper surface |

## Audit status

- `COVERAGE_AUDIT.md` now reports `193 / 193` non-exempt QuartzCore `CA*.h` symbols wrapped.
- Remaining audit gaps: `0`.
- `CAOpenGLLayer` remains intentionally exempt because it is deprecated on macOS.

## Existing v0.1.0 surface retained

- `CARenderer` offscreen rendering remains available and is still covered by `examples/01_layer_render_smoke.rs`.
- `DisplayLink` continues to wrap `CVDisplayLink`; `QuartzDisplayLink` and `MetalDisplayLink` are additive rather than breaking replacements.
- Existing `Color`, `Path`, `EmitterCell`, `GradientLayer`, `TextLayer`, `ShapeLayer`, `MetalLayer`, and transaction completion helpers remain part of the public API.

## Known partial and skipped QuartzCore areas

| Area | Status | Current limitation |
| --- | --- | --- |
| `CAPropertyAnimation` / object-valued `CABasicAnimation` | 🟡 | Higher-level typed key-path helpers and non-number `CABasicAnimation` values remain future work |
| `CAKeyframeAnimation` advanced authoring helpers | 🟡 | The audited QuartzCore symbols are covered, but not every path/interpolation convenience is wrapped |
| `CADisplayLink` / `CAMetalDisplayLink` scheduling ergonomics | 🟡 | Lifecycle, delegates, timestamps, latency, and frame-rate range are wrapped, but construction remains focused on main/current-run-loop flows |
| `CATextLayer` attributed string / font management and `CAShapeLayer` full dash editing | 🟡 | A focused slice is exposed; richer content-editing APIs remain future work |
| `CAEmitterLayer` / `CAEmitterCell` advanced particle tuning | 🟡 | Existing particle support is useful but not exhaustive |
| `CAOpenGLLayer` | ⏭️ | Deprecated on macOS and intentionally exempt from the audit |
