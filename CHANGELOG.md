# Changelog

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
