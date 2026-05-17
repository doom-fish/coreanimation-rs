use crate::layer::{Layer, LayerLike};
use crate::private::handle_type;

handle_type!(RemoteLayerServer);
handle_type!(RemoteLayerClient);

impl RemoteLayerServer {
    #[must_use]
    pub fn shared() -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_remote_layer_server_shared()) }
    }

    #[must_use]
    pub fn server_port(&self) -> u32 {
        unsafe { crate::ffi::ca_remote_layer_server_get_port(self.as_ptr()) }
    }

    #[must_use]
    pub fn layer_with_client_id(client_id: u32) -> Option<Layer> {
        unsafe { Layer::from_raw(crate::ffi::ca_remote_layer_server_layer_with_client_id(client_id)) }
    }
}

impl RemoteLayerClient {
    #[must_use]
    pub fn new(server_port: u32) -> Option<Self> {
        unsafe { Self::from_raw(crate::ffi::ca_remote_layer_client_new(server_port)) }
    }

    pub fn invalidate(&self) {
        unsafe { crate::ffi::ca_remote_layer_client_invalidate(self.as_ptr()) };
    }

    #[must_use]
    pub fn client_id(&self) -> u32 {
        unsafe { crate::ffi::ca_remote_layer_client_get_client_id(self.as_ptr()) }
    }

    #[must_use]
    pub fn layer(&self) -> Option<Layer> {
        unsafe { Layer::from_raw(crate::ffi::ca_remote_layer_client_get_layer(self.as_ptr())) }
    }

    pub fn set_layer<L: LayerLike>(&self, layer: Option<&L>) {
        unsafe {
            crate::ffi::ca_remote_layer_client_set_layer(
                self.as_ptr(),
                layer.map_or(core::ptr::null_mut(), LayerLike::as_layer_ptr),
            )
        };
    }
}
