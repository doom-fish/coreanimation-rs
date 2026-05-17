import Foundation
import QuartzCore

private func caData(_ bytes: UnsafeRawPointer?, _ length: Int) -> Data? {
    guard let bytes, length > 0 else { return nil }
    return Data(bytes: bytes, count: length)
}

@_cdecl("ca_edr_metadata_is_available")
public func ca_edr_metadata_is_available() -> Bool {
    if #available(macOS 13.0, *) {
        return CAEDRMetadata.isAvailable
    }
    return false
}

@_cdecl("ca_edr_metadata_new_hdr10_with_display_info")
public func ca_edr_metadata_new_hdr10_with_display_info(
    _ displayInfoBytes: UnsafeRawPointer?,
    _ displayInfoLen: Int,
    _ contentInfoBytes: UnsafeRawPointer?,
    _ contentInfoLen: Int,
    _ opticalOutputScale: Float
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *) else { return nil }
    return caRetain(
        CAEDRMetadata.hdr10(
            displayInfo: caData(displayInfoBytes, displayInfoLen),
            contentInfo: caData(contentInfoBytes, contentInfoLen),
            opticalOutputScale: opticalOutputScale
        )
    )
}

@_cdecl("ca_edr_metadata_new_hdr10")
public func ca_edr_metadata_new_hdr10(
    _ minLuminance: Float,
    _ maxLuminance: Float,
    _ opticalOutputScale: Float
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 10.15, *) else { return nil }
    return caRetain(
        CAEDRMetadata.hdr10(
            minLuminance: minLuminance,
            maxLuminance: maxLuminance,
            opticalOutputScale: opticalOutputScale
        )
    )
}

@_cdecl("ca_edr_metadata_new_hlg")
public func ca_edr_metadata_new_hlg(
    _ ambientViewingEnvironmentBytes: UnsafeRawPointer?,
    _ ambientViewingEnvironmentLen: Int
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *), let data = caData(ambientViewingEnvironmentBytes, ambientViewingEnvironmentLen) else {
        return nil
    }
    return caRetain(CAEDRMetadata.hlg(ambientViewingEnvironment: data))
}

@_cdecl("ca_edr_metadata_get_hlg")
public func ca_edr_metadata_get_hlg() -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.0, *) else { return nil }
    return caRetain(CAEDRMetadata.hlg)
}
