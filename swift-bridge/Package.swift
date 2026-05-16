// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreMIDIBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "CoreMIDIBridge",
            type: .static,
            targets: ["CoreMIDIBridge"]
        )
    ],
    targets: [
        .target(
            name: "CoreMIDIBridge",
            path: "Sources/CoreMIDIBridge",
            publicHeadersPath: "include"
        )
    ]
)
