# Changelog

All notable changes to `coreanimation-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-09-24

### Security

- `read_texture_bytes` sized its buffer as 4 bytes per pixel for every
  texture, so RGBA16Float, RGBA32Float and BGRA10_XR textures overflowed the
  `Vec` from safe code. Rows are now sized from `apple_metal::bytes_per_pixel`;
  compressed, depth, stencil and unknown formats, textures that are not plain
  2D, private or memoryless storage and framebuffer-only textures are refused,
  and the bridge checks sizes before calling `getBytes`.
- `DisplayLink` output callbacks and `Transaction` completion blocks took a raw
  function pointer and context through safe functions with nothing keeping the
  context alive, and a panicking callback unwound into the framework. Both now
  take owned closures in a reference-counted context (see Changed).
- `AnimationDelegate` callbacks run on the main thread even when the animation
  was added on another thread, but the closures were not `Send`, dropping the
  delegate freed them while a callback could be running, and the bridge's
  callback slots were written and read from different threads without
  synchronization.
- The `CAMetalDisplayLink` delegate context was freed while an update callback
  could still be running; it is now reference counted, and the Swift delegate
  holds a reference until it is deallocated. Panics in delegate callbacks are
  contained.
- Cloning a `MetalDrawable` copied the pointer without a retain, so dropping
  both handles over-released the drawable (a crash from safe code).
- `Layer::contents`, `EmitterCell::contents` and the gradient color accessors
  returned whatever object the property held, such as the private backing
  image of a displayed layer, typed as a `CGImage` or `CGColor`. The bridge now
  checks the Core Foundation type ID.
- `LayerLike`, `AnimationLike` and `ActionLike` were safe public traits whose
  raw pointers the bridge retained and dereferenced; they are now sealed. The
  layer delegate's action lookup leaked any returned handle that was not a
  `CAAction`.

### Fixed

- Safe calls no longer raise uncatchable Objective-C exceptions that abort the
  process: `set_maximum_drawable_count` outside 2 and 3; `set_pixel_format`
  with a format `CAMetalLayer` rejects (now caught in an Objective-C `@try`
  and reported); NaN frames, bounds, positions and scroll targets; sublayer
  cycles; and constraints with non-finite scale or offset. NaN anchor points
  and transforms, which made the next frame change throw, are refused.
- `Layer::set_mask` refuses masks that already have a superlayer (undefined
  behavior per `CALayer.h`) or that are the layer or one of its ancestors,
  which silently made the tree cyclic.
- The README described v0.2.2 and claimed zero coverage gaps. It now gives the
  macOS 11 minimum, the APIs that need newer systems, installation and
  threading rules; COVERAGE files say they count top-level symbols only and
  list known member gaps.

### Changed

- **Breaking:** `DisplayLink::set_output_handler` takes an
  `FnMut(&CVTimeStamp, &CVTimeStamp) + Send + 'static` closure; the raw form is
  `unsafe fn set_output_callback`. Replacing or clearing the handler of a
  running link stops it first (clearing leaves it stopped), and dropping the
  last `DisplayLink` stops the link and removes its callback before releasing
  it. Clones share one handle. `as_ptr` is no longer `const`.
- **Breaking:** `Transaction::set_completion_handler` takes an
  `FnOnce() + Send + 'static` closure; the raw form is
  `unsafe fn set_completion_block`.
- **Breaking:** `AnimationDelegate::set_did_start` and `set_did_stop` require
  `Send` closures.
- **Breaking:** `MetalLayer::set_maximum_drawable_count`, `set_pixel_format`,
  `Layer::set_frame`, `set_bounds`, `set_position`, `set_anchor_point`,
  `set_transform`, `add_sublayer`, `set_mask` and `ScrollLayer::scroll_to_point`
  / `scroll_to_rect` return `Result<(), CoreAnimationError>`.
- **Breaking:** `LayerLike`, `AnimationLike` and `ActionLike` are sealed.
- **Breaking:** `read_texture_bytes` returns rows of `width * bytes_per_pixel`
  bytes for the texture's format and fails for textures it cannot read.
- `Constraint::new` returns `None` for non-finite scale or offset.
- Requires `apple-cf >=0.11, <0.12`, `apple-metal >=0.10, <0.11` and
  `doom-fish-utils >=0.4.1, <0.5`, all as path dependencies; `rust-version` is
  1.82 (was 1.76). The crate now links Core Image.

### Added

- `DisplayLink::clear_output_handler`; `CVDisplayLinkOutputCallback` is
  re-exported from `display_link`.
- `MetalLayer::wants_extended_dynamic_range_content` and its setter,
  `preferred_device`, `supports_developer_hud_properties`,
  `developer_hud_properties` and `set_developer_hud_properties` (macOS 13).
- `SpringAnimation::with_perceptual_duration`,
  `supports_perceptual_parameters`, `perceptual_duration`, `bounce`,
  `allows_overdamping` and `set_allows_overdamping` (macOS 14).
- `Transition::filter_name` and `set_filter_name` for Core Image transition
  filters.
- `Renderer::add_update_rect`.

### Removed

- The `ca_animation_delegate_set_did_start_callback` and
  `ca_animation_delegate_set_did_stop_callback` raw exports.

## [0.5.2] - 2026-05-20

- Added in-`src/` unit tests across animation, ca_frame_rate_range, error, layer, and transform (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.5.1] - 2026-05-18

- Added one-line rustdoc coverage across the core layer, animation, display-link, emitter, renderer, transaction, constraint, path, and transform surfaces, lifting measured public-item coverage above 80%.

## [0.5.0] - 2026-05-18

### Breaking

- Re-exported `CVDisplayLinkOutputCallback`, `CVSMPTETime`, `CVTime`, `CVTimeStamp`, and `CVReturn` from `apple-cf` instead of maintaining duplicate local definitions.
- Enabled the `cv` feature on `apple-cf` so the canonical Core Video raw types back the display-link API surface.

## [0.4.2] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.4.1] - 2026-05-18

- Widen apple-metal version bound so the 0.x bump dep resolves. No source changes.

## [0.4.0] - 2026-05-18

### Breaking

- Bumped `apple-cf` support to `>=0.7, <0.9` so the crate resolves `apple-cf` 0.8.x, including the canonical nested `CGRect { origin, size }` layout.
- Migrated all `CGRect` field access sites in `src/renderer.rs`, `src/ca_scroll_layer.rs`, `src/path.rs`, and `src/layer.rs` from flat fields (`rect.x`, `rect.y`, `rect.width`, `rect.height`) to nested accessors (`rect.origin.x`, `rect.origin.y`, `rect.size.width`, `rect.size.height`).
- Bumped crate version from `0.3.0` to `0.4.0` for the dependency-driven breaking change.

## [0.3.0] - 2026-05-18

### Breaking

- `pub mod ffi` is now feature-gated behind the new `raw-ffi` Cargo feature.
  By default, raw FFI symbols are no longer reachable through
  `coreanimation::ffi::*`. Users who need raw FFI access must opt in with
  `features = ["raw-ffi"]`. The `CVSMPTETime`, `CVTime`, `CVTimeStamp`, and
  `CVReturn` types remain available at the crate root and via
  `coreanimation::display_link::*` regardless of feature configuration.
- Dropped the dead `cargo-clippy = []` feature (vestigial; never used).

## [0.2.2] - 2026-05-17

- Added additive wrappers for `Action`, `AnimationDelegate`, `FrameRateRange`, `current_media_time`, `Constraint`, `ConstraintLayoutManager`, `LayoutManager`, `EDRMetadata`, `RemoteLayerClient`, and `RemoteLayerServer`
- Extended `Layer`, `MetalLayer`, `MetalDisplayLink`, `Renderer`, and `Transform3D` with the remaining audited QuartzCore/Core Animation surface, including action dictionaries, advanced layer enums, HDR/EDR metadata, renderer color-space options, and affine/concat/invert transform helpers
- Added 5 focused examples and 5 integration tests covering the new wrappers, plus callback-focused unit coverage for `CAAnimationDelegate`
- Refreshed `README.md`, `COVERAGE.md`, and `COVERAGE_AUDIT.md` to reflect full non-exempt `CA*.h` audit coverage (0 gaps, 1 deprecated exemption)

## [0.2.1] - 2026-05-16

- Added `PropertyAnimation`, `ValueFunction`, and `ValueFunctionName`, plus additive/cumulative/value-function helpers across the property-animation wrappers
- Added `TimingFunction` object wrappers for named and custom cubic Bezier functions plus object-based `Animation` / `Transaction` timing-function accessors
- Added `ToneMapMode` accessors for `Layer` / `MetalLayer` and a callback-driven `MetalDisplayLink` / `MetalDisplayLinkUpdate` bridge
- Added 4 focused examples and 4 integration tests covering the new wrappers and refreshed the coverage docs/audit

## [0.2.0] - 2026-05-16

- Preserved the existing v0.1.0 public API and added additive `ca_*` extension modules plus Swift bridge files for `CALayer`, `CAAnimation`, `CAKeyframeAnimation`, `CABasicAnimation`, `CAAnimationGroup`, `CASpringAnimation`, `CATransition`, `CAMediaTiming`, `CATransaction`, `CADisplayLink`, `CAMetalLayer`, `CAGradientLayer`, `CATextLayer`, `CAShapeLayer`, `CATransformLayer`, `CAReplicatorLayer`, `CAEmitterLayer`, `CAScrollLayer`, and `CATiledLayer`
- Added new root exports including `QuartzDisplayLink`, `TimingFunctionName`, `MediaTimingFillMode`, `ReplicatorLayer`, `ScrollLayer`, `ScrollMode`, `ShapeFillRule`, `TiledLayer`, `TransactionLockGuard`, and `TransformLayer`
- Added 19 focused examples and 19 integration tests so each requested logical area now has at least one runnable example and one test
- Linked AppKit for the new `CADisplayLink` bridge and documented the current implementation matrix in `COVERAGE.md`

## [0.1.0] - 2026-05-16

- Initial release of `coreanimation-rs`
- Added layer, animation, transaction, `CVDisplayLink`, and offscreen `CARenderer` bindings
- Added a Metal-backed smoke example that verifies layer rendering end to end
