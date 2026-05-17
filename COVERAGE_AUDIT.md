# coreanimation-rs coverage audit (vs MacOSX26.2.sdk)

- Scope: `QuartzCore.framework` `CA*.h` CoreAnimation headers only (not the bundled CoreImage/CoreVideo umbrella headers).
- Filtered out macOS-unavailable surface: `CAEAGLLayer` (`API_UNAVAILABLE(macos, ...)`).
- Deprecated macOS symbol kept as EXEMPT: `CAOpenGLLayer`.

SDK_PUBLIC_SYMBOLS: 194
VERIFIED: 193
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 99.5%
NON_EXEMPT_COVERAGE_PCT: 100.0%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CAAnimation` | interface | `CAAnimation.h` | `Animation` |
| `CAAnimationCalculationMode` | typealias | `CAAnimation.h` | `AnimationCalculationMode` |
| `CAAnimationGroup` | interface | `CAAnimation.h` | `AnimationGroup` |
| `CAAnimationRotationMode` | typealias | `CAAnimation.h` | `RotationMode` |
| `CABasicAnimation` | interface | `CAAnimation.h` | `BasicAnimation` |
| `CAPropertyAnimation` | interface | `CAAnimation.h` | `PropertyAnimation` |
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
| `CAToneMapMode` | typealias | `CALayer.h` | `ToneMapMode` |
| `CAToneMapModeAutomatic` | constant | `CALayer.h` | `ToneMapMode::Automatic` |
| `CAToneMapModeIfSupported` | constant | `CALayer.h` | `ToneMapMode::IfSupported` |
| `CAToneMapModeNever` | constant | `CALayer.h` | `ToneMapMode::Never` |
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
| `CAMediaTimingFunction` | interface | `CAMediaTimingFunction.h` | `TimingFunction` |
| `kCAMediaTimingFunctionDefault` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::Default` |
| `kCAMediaTimingFunctionEaseIn` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseIn` |
| `kCAMediaTimingFunctionEaseInEaseOut` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseInEaseOut` |
| `kCAMediaTimingFunctionEaseOut` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::EaseOut` |
| `kCAMediaTimingFunctionLinear` | constant | `CAMediaTimingFunction.h` | `TimingFunctionName::Linear` |
| `CAMetalDrawable` | protocol | `CAMetalLayer.h` | `MetalDrawable` |
| `CAMetalLayer` | interface | `CAMetalLayer.h` | `MetalLayer` |
| `CAMetalDisplayLink` | interface | `CAMetalDisplayLink.h` | `MetalDisplayLink` |
| `CAMetalDisplayLinkDelegate` | protocol | `CAMetalDisplayLink.h` | `MetalDisplayLink::set_delegate` |
| `CAMetalDisplayLinkUpdate` | interface | `CAMetalDisplayLink.h` | `MetalDisplayLinkUpdate` |
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
| `CAValueFunction` | interface | `CAValueFunction.h` | `ValueFunction` |
| `CAValueFunctionName` | typealias | `CAValueFunction.h` | `ValueFunctionName` |
| `kCAValueFunctionRotateX` | constant | `CAValueFunction.h` | `ValueFunctionName::RotateX` |
| `kCAValueFunctionRotateY` | constant | `CAValueFunction.h` | `ValueFunctionName::RotateY` |
| `kCAValueFunctionRotateZ` | constant | `CAValueFunction.h` | `ValueFunctionName::RotateZ` |
| `kCAValueFunctionScale` | constant | `CAValueFunction.h` | `ValueFunctionName::Scale` |
| `kCAValueFunctionScaleX` | constant | `CAValueFunction.h` | `ValueFunctionName::ScaleX` |
| `kCAValueFunctionScaleY` | constant | `CAValueFunction.h` | `ValueFunctionName::ScaleY` |
| `kCAValueFunctionScaleZ` | constant | `CAValueFunction.h` | `ValueFunctionName::ScaleZ` |
| `kCAValueFunctionTranslate` | constant | `CAValueFunction.h` | `ValueFunctionName::Translate` |
| `kCAValueFunctionTranslateX` | constant | `CAValueFunction.h` | `ValueFunctionName::TranslateX` |
| `kCAValueFunctionTranslateY` | constant | `CAValueFunction.h` | `ValueFunctionName::TranslateY` |
| `kCAValueFunctionTranslateZ` | constant | `CAValueFunction.h` | `ValueFunctionName::TranslateZ` |
| `CAAnimationDelegate` | protocol | `CAAnimation.h` | `AnimationDelegate` |
| `CACurrentMediaTime` | function | `CABase.h` | `current_media_time` |
| `CAConstraint` | interface | `CAConstraintLayoutManager.h` | `Constraint` |
| `CAConstraintLayoutManager` | interface | `CAConstraintLayoutManager.h` | `ConstraintLayoutManager` |
| `CAEDRMetadata` | interface | `CAEDRMetadata.h` | `EDRMetadata` |
| `CAFrameRateRange` | typedef | `CAFrameRateRange.h` | `FrameRateRange` |
| `CAFrameRateRangeDefault` | constant | `CAFrameRateRange.h` | `FrameRateRange::DEFAULT` |
| `CAFrameRateRangeIsEqualToRange` | function | `CAFrameRateRange.h` | `FrameRateRange::is_equal_to_range` |
| `CAFrameRateRangeMake` | function | `CAFrameRateRange.h` | `FrameRateRange::make` |
| `CAAction` | protocol | `CALayer.h` | `Action`, `Layer::{action_handle_for_key, set_action_handle_for_key}` |
| `CAAutoresizingMask` | enum | `CALayer.h` | `AutoresizingMask` |
| `CACornerMask` | enum | `CALayer.h` | `CornerMask` |
| `CADynamicRange` | typealias | `CALayer.h` | `DynamicRange` |
| `CADynamicRangeAutomatic` | constant | `CALayer.h` | `DynamicRange::Automatic` |
| `CADynamicRangeConstrainedHigh` | constant | `CALayer.h` | `DynamicRange::ConstrainedHigh` |
| `CADynamicRangeHigh` | constant | `CALayer.h` | `DynamicRange::High` |
| `CADynamicRangeStandard` | constant | `CALayer.h` | `DynamicRange::Standard` |
| `CAEdgeAntialiasingMask` | enum | `CALayer.h` | `EdgeAntialiasingMask` |
| `CALayerContentsFilter` | typealias | `CALayer.h` | `ContentsFilter` |
| `CALayerContentsFormat` | typealias | `CALayer.h` | `ContentsFormat` |
| `CALayerCornerCurve` | typealias | `CALayer.h` | `CornerCurve` |
| `CALayerDelegate` | protocol | `CALayer.h` | `LayerDelegate`, `Layer::set_delegate` |
| `CALayoutManager` | protocol | `CALayer.h` | `LayoutManager`, `ConstraintLayoutManager` |
| `kCAContentsFormatGray8Uint` | constant | `CALayer.h` | `ContentsFormat::Gray8Uint` |
| `kCAContentsFormatRGBA16Float` | constant | `CALayer.h` | `ContentsFormat::RGBA16Float` |
| `kCAContentsFormatRGBA8Uint` | constant | `CALayer.h` | `ContentsFormat::RGBA8Uint` |
| `kCACornerCurveCircular` | constant | `CALayer.h` | `CornerCurve::Circular` |
| `kCACornerCurveContinuous` | constant | `CALayer.h` | `CornerCurve::Continuous` |
| `kCAFilterLinear` | constant | `CALayer.h` | `ContentsFilter::Linear` |
| `kCAFilterNearest` | constant | `CALayer.h` | `ContentsFilter::Nearest` |
| `kCAFilterTrilinear` | constant | `CALayer.h` | `ContentsFilter::Trilinear` |
| `kCAOnOrderIn` | constant | `CALayer.h` | `LayerActionKeys::ON_ORDER_IN` |
| `kCAOnOrderOut` | constant | `CALayer.h` | `LayerActionKeys::ON_ORDER_OUT` |
| `kCATransition` | constant | `CALayer.h` | `LayerActionKeys::TRANSITION` |
| `CARemoteLayerClient` | interface | `CARemoteLayerClient.h` | `RemoteLayerClient` |
| `CARemoteLayerServer` | interface | `CARemoteLayerServer.h` | `RemoteLayerServer` |
| `kCARendererColorSpace` | constant | `CARenderer.h` | `Renderer::new_with_color_space` |
| `CATransform3DConcat` | function | `CATransform3D.h` | `Transform3D::concat` |
| `CATransform3DGetAffineTransform` | function | `CATransform3D.h` | `Transform3D::to_affine` |
| `CATransform3DInvert` | function | `CATransform3D.h` | `Transform3D::inverted` |
| `CATransform3DIsAffine` | function | `CATransform3D.h` | `Transform3D::is_affine` |
| `CATransform3DMakeAffineTransform` | function | `CATransform3D.h` | `Transform3D::from_affine` |
| `CATransform3DMakeRotation` | function | `CATransform3D.h` | `Transform3D::rotation` |
| `CATransform3DRotate` | function | `CATransform3D.h` | `Transform3D::rotated` |
| `CATransform3DScale` | function | `CATransform3D.h` | `Transform3D::scaled` |
| `CATransform3DTranslate` | function | `CATransform3D.h` | `Transform3D::translated` |

## 🔴 GAPS
None. All non-exempt QuartzCore `CA*.h` symbols from the audit are wrapped.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CAOpenGLLayer` | interface | `CAOpenGLLayer.h` | OpenGL layer support is deprecated on macOS and intentionally skipped per the audit instructions. | `API_DEPRECATED("OpenGL is deprecated. (Define GL_SILENCE_DEPRECATION to silence these warnings)", macos(10.5, 10.14), macCatalyst(13.1, 13.1))` |

