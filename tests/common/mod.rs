use std::os::unix::process::CommandExt;
use std::process::Command;
use std::sync::OnceLock;

use coremidi::prelude::*;

const REEXEC_ENV: &str = "COREMIDI_RS_TEST_REEXEC";

pub fn connect_midi_server() {
    static CONNECTED: OnceLock<()> = OnceLock::new();
    CONNECTED.get_or_init(
        || match MidiClient::new("coremidi-rs test server connection") {
            Ok(client) => std::mem::forget(client),
            Err(MidiError::Status(MidiStatus::OsStatus(-304) | MidiStatus::ServerStart))
                if std::env::var_os(REEXEC_ENV).is_none() =>
            {
                let error = Command::new(std::env::current_exe().expect("test binary path"))
                    .args(std::env::args_os().skip(1))
                    .env(REEXEC_ENV, "1")
                    .exec();
                panic!("re-running the test binary failed: {error}");
            }
            Err(error) => panic!("CoreMIDI client creation failed: {error}"),
        },
    );
}
