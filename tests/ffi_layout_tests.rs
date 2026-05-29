//! ABI layout assertions for the `#[repr(C)]` structs shared with the Swift bridge.
//!
//! `Transform3D` and `FrameRateRange` cross the Rust <-> Swift `@_cdecl` FFI
//! boundary (passed by value or marshalled through packed buffers). If their
//! size or alignment ever drifts from what the Swift side expects, the data
//! marshalling silently corrupts. These tests pin the layout so accidental
//! field reordering / type changes are caught at `cargo test` time rather than
//! as runtime garbage.

use std::mem::{align_of, size_of};

use coreanimation::ca_frame_rate_range::FrameRateRange;
use coreanimation::transform::Transform3D;

#[test]
fn transform3d_layout() {
    // 16 x f64
    assert_eq!(size_of::<Transform3D>(), 128, "Transform3D size drifted");
    assert_eq!(
        align_of::<Transform3D>(),
        8,
        "Transform3D alignment drifted"
    );
}

#[test]
fn frame_rate_range_layout() {
    // 3 x f32
    assert_eq!(
        size_of::<FrameRateRange>(),
        12,
        "FrameRateRange size drifted"
    );
    assert_eq!(
        align_of::<FrameRateRange>(),
        4,
        "FrameRateRange alignment drifted"
    );
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its*
/// `MemoryLayout` (size/stride/alignment) for `CATransform3D` and
/// `CAFrameRateRange` matches the values pinned on the Rust side. A `false`
/// return means the Rust and Swift layouts genuinely disagree, which is a real
/// ABI bug.
///
/// Gated on `raw-ffi` because that feature makes the `ffi` module public.
#[cfg(feature = "raw-ffi")]
#[test]
fn ffi_layout_matches_swift() {
    // SAFETY: `ca_verify_ffi_layout` takes no arguments and only reads
    // compile-time `MemoryLayout` constants in the Swift bridge.
    let matches = unsafe { coreanimation::ffi::ca_verify_ffi_layout() };
    assert!(
        matches,
        "Swift FFI struct layout disagrees with Rust layout (ABI mismatch)"
    );
}
