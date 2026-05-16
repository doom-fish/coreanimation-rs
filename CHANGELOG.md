# Changelog

## [0.2.0] - 2026-05-16

- Preserved the existing v0.1.0 public API and added additive `ca_*` extension modules plus Swift bridge files for `CALayer`, `CAAnimation`, `CAKeyframeAnimation`, `CABasicAnimation`, `CAAnimationGroup`, `CASpringAnimation`, `CATransition`, `CAMediaTiming`, `CATransaction`, `CADisplayLink`, `CAMetalLayer`, `CAGradientLayer`, `CATextLayer`, `CAShapeLayer`, `CATransformLayer`, `CAReplicatorLayer`, `CAEmitterLayer`, `CAScrollLayer`, and `CATiledLayer`
- Added new root exports including `QuartzDisplayLink`, `TimingFunctionName`, `MediaTimingFillMode`, `ReplicatorLayer`, `ScrollLayer`, `ScrollMode`, `ShapeFillRule`, `TiledLayer`, `TransactionLockGuard`, and `TransformLayer`
- Added 19 focused examples and 19 integration tests so each requested logical area now has at least one runnable example and one test
- Linked AppKit for the new `CADisplayLink` bridge and documented the current implementation matrix in `COVERAGE.md`

## [0.1.0] - 2026-05-16

- Initial release of `coreanimation-rs`
- Added layer, animation, transaction, `CVDisplayLink`, and offscreen `CARenderer` bindings
- Added a Metal-backed smoke example that verifies layer rendering end to end
