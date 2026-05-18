#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::borrow_as_ptr)]
#![allow(clippy::ref_as_ptr)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::tuple_array_conversions)]
#![allow(clippy::use_self)]
#![allow(clippy::redundant_closure_for_method_calls)]

mod private;

/// `Core Animation` animation wrappers and related value enums.
pub mod animation;
/// Extensions for `CAAction` wrappers.
pub mod ca_action;
/// Extensions for `CAAnimation` wrappers.
pub mod ca_animation;
/// Helpers for `CAAnimationDelegate` callbacks.
pub mod ca_animation_delegate;
/// Extensions for `CAAnimationGroup` wrappers.
pub mod ca_animation_group;
/// Extensions for `CABasicAnimation` wrappers.
pub mod ca_basic_animation;
/// Extensions for `CAConstraint` and layout-manager wrappers.
pub mod ca_constraint;
/// Extensions for `CADisplayLink` wrappers.
pub mod ca_display_link;
/// Extensions for EDR metadata wrappers.
pub mod ca_edr_metadata;
/// Extensions for `CAEmitterLayer` wrappers.
pub mod ca_emitter_layer;
/// Extensions for frame-rate range helpers.
pub mod ca_frame_rate_range;
/// Extensions for `CAGradientLayer` wrappers.
pub mod ca_gradient_layer;
/// Extensions for `CAKeyframeAnimation` wrappers.
pub mod ca_keyframe_animation;
/// Extensions for `CALayer` wrappers.
pub mod ca_layer;
/// Extensions for `CAMediaTiming` and timing-function wrappers.
pub mod ca_media_timing;
/// Extensions for `CAMetalDisplayLink` wrappers.
pub mod ca_metal_display_link;
/// Extensions for `CAMetalLayer` wrappers.
pub mod ca_metal_layer;
/// Extensions for `CAPropertyAnimation` wrappers.
pub mod ca_property_animation;
/// Extensions for remote layer wrappers.
pub mod ca_remote_layer;
/// Extensions for `CAReplicatorLayer` wrappers.
pub mod ca_replicator_layer;
/// Extensions for `CAScrollLayer` wrappers.
pub mod ca_scroll_layer;
/// Extensions for `CAShapeLayer` wrappers.
pub mod ca_shape_layer;
/// Extensions for `CASpringAnimation` wrappers.
pub mod ca_spring_animation;
/// Extensions for `CATextLayer` wrappers.
pub mod ca_text_layer;
/// Extensions for `CATiledLayer` wrappers.
pub mod ca_tiled_layer;
/// Extensions for `CATransaction` helpers.
pub mod ca_transaction;
/// Extensions for `CATransformLayer` wrappers.
pub mod ca_transform_layer;
/// Extensions for `CATransition` wrappers.
pub mod ca_transition;
/// Extensions for `CAValueFunction` wrappers.
pub mod ca_value_function;
/// Color helpers backed by Core Graphics colors.
pub mod color;
/// `Core Video` display-link wrappers.
pub mod display_link;
/// Emitter layer and emitter cell wrappers.
pub mod emitter;
/// Error types used by the crate.
pub mod error;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
/// Raw ``QuartzCore`` and `Core Video` FFI bindings.
pub mod ffi;
#[cfg(not(feature = "raw-ffi"))]
/// Raw ``QuartzCore`` and `Core Video` FFI bindings.
pub(crate) mod ffi;
/// Layer wrappers, delegates, and related enums.
pub mod layer;
/// Path helpers backed by Core Graphics mutable paths.
pub mod path;
/// `CARenderer` wrappers and texture readback helpers.
pub mod renderer;
/// `CATransaction` helpers.
pub mod transaction;
/// `CATransform3D` helpers and conversions.
pub mod transform;

pub use animation::{
    Animation, AnimationCalculationMode, AnimationGroup, AnimationLike, BasicAnimation,
    KeyframeAnimation, PropertyAnimation, RotationMode, SpringAnimation, Transition,
    TransitionSubtype, TransitionType,
};
pub use ca_action::{Action, ActionLike};
pub use ca_animation_delegate::{current_media_time, AnimationDelegate};
pub use ca_constraint::{Constraint, ConstraintAttribute, ConstraintLayoutManager, LayoutManager};
pub use ca_display_link::QuartzDisplayLink;
pub use ca_edr_metadata::EDRMetadata;
pub use ca_frame_rate_range::FrameRateRange;
pub use ca_media_timing::{MediaTimingFillMode, TimingFunction, TimingFunctionName};
pub use ca_metal_display_link::{MetalDisplayLink, MetalDisplayLinkUpdate};
pub use ca_remote_layer::{RemoteLayerClient, RemoteLayerServer};
pub use ca_replicator_layer::ReplicatorLayer;
pub use ca_scroll_layer::{ScrollLayer, ScrollMode};
pub use ca_shape_layer::ShapeFillRule;
pub use ca_tiled_layer::TiledLayer;
pub use ca_transaction::TransactionLockGuard;
pub use ca_transform_layer::TransformLayer;
pub use ca_value_function::{ValueFunction, ValueFunctionName};
pub use color::Color;
pub use display_link::{CVReturn, CVSMPTETime, CVTime, CVTimeStamp, DisplayLink};
pub use emitter::{EmitterCell, EmitterLayer, EmitterMode, EmitterRenderMode, EmitterShape};
pub use error::CoreAnimationError;
pub use layer::{
    AutoresizingMask, ContentsFilter, ContentsFormat, ContentsGravity, CornerCurve, CornerMask,
    DynamicRange, EdgeAntialiasingMask, GradientLayer, GradientType, Layer, LayerActionKeys,
    LayerDelegate, LayerLike, LineCap, LineJoin, MetalDrawable, MetalLayer, ShapeLayer,
    TextAlignmentMode, TextLayer, TextTruncationMode, ToneMapMode,
};
pub use path::Path;
pub use renderer::{read_texture_bytes, Renderer};
pub use transaction::{Transaction, TransactionCompletion};
pub use transform::Transform3D;

pub use apple_cf::cg::{CGAffineTransform, CGColorSpace, CGImage, CGPoint, CGRect, CGSize};

/// Common ``QuartzCore``, `Core Video`, and helper re-exports.
pub mod prelude {
    pub use crate::{
        current_media_time, read_texture_bytes, Action, ActionLike, Animation,
        AnimationCalculationMode, AnimationDelegate, AnimationGroup, AnimationLike,
        AutoresizingMask, BasicAnimation, CGAffineTransform, CGColorSpace, CVReturn, CVSMPTETime,
        CVTime, CVTimeStamp, Color, Constraint, ConstraintAttribute, ConstraintLayoutManager,
        ContentsFilter, ContentsFormat, ContentsGravity, CoreAnimationError, CornerCurve,
        CornerMask, DisplayLink, DynamicRange, EDRMetadata, EdgeAntialiasingMask, EmitterCell,
        EmitterLayer, EmitterMode, EmitterRenderMode, EmitterShape, FrameRateRange, GradientLayer,
        GradientType, KeyframeAnimation, Layer, LayerActionKeys, LayerDelegate, LayerLike,
        LayoutManager, LineCap, LineJoin, MediaTimingFillMode, MetalDisplayLink,
        MetalDisplayLinkUpdate, MetalDrawable, MetalLayer, Path, PropertyAnimation,
        QuartzDisplayLink, RemoteLayerClient, RemoteLayerServer, Renderer, ReplicatorLayer,
        RotationMode, ScrollLayer, ScrollMode, ShapeFillRule, ShapeLayer, SpringAnimation,
        TextAlignmentMode, TextLayer, TextTruncationMode, TiledLayer, TimingFunction,
        TimingFunctionName, ToneMapMode, Transaction, TransactionCompletion, TransactionLockGuard,
        Transform3D, TransformLayer, Transition, TransitionSubtype, TransitionType, ValueFunction,
        ValueFunctionName,
    };
}
