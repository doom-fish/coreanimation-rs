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
    KeyframeAnimation, RotationMode, SpringAnimation, Transition, TransitionSubtype,
    TransitionType,
};
pub use color::Color;
pub use display_link::{CVReturn, CVSMPTETime, CVTime, CVTimeStamp, DisplayLink};
pub use emitter::{EmitterCell, EmitterLayer, EmitterMode, EmitterRenderMode, EmitterShape};
pub use error::CoreAnimationError;
pub use layer::{
    ContentsGravity, GradientLayer, GradientType, Layer, LayerLike, LineCap, LineJoin,
    MetalDrawable, MetalLayer, ShapeLayer, TextAlignmentMode, TextLayer, TextTruncationMode,
};
pub use path::Path;
pub use renderer::{read_texture_bytes, Renderer};
pub use transaction::{Transaction, TransactionCompletion};
pub use transform::Transform3D;

pub use apple_cf::cg::{CGImage, CGPoint, CGRect, CGSize};

pub mod prelude {
    pub use crate::{
        read_texture_bytes, Animation, AnimationCalculationMode, AnimationGroup, AnimationLike,
        BasicAnimation, CVReturn, CVSMPTETime, CVTime, CVTimeStamp, Color, ContentsGravity,
        CoreAnimationError, DisplayLink, EmitterCell, EmitterLayer, EmitterMode, EmitterRenderMode,
        EmitterShape, GradientLayer, GradientType, KeyframeAnimation, Layer, LayerLike, LineCap,
        LineJoin, MetalDrawable, MetalLayer, Path, Renderer, RotationMode, ShapeLayer,
        SpringAnimation, TextAlignmentMode, TextLayer, TextTruncationMode, Transaction,
        TransactionCompletion, Transform3D, Transition, TransitionSubtype, TransitionType,
    };
}
