use std::ops::Deref;

use crate::layer::{Layer, LayerLike};

#[derive(Debug, Clone)]
pub struct TransformLayer {
    inner: Layer,
}

impl TransformLayer {
    #[must_use]
    pub fn new() -> Option<Self> {
        unsafe { Layer::from_raw(crate::ffi::ca_transform_layer_new()) }.map(|inner| Self { inner })
    }
}

impl Deref for TransformLayer {
    type Target = Layer;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LayerLike for TransformLayer {
    fn as_layer_ptr(&self) -> *mut core::ffi::c_void {
        self.inner.as_ptr()
    }
}
