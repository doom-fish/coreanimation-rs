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
            name: "CoreAnimationBridge",
            path: "Sources/CoreAnimationBridge",
            publicHeadersPath: "include")
    ]
)
