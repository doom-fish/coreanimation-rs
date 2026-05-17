import Foundation
import QuartzCore

private func caConstraintAttribute(_ raw: Int32) -> CAConstraintAttribute {
    switch raw {
    case 1: return .midX
    case 2: return .maxX
    case 3: return .width
    case 4: return .minY
    case 5: return .midY
    case 6: return .maxY
    case 7: return .height
    default: return .minX
    }
}

private func caConstraintAttributeRaw(_ value: CAConstraintAttribute) -> Int32 {
    switch value {
    case .midX: return 1
    case .maxX: return 2
    case .width: return 3
    case .minY: return 4
    case .midY: return 5
    case .maxY: return 6
    case .height: return 7
    default: return 0
    }
}

private func caBorrowLayoutManager(_ handle: UnsafeMutableRawPointer?) -> CALayoutManager? {
    guard let handle else { return nil }
    return Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? CALayoutManager
}

@_cdecl("ca_constraint_layout_manager_new")
public func ca_constraint_layout_manager_new() -> UnsafeMutableRawPointer? {
    caRetain(CAConstraintLayoutManager())
}

@_cdecl("ca_constraint_new")
public func ca_constraint_new(
    _ attribute: Int32,
    _ sourceName: UnsafePointer<CChar>?,
    _ sourceAttribute: Int32,
    _ scale: Double,
    _ offset: Double
) -> UnsafeMutableRawPointer? {
    guard let sourceName = caCString(sourceName) else { return nil }
    return caRetain(
        CAConstraint(
            attribute: caConstraintAttribute(attribute),
            relativeTo: sourceName,
            attribute: caConstraintAttribute(sourceAttribute),
            scale: scale,
            offset: offset
        )
    )
}

@_cdecl("ca_constraint_get_attribute")
public func ca_constraint_get_attribute(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let constraint: CAConstraint = caBorrow(handle) else { return 0 }
    return caConstraintAttributeRaw(constraint.attribute)
}

@_cdecl("ca_constraint_get_source_name")
public func ca_constraint_get_source_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let constraint: CAConstraint = caBorrow(handle) else { return nil }
    return caDup(constraint.sourceName)
}

@_cdecl("ca_constraint_get_source_attribute")
public func ca_constraint_get_source_attribute(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let constraint: CAConstraint = caBorrow(handle) else { return 0 }
    return caConstraintAttributeRaw(constraint.sourceAttribute)
}

@_cdecl("ca_constraint_get_scale")
public func ca_constraint_get_scale(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: CAConstraint = caBorrow(handle) else { return 0 }
    return constraint.scale
}

@_cdecl("ca_constraint_get_offset")
public func ca_constraint_get_offset(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let constraint: CAConstraint = caBorrow(handle) else { return 0 }
    return constraint.offset
}

@_cdecl("ca_layer_get_layout_manager")
public func ca_layer_get_layout_manager(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let layoutManager = layer.layoutManager else {
        return nil
    }
    return caRetain(layoutManager as AnyObject)
}

@_cdecl("ca_layer_set_layout_manager")
public func ca_layer_set_layout_manager(_ handle: UnsafeMutableRawPointer?, _ managerHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    layer.layoutManager = caBorrowLayoutManager(managerHandle)
}

@_cdecl("ca_layer_constraint_count")
public func ca_layer_constraint_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let layer: CALayer = caBorrow(handle) else { return 0 }
    return layer.constraints?.count ?? 0
}

@_cdecl("ca_layer_constraint_at")
public func ca_layer_constraint_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let layer: CALayer = caBorrow(handle), let constraints = layer.constraints, index >= 0, index < constraints.count else {
        return nil
    }
    return caRetain(constraints[index])
}

@_cdecl("ca_layer_set_constraints")
public func ca_layer_set_constraints(
    _ handle: UnsafeMutableRawPointer?,
    _ constraintHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    guard let layer: CALayer = caBorrow(handle) else { return }
    guard let constraintHandles, count > 0 else {
        layer.constraints = nil
        return
    }
    layer.constraints = (0..<count).compactMap { index in
        let handle = constraintHandles[index]
        let constraint: CAConstraint? = caBorrow(handle)
        return constraint
    }
}

@_cdecl("ca_layer_add_constraint")
public func ca_layer_add_constraint(_ handle: UnsafeMutableRawPointer?, _ constraintHandle: UnsafeMutableRawPointer?) {
    guard let layer: CALayer = caBorrow(handle), let constraint: CAConstraint = caBorrow(constraintHandle) else { return }
    layer.addConstraint(constraint)
}
