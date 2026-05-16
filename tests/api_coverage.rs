#![allow(clippy::cast_precision_loss)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn sdk_root() -> PathBuf {
    let output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun must be available");
    assert!(output.status.success(), "xcrun --show-sdk-path failed");
    PathBuf::from(String::from_utf8(output.stdout).unwrap().trim().to_string())
}

fn extract_header_symbols(path: &Path) -> BTreeSet<String> {
    let contents = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("can't read {}: {error}", path.display()));
    let regex = regex_lite::Regex::new(r"\b(MIDI[A-Za-z0-9_]+)\s*\(").unwrap();
    regex
        .captures_iter(&contents)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn extract_sdk_symbols(sdk_root: &Path) -> BTreeSet<String> {
    let header_root = sdk_root.join("System/Library/Frameworks/CoreMIDI.framework/Headers");
    [
        "MIDIServices.h",
        "MIDISetup.h",
        "MIDIDriver.h",
        "MIDIThruConnection.h",
        "MIDIBluetoothConnection.h",
    ]
    .into_iter()
    .map(|header| extract_header_symbols(&header_root.join(header)))
    .fold(BTreeSet::new(), |mut all, symbols| {
        all.extend(symbols);
        all
    })
}

fn extract_rust_symbols() -> BTreeSet<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    let contents = std::fs::read_to_string(path).expect("ffi source must be readable");
    let regex =
        regex_lite::Regex::new(r"pub(?:\(crate\))?\s+fn\s+(MIDI[A-Za-z0-9_]+)\s*\(").unwrap();
    regex
        .captures_iter(&contents)
        .map(|capture| capture[1].to_string())
        .collect()
}

fn required_symbols() -> BTreeSet<String> {
    [
        "MIDIClientCreate",
        "MIDIClientDispose",
        "MIDIInputPortCreate",
        "MIDIInputPortCreateWithProtocol",
        "MIDIOutputPortCreate",
        "MIDIPortDispose",
        "MIDIPortConnectSource",
        "MIDIPortDisconnectSource",
        "MIDIGetNumberOfDevices",
        "MIDIGetDevice",
        "MIDIDeviceGetNumberOfEntities",
        "MIDIDeviceGetEntity",
        "MIDIEntityGetNumberOfSources",
        "MIDIEntityGetSource",
        "MIDIEntityGetNumberOfDestinations",
        "MIDIEntityGetDestination",
        "MIDIObjectGetStringProperty",
        "MIDIObjectGetIntegerProperty",
        "MIDIDestinationCreate",
        "MIDISourceCreate",
        "MIDIEndpointDispose",
        "MIDIPacketListInit",
        "MIDIPacketListAdd",
        "MIDISend",
        "MIDIReceived",
        "MIDIEventListInit",
        "MIDIEventListAdd",
        "MIDISendEventList",
        "MIDIReceivedEventList",
        "MIDIExternalDeviceCreate",
        "MIDISetupAddExternalDevice",
        "MIDISetupRemoveExternalDevice",
        "MIDIThruConnectionCreate",
        "MIDIThruConnectionGetParams",
        "MIDIThruConnectionSetParams",
        "MIDIThruConnectionFind",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

#[test]
fn coremidi_symbol_audit() {
    let sdk = sdk_root();
    let apple = extract_sdk_symbols(&sdk);
    let ours = extract_rust_symbols();
    let required = required_symbols();

    for symbol in &required {
        assert!(
            apple.contains(symbol),
            "Apple SDK headers missing expected symbol {symbol}"
        );
        assert!(
            ours.contains(symbol),
            "Rust ffi missing required symbol {symbol}"
        );
    }

    let unknown: BTreeSet<_> = ours.difference(&apple).cloned().collect();
    assert!(
        unknown.is_empty(),
        "unknown CoreMIDI symbol(s) in src/ffi/mod.rs: {unknown:?}"
    );

    println!(
        "wrapped {} CoreMIDI C symbols across MIDIServices/MIDISetup/MIDIDriver/MIDIThruConnection/MIDIBluetoothConnection",
        ours.len(),
    );
}
