# coreanimation-rs coverage audit (vs MacOSX26.2.sdk)

- Scope: `QuartzCore.framework` `CA*.h` CoreAnimation headers only (not the bundled CoreImage/CoreVideo umbrella headers).
- Filtered out macOS-unavailable surface: `CAEAGLLayer` (`API_UNAVAILABLE(macos, ...)`).
- Deprecated macOS symbol kept as EXEMPT: `CAOpenGLLayer`.

SDK_PUBLIC_SYMBOLS: 194
VERIFIED: 125
GAPS: 68
EXEMPT: 1
COVERAGE_PCT: 64.8%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CAAnimation` | interface | `CAAnimation.h` | `Animation` |
| `CAAnimationCalculationMode` | typealias | `CAAnimation.h` | `AnimationCalculationMode` |
| `CAAnimationGroup` | interface | `CAAnimation.h` | `AnimationGroup` |
| `CAAnimationRotationMode` | typealias | `CAAnimation.h` | `RotationMode` |
| `CABasicAnimation` | interface | `CAAnimation.h` | `BasicAnimation` |
| `CAKeyframeAnimation` | interface | `CAAnimation.h` | `KeyframeAnimation` |
| `CASpringAnimation` | interface | `CAAnimation.h` | `SpringAnimation` |
| `CATransition` | interface | `CAAnimation.h` | `Transition` |
| `CATransitionSubtype` | typealias | `CAAnimation.h` | `TransitionSubtype` |
| `CATransitionType` | typealias | `CAAnimation.h` | `TransitionType` |
| `kCAAnimationCubic` | constant | `CAAnimation.h` | `AnimationCalculationMode::Cubic` |
| `kCAAnimationCubicPaced` | constant | `CAAnimation.h` | `AnimationCalculationMode::CubicPaced` |
| `kCAAnimationDiscrete` | constant | `CAAnimation.h` | `AnimationCalculationMode::Discrete` |
| `kCAAnimationLinear` | constant | `CAAnimation.h` | `AnimationCalculationMode::Linear` |
| `kCAAnimationPaced` | constant | `CAAnimation.h` | `AnimationCalculationMode::Paced` |
| `kCAAnimationRotateAuto` | constant | `CAAnimation.h` | `RotationMode::Auto` |
| `kCAAnimationRotateAutoReverse` | constant | `CAAnimation.h` | `RotationMode::AutoReverse` |
| `kCATransitionFade` | constant | `CAAnimation.h` | `TransitionType::Fade` |
| `kCATransitionFromBottom` | constant | `CAAnimation.h` | `TransitionSubtype::FromBottom` |
| `kCATransitionFromLeft` | constant | `CAAnimation.h` | `TransitionSubtype::FromLeft` |
| `kCATransitionFromRight` | constant | `CAAnimation.h` | `TransitionSubtype::FromRight` |
| `kCATransitionFromTop` | constant | `CAAnimation.h` | `TransitionSubtype::FromTop` |
| `kCATransitionMoveIn` | constant | `CAAnimation.h` | `TransitionType::MoveIn` |
| `kCATransitionPush` | constant | `CAAnimation.h` | `TransitionType::Push` |
| `kCATransitionReveal` | constant | `CAAnimation.h` | `TransitionType::Reveal` |
| `CADisplayLink` | interface | `CADisplayLink.h` | `QuartzDisplayLink` |
| `CAEmitterCell` | interface | `CAEmitterCell.h` | `EmitterCell` |
| `CAEmitterLayer` | interface | `CAEmitterLayer.h` | `EmitterLayer` |
| `CAEmitterLayerEmitterMode` | typealias | `CAEmitterLayer.h` | `EmitterMode` |
| `CAEmitterLayerEmitterShape` | typealias | `CAEmitterLayer.h` | `EmitterShape` |
| `CAEmitterLayerRenderMode` | typealias | `CAEmitterLayer.h` | `EmitterRenderMode` |
| `kCAEmitterLayerAdditive` | constant | `CAEmitterLayer.h` | `EmitterRenderMode::Additive` |
| `kCAEmitterLayerBackToFront` | constant | `CAEmitterLayer.h` | `EmitterRenderMode::BackToFront` |
| `kCAEmitterLayerCircle` | constant | `CAEmitterLayer.h` | `EmitterShape::Circle` |
| `kCAEmitterLayerCuboid` | constant | `CAEmitterLayer.h` | `EmitterShape::Cuboid` |
| `kCAEmitterLayerLine` | constant | `CAEmitterLayer.h` | `EmitterShape::Line` |
| `kCAEmitterLayerOldestFirst` | constant | `CAEmitterLayer.h` | `EmitterRenderMode::OldestFirst` |
| `kCAEmitterLayerOldestLast` | constant | `CAEmitterLayer.h` | `EmitterRenderMode::OldestLast` |
| `kCAEmitterLayerOutline` | constant | `CAEmitterLayer.h` | `EmitterMode::Outline` |
| `kCAEmitterLayerPoint` | constant | `CAEmitterLayer.h` | `EmitterShape::Point` |
| `kCAEmitterLayerPoints` | constant | `CAEmitterLayer.h` | `EmitterMode::Points` |
| `kCAEmitterLayerRectangle` | constant | `CAEmitterLayer.h` | `EmitterShape::Rectangle` |
| `kCAEmitterLayerSphere` | constant | `CAEmitterLayer.h` | `EmitterShape::Sphere` |
| `kCAEmitterLayerSurface` | constant | `CAEmitterLayer.h` | `EmitterMode::Surface` |
| `kCAEmitterLayerUnordered` | constant | `CAEmitterLayer.h` | `EmitterRenderMode::Unordered` |
| `kCAEmitterLayerVolume` | constant | `CAEmitterLayer.h` | `EmitterMode::Volume` |
| `CAGradientLayer` | interface | `CAGradientLayer.h` | `GradientLayer` |
| `CAGradientLayerType` | typealias | `CAGradientLayer.h` | `GradientType` |
| `kCAGradientLayerAxial` | constant | `CAGradientLayer.h` | `GradientType::Axial` |
| `kCAGradientLayerConic` | constant | `CAGradientLayer.h` | `GradientType::Conic` |
| `kCAGradientLayerRadial` | constant | `CAGradientLayer.h` | `GradientType::Radial` |
| `CALayer` | interface | `CALayer.h` | `Layer` |
| `CALayerContentsGravity` | typealias | `CALayer.h` | `ContentsGravity` |
| `kCAGravityBottom` | constant | `CALayer.h` | `ContentsGravity::Bottom` |
| `kCAGravityBottomLeft` | constant | `CALayer.h` | `ContentsGravity::BottomLeft` |
| `kCAGravityBottomRight` | constant | `CALayer.h` | `ContentsGravity::BottomRight` |
| `kCAGravityCenter` | constant | `CALayer.h` | `ContentsGravity::Center` |
| `kCAGravityLeft` | constant | `CALayer.h` | `ContentsGravity::Left` |
| `kCAGravityResize` | constant | `CALayer.h` | `ContentsGravity::Resize` |
| `kCAGravityResizeAspect` | constant | `CALayer.h` | `ContentsGravity::ResizeAspect` |
| `kCAGravityResizeAspectFill` | constant | `CALayer.h` | `ContentsGravity::ResizeAspectFill` |
| `kCAGravityRight` | constant | `CALayer.h` | `ContentsGravity::Right` |
| `kCAGravityTop` | constant | `CALayer.h` | `ContentsGravity::Top` |
| `kCAGravityTopLeft` | constant | `CALayer.h` | `ContentsGravity::TopLeft` |
| `kCAGravityTopRight` | constant | `CALayer.h` | `ContentsGravity::TopRight` |
| `CAMediaTiming` | protocol | `CAMediaTiming.h` | `Animation::{begin_time, speed, time_offset, repeat_duration, fill_mode}` |
| `CAMediaTimingFillMode` | typealias | `CAMediaTiming.h` | `MediaTimingFillMode` |
| `kCAFillModeBackwards` | constant | `CAMediaTiming.h` | `MediaTimingFillMode::Backwards` |
| `kCAFillModeBoth` | constant | `CAMediaTiming.h` | `MediaTimingFillMode::Both` |
| `kCAFillModeForwards` | constant | `CAMediaTiming.h` | `MediaTimingFillMode::Forwards` |
| `kCAFillModeRemoved` | constant | `CAMediaTiming.h` | `MediaTimingFillMode::Removed` |
| `CAMediaTimingFunctionName` | typealias | `CAMediaTimingFunction.h` | `TimingFunctionName` |
| `kCAMediaTimingFunctionDefault` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::Default` |
| `kCAMediaTimingFunctionEaseIn` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseIn` |
| `kCAMediaTimingFunctionEaseInEaseOut` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseInEaseOut` |
| `kCAMediaTimingFunctionEaseOut` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseOut` |
| `kCAMediaTimingFunctionLinear` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::Linear` |
| `CAMetalDrawable` | protocol | `CAMetalLayer.h` | `MetalDrawable` |
| `CAMetalLayer` | interface | `CAMetalLayer.h` | `MetalLayer` |
| `CARenderer` | interface | `CARenderer.h` | `Renderer` |
| `kCARendererMetalCommandQueue` | constant | `CARenderer.h` | `Renderer::new(texture, Some(queue))` |
| `CAReplicatorLayer` | interface | `CAReplicatorLayer.h` | `ReplicatorLayer` |
| `CAScrollLayer` | interface | `CAScrollLayer.h` | `ScrollLayer` |
| `CAScrollLayerScrollMode` | typealias | `CAScrollLayer.h` | `ScrollMode` |
| `kCAScrollBoth` | constant | `CAScrollLayer.h` | `ScrollMode::Both` |
| `kCAScrollHorizontally` | constant | `CAScrollLayer.h` | `ScrollMode::Horizontally` |
| `kCAScrollNone` | constant | `CAScrollLayer.h` | `ScrollMode::None` |
| `kCAScrollVertically` | constant | `CAScrollLayer.h` | `ScrollMode::Vertically` |
| `CAShapeLayer` | interface | `CAShapeLayer.h` | `ShapeLayer` |
| `CAShapeLayerFillRule` | typealias | `CAShapeLayer.h` | `ShapeFillRule` |
| `CAShapeLayerLineCap` | typealias | `CAShapeLayer.h` | `LineCap` |
| `CAShapeLayerLineJoin` | typealias | `CAShapeLayer.h` | `LineJoin` |
| `kCAFillRuleEvenOdd` | constant | `CAShapeLayer.h` | `ShapeFillRule::EvenOdd` |
| `kCAFillRuleNonZero` | constant | `CAShapeLayer.h` | `ShapeFillRule::NonZero` |
| `kCALineCapButt` | constant | `CAShapeLayer.h` | `LineCap::Butt` |
| `kCALineCapRound` | constant | `CAShapeLayer.h` | `LineCap::Round` |
| `kCALineCapSquare` | constant | `CAShapeLayer.h` | `LineCap::Square` |
| `kCALineJoinBevel` | constant | `CAShapeLayer.h` | `LineJoin::Bevel` |
| `kCALineJoinMiter` | constant | `CAShapeLayer.h` | `LineJoin::Miter` |
| `kCALineJoinRound` | constant | `CAShapeLayer.h` | `LineJoin::Round` |
| `CATextLayer` | interface | `CATextLayer.h` | `TextLayer` |
| `CATextLayerAlignmentMode` | typealias | `CATextLayer.h` | `TextAlignmentMode` |
| `CATextLayerTruncationMode` | typealias | `CATextLayer.h` | `TextTruncationMode` |
| `kCAAlignmentCenter` | constant | `CATextLayer.h` | `TextAlignmentMode::Center` |
| `kCAAlignmentJustified` | constant | `CATextLayer.h` | `TextAlignmentMode::Justified` |
| `kCAAlignmentLeft` | constant | `CATextLayer.h` | `TextAlignmentMode::Left` |
| `kCAAlignmentNatural` | constant | `CATextLayer.h` | `TextAlignmentMode::Natural` |
| `kCAAlignmentRight` | constant | `CATextLayer.h` | `TextAlignmentMode::Right` |
| `kCATruncationEnd` | constant | `CATextLayer.h` | `TextTruncationMode::End` |
| `kCATruncationMiddle` | constant | `CATextLayer.h` | `TextTruncationMode::Middle` |
| `kCATruncationNone` | constant | `CATextLayer.h` | `TextTruncationMode::None` |
| `kCATruncationStart` | constant | `CATextLayer.h` | `TextTruncationMode::Start` |
| `CATiledLayer` | interface | `CATiledLayer.h` | `TiledLayer` |
| `CATransaction` | interface | `CATransaction.h` | `Transaction` |
| `kCATransactionAnimationDuration` | constant | `CATransaction.h` | `Transaction::{animation_duration, set_animation_duration}` |
| `kCATransactionAnimationTimingFunction` | constant | `CATransaction.h` | `Transaction::{animation_timing_function_name, set_animation_timing_function_name}` |
| `kCATransactionCompletionBlock` | constant | `CATransaction.h` | `Transaction::set_completion_block` |
| `kCATransactionDisableActions` | constant | `CATransaction.h` | `Transaction::{disable_actions, set_disable_actions}` |
| `CATransform3D` | typedef | `CATransform3D.h` | `Transform3D` |
| `CATransform3DEqualToTransform` | function | `CATransform3D.h` | `Transform3D: PartialEq` |
| `CATransform3DIdentity` | constant | `CATransform3D.h` | `Transform3D::identity` |
| `CATransform3DIsIdentity` | function | `CATransform3D.h` | `Transform3D::identity + PartialEq` |
| `CATransform3DMakeScale` | function | `CATransform3D.h` | `Transform3D::scale` |
| `CATransform3DMakeTranslation` | function | `CATransform3D.h` | `Transform3D::translation` |
| `CATransformLayer` | interface | `CATransformLayer.h` | `TransformLayer` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `CAAnimationDelegate` | protocol | `CAAnimation.h` | No delegate bridge for animation lifecycle callbacks. |
| `CAPropertyAnimation` | interface | `CAAnimation.h` | Key-path/additive/cumulative helpers exist on concrete animation wrappers, but there is no first-class `PropertyAnimation` type. |
| `CACurrentMediaTime` | function | `CABase.h` | No Rust helper for the global Core Animation media clock. |
| `CAConstraint` | interface | `CAConstraintLayoutManager.h` | Constraint-based layer layout APIs are not wrapped. |
| `CAConstraintLayoutManager` | interface | `CAConstraintLayoutManager.h` | Constraint-based layer layout APIs are not wrapped. |
| `CAEDRMetadata` | interface | `CAEDRMetadata.h` | HDR/EDR metadata objects are not exposed. |
| `CAFrameRateRange` | typedef | `CAFrameRateRange.h` | No `CAFrameRateRange` value type or preferred-frame-rate bindings. |
| `CAFrameRateRangeDefault` | constant | `CAFrameRateRange.h` | No `CAFrameRateRange` value type or preferred-frame-rate bindings. |
| `CAFrameRateRangeIsEqualToRange` | function | `CAFrameRateRange.h` | No `CAFrameRateRange` value type or preferred-frame-rate bindings. |
| `CAFrameRateRangeMake` | function | `CAFrameRateRange.h` | No `CAFrameRateRange` value type or preferred-frame-rate bindings. |
| `CAAction` | protocol | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `CAAutoresizingMask` | enum | `CALayer.h` | Layer bitmask enums for autoresizing / edge-antialiasing / masked-corners are not exposed. |
| `CACornerMask` | enum | `CALayer.h` | Layer bitmask enums for autoresizing / edge-antialiasing / masked-corners are not exposed. |
| `CADynamicRange` | typealias | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CADynamicRangeAutomatic` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CADynamicRangeConstrainedHigh` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CADynamicRangeHigh` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CADynamicRangeStandard` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CAEdgeAntialiasingMask` | enum | `CALayer.h` | Layer bitmask enums for autoresizing / edge-antialiasing / masked-corners are not exposed. |
| `CALayerContentsFilter` | typealias | `CALayer.h` | Layer contents-filter APIs are not exposed. |
| `CALayerContentsFormat` | typealias | `CALayer.h` | Layer contents-format APIs are not exposed. |
| `CALayerCornerCurve` | typealias | `CALayer.h` | Layer corner-curve APIs are not exposed. |
| `CALayerDelegate` | protocol | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `CALayoutManager` | protocol | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `CAToneMapMode` | typealias | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CAToneMapModeAutomatic` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CAToneMapModeIfSupported` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `CAToneMapModeNever` | constant | `CALayer.h` | HDR dynamic-range / tone-mapping APIs on `CALayer` / `CAMetalLayer` are not wrapped. |
| `kCAContentsFormatGray8Uint` | constant | `CALayer.h` | Layer contents-format APIs are not exposed. |
| `kCAContentsFormatRGBA16Float` | constant | `CALayer.h` | Layer contents-format APIs are not exposed. |
| `kCAContentsFormatRGBA8Uint` | constant | `CALayer.h` | Layer contents-format APIs are not exposed. |
| `kCACornerCurveCircular` | constant | `CALayer.h` | Layer corner-curve APIs are not exposed. |
| `kCACornerCurveContinuous` | constant | `CALayer.h` | Layer corner-curve APIs are not exposed. |
| `kCAFilterLinear` | constant | `CALayer.h` | Layer contents-filter APIs are not exposed. |
| `kCAFilterNearest` | constant | `CALayer.h` | Layer contents-filter APIs are not exposed. |
| `kCAFilterTrilinear` | constant | `CALayer.h` | Layer contents-filter APIs are not exposed. |
| `kCAOnOrderIn` | constant | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `kCAOnOrderOut` | constant | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `kCATransition` | constant | `CALayer.h` | No CALayer action / delegate / layout-manager bridge or action-key constants. |
| `CAMediaTimingFunction` | interface | `CAMediaTimingFunction.h` | Only named timing functions are exposed; there is no `CAMediaTimingFunction` object wrapper. |
| `CAMetalDisplayLink` | interface | `CAMetalDisplayLink.h` | No `CAMetalDisplayLink` / update / delegate bindings. |
| `CAMetalDisplayLinkDelegate` | protocol | `CAMetalDisplayLink.h` | No `CAMetalDisplayLink` / update / delegate bindings. |
| `CAMetalDisplayLinkUpdate` | interface | `CAMetalDisplayLink.h` | No `CAMetalDisplayLink` / update / delegate bindings. |
| `CARemoteLayerClient` | interface | `CARemoteLayerClient.h` | Remote layer client/server APIs are not exposed. |
| `CARemoteLayerServer` | interface | `CARemoteLayerServer.h` | Remote layer client/server APIs are not exposed. |
| `kCARendererColorSpace` | constant | `CARenderer.h` | Renderer options do not expose the color-space key. |
| `CATransform3DConcat` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DGetAffineTransform` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DInvert` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DIsAffine` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DMakeAffineTransform` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DMakeRotation` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DRotate` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DScale` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CATransform3DTranslate` | function | `CATransform3D.h` | Rotation / concat / invert / affine matrix helpers are not exposed on `Transform3D`. |
| `CAValueFunction` | interface | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `CAValueFunctionName` | typealias | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionRotateX` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionRotateY` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionRotateZ` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionScale` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionScaleX` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionScaleY` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionScaleZ` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionTranslate` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionTranslateX` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionTranslateY` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |
| `kCAValueFunctionTranslateZ` | constant | `CAValueFunction.h` | No `CAValueFunction` wrapper or value-function-name enum is exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CAOpenGLLayer` | interface | `CAOpenGLLayer.h` | OpenGL layer support is deprecated on macOS and intentionally skipped per the audit instructions. | `API_DEPRECATED("OpenGL is deprecated. (Define GL_SILENCE_DEPRECATION to silence these warnings)", macos(10.5, 10.14), macCatalyst(13.1, 13.1))` |

