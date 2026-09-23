// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreAnimationBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "CoreAnimationBridge",
            type: .static,
            targets: ["CoreAnimationBridge"])
    ],
    targets: [
        .target(
            name: "CoreAnimationObjCBridge",
            path: "Sources/CoreAnimationObjCBridge",
            publicHeadersPath: "include"),
        .target(
            name: "CoreAnimationBridge",
            dependencies: ["CoreAnimationObjCBridge"],
            path: "Sources/CoreAnimationBridge")
    ]
)
