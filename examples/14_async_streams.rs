//! Example: async event streams (requires `--features async`)
//!
//! Demonstrates `MidiEventStream`, `MidiClientNotificationStream`, and
//! `MidiVirtualDestinationStream` with short timeouts so the example
//! exits cleanly on headless machines.

fn main() {
    #[cfg(not(feature = "async"))]
    {
        eprintln!("This example requires the `async` feature. Run with:");
        eprintln!("  cargo run --example 14_async_streams --features async");
        return;
    }

    #[cfg(feature = "async")]
    pollster::block_on(async_main());
}

#[cfg(feature = "async")]
async fn async_main() {
    use std::thread;
    use std::time::Duration;

    use coremidi::{
        sources, MidiClient, MidiClientNotificationStream, MidiEventStream, MidiProtocol,
        MidiVirtualDestinationStream,
    };

    std::future::ready(()).await;

    let poll_delay = Duration::from_millis(10);

    match MidiClientNotificationStream::subscribe("coremidi async example notifications", 16) {
        Ok(stream) => {
            for attempt in 0..5 {
                if let Some(notification) = stream.try_next() {
                    println!("notification[{attempt}]: {notification:?}");
                    break;
                }
                thread::sleep(poll_delay);
            }
            println!(
                "notification stream ready (buffered={})",
                stream.buffered_count()
            );
        }
        Err(error) => eprintln!("warning: notification stream unavailable: {error}"),
    }

    let client = match MidiClient::new("coremidi async example client") {
        Ok(client) => client,
        Err(error) => {
            eprintln!("warning: MidiClient::new failed: {error}");
            println!("async example complete");
            return;
        }
    };

    if let Some(source) = sources().next() {
        match MidiEventStream::subscribe(client.raw(), source.raw(), MidiProtocol::Midi1, 8) {
            Ok(stream) => {
                println!(
                    "event stream subscribed to source {} (buffered={})",
                    source.raw(),
                    stream.buffered_count()
                );
            }
            Err(error) => eprintln!("warning: event stream unavailable: {error}"),
        }
    } else {
        eprintln!("warning: no MIDI sources available; skipping MidiEventStream demo");
    }

    match MidiVirtualDestinationStream::create(
        client.raw(),
        "coremidi async example destination",
        MidiProtocol::Midi2,
        16,
    ) {
        Ok(stream) => {
            println!(
                "virtual destination endpoint={} buffered={}",
                stream.endpoint(),
                stream.buffered_count()
            );
        }
        Err(error) => eprintln!("warning: virtual destination stream unavailable: {error}"),
    }

    println!("async example complete");
}
