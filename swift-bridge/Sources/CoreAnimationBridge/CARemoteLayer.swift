import Foundation
import QuartzCore

@_cdecl("ca_remote_layer_server_shared")
public func ca_remote_layer_server_shared() -> UnsafeMutableRawPointer? {
    caRetain(CARemoteLayerServer.shared())
}

@_cdecl("ca_remote_layer_server_get_port")
public func ca_remote_layer_server_get_port(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let server: CARemoteLayerServer = caBorrow(handle) else { return 0 }
    return UInt32(server.serverPort)
}

@_cdecl("ca_remote_layer_server_layer_with_client_id")
public func ca_remote_layer_server_layer_with_client_id(_ clientId: UInt32) -> UnsafeMutableRawPointer? {
    caRetain(CALayer(remoteClientId: clientId))
}

@_cdecl("ca_remote_layer_client_new")
public func ca_remote_layer_client_new(_ serverPort: UInt32) -> UnsafeMutableRawPointer? {
    caRetain(CARemoteLayerClient(serverPort: serverPort))
}

@_cdecl("ca_remote_layer_client_invalidate")
public func ca_remote_layer_client_invalidate(_ handle: UnsafeMutableRawPointer?) {
    guard let client: CARemoteLayerClient = caBorrow(handle) else { return }
    client.invalidate()
}

@_cdecl("ca_remote_layer_client_get_client_id")
public func ca_remote_layer_client_get_client_id(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let client: CARemoteLayerClient = caBorrow(handle) else { return 0 }
    return client.clientId
}

@_cdecl("ca_remote_layer_client_get_layer")
public func ca_remote_layer_client_get_layer(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let client: CARemoteLayerClient = caBorrow(handle), let layer = client.layer else { return nil }
    return caRetain(layer)
}

@_cdecl("ca_remote_layer_client_set_layer")
public func ca_remote_layer_client_set_layer(_ handle: UnsafeMutableRawPointer?, _ layerHandle: UnsafeMutableRawPointer?) {
    guard let client: CARemoteLayerClient = caBorrow(handle) else { return }
    let layer: CALayer? = caBorrow(layerHandle)
    client.layer = layer
}
