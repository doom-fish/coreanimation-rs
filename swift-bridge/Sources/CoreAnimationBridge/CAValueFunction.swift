import Foundation
import QuartzCore

@_cdecl("ca_value_function_new")
public func ca_value_function_new(_ value: Int32) -> UnsafeMutableRawPointer? {
    guard let function = CAValueFunction(name: caValueFunctionName(value)) else { return nil }
    return caRetain(function)
}

@_cdecl("ca_value_function_get_name")
public func ca_value_function_get_name(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let function: CAValueFunction = caBorrow(handle) else { return -1 }
    return caValueFunctionNameRaw(function.name)
}
