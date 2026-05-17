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

pub mod animation;
pub mod ca_action;
pub mod ca_animation;
pub mod ca_animation_delegate;
pub mod ca_animation_group;
pub mod ca_basic_animation;
pub mod ca_constraint;
pub mod ca_display_link;
pub mod ca_edr_metadata;
pub mod ca_emitter_layer;
pub mod ca_frame_rate_range;
pub mod ca_gradient_layer;
pub mod ca_keyframe_animation;
pub mod ca_layer;
pub mod ca_media_timing;
pub mod ca_metal_display_link;
pub mod ca_metal_layer;
pub mod ca_property_animation;
pub mod ca_remote_layer;
pub mod ca_replicator_layer;
pub mod ca_scroll_layer;
pub mod ca_shape_layer;
pub mod ca_spring_animation;
pub mod ca_text_layer;
pub mod ca_tiled_layer;
pub mod ca_transaction;
pub mod ca_transform_layer;
pub mod ca_transition;
pub mod ca_value_function;
pub mod color;
pub mod display_link;
pub mod emitter;
pub mod error;
pub mod ffi;
pub mod layer;
pub mod path;
pub mod renderer;
pub mod transaction;
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

pub mod prelude {
    pub use crate::{
        current_media_time, read_texture_bytes, Action, ActionLike, Animation,
        AnimationCalculationMode, AnimationDelegate, AnimationGroup, AnimationLike,
        AutoresizingMask, BasicAnimation, CGAffineTransform, CGColorSpace, CVReturn,
        CVSMPTETime, CVTime, CVTimeStamp, Color,
        Constraint, ConstraintAttribute, ConstraintLayoutManager, ContentsFilter, ContentsFormat,
        ContentsGravity, CoreAnimationError, CornerCurve, CornerMask, DisplayLink, DynamicRange,
        EDRMetadata, EdgeAntialiasingMask, EmitterCell, EmitterLayer, EmitterMode,
        EmitterRenderMode, EmitterShape, FrameRateRange, GradientLayer, GradientType,
        KeyframeAnimation, Layer, LayerActionKeys, LayerDelegate, LayerLike, LayoutManager,
        LineCap, LineJoin, MediaTimingFillMode, MetalDisplayLink, MetalDisplayLinkUpdate,
        MetalDrawable, MetalLayer, Path, PropertyAnimation, QuartzDisplayLink, RemoteLayerClient,
        RemoteLayerServer, Renderer, ReplicatorLayer, RotationMode, ScrollLayer, ScrollMode,
        ShapeFillRule, ShapeLayer, SpringAnimation, TextAlignmentMode, TextLayer,
        TextTruncationMode, TiledLayer, TimingFunction, TimingFunctionName, ToneMapMode,
        Transaction, TransactionCompletion, TransactionLockGuard, Transform3D, TransformLayer,
        Transition, TransitionSubtype, TransitionType, ValueFunction, ValueFunctionName,
    };
}
