mod common;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use apple_cf::cf::CFRunLoop;
use coremidi::prelude::*;

fn spin_until(mut condition: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        if Instant::now() >= deadline {
            return false;
        }
        let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(10), false);
    }
    true
}

fn spin_for(duration: Duration) {
    let deadline = Instant::now() + duration;
    while Instant::now() < deadline {
        let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(10), false);
    }
}

const fn is_added(notification: &Notification, endpoint: MidiEndpoint) -> bool {
    matches!(notification, Notification::ObjectAdded { child, .. } if *child == endpoint.raw())
}

#[test]
fn notifications_reach_live_handlers_and_streams_only() -> MidiResult<()> {
    common::connect_midi_server();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&seen);
    let watcher = MidiClient::with_notification_handler(
        "notification delivery watcher",
        move |notification| sink.lock().expect("notification log").push(notification),
    )?;
    let publisher = MidiClient::new("notification delivery publisher")?;

    let first = publisher.virtual_source("notification delivery first")?;
    assert!(spin_until(|| seen
        .lock()
        .expect("notification log")
        .iter()
        .any(|notification| is_added(notification, first.endpoint()))));

    #[cfg(feature = "async")]
    streams::deliver_notifications(&publisher)?;

    drop(watcher);
    let delivered = seen.lock().expect("notification log").len();
    let second = publisher.virtual_source("notification delivery second")?;
    spin_for(Duration::from_millis(300));
    assert_eq!(seen.lock().expect("notification log").len(), delivered);
    drop((first, second));

    #[cfg(feature = "async")]
    streams::drop_streams_while_notifications_run()?;
    Ok(())
}

#[cfg(feature = "async")]
mod streams {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc};
    use std::thread;
    use std::time::Duration;

    use apple_cf::cf::CFRunLoop;
    use coremidi::prelude::*;
    use coremidi::thru_connection::{ThruConnection, ThruConnectionEndpoint, ThruConnectionParams};
    use coremidi::{
        MidiClientNotificationStream, MidiThruConnectionStream, MidiVirtualDestinationStream,
    };

    use super::{is_added, spin_for, spin_until};

    pub fn deliver_notifications(publisher: &MidiClient) -> MidiResult<()> {
        let notifications =
            MidiClientNotificationStream::subscribe("notification delivery stream", 64)?;
        let thru = MidiThruConnectionStream::subscribe("notification delivery thru", 8)?;

        let source = publisher.virtual_source("notification delivery stream source")?;
        let mut received = Vec::new();
        assert!(spin_until(|| {
            received.extend(std::iter::from_fn(|| notifications.try_next()));
            received
                .iter()
                .any(|notification| is_added(notification, source.endpoint()))
        }));

        let destination = MidiVirtualDestinationStream::create(
            publisher.raw(),
            "notification delivery thru destination",
            MidiProtocol::Midi1,
            4,
        )?;
        let params = ThruConnectionParams {
            sources: vec![ThruConnectionEndpoint {
                endpoint_ref: source.raw(),
                unique_id: 0,
            }],
            destinations: vec![ThruConnectionEndpoint {
                endpoint_ref: destination.endpoint(),
                unique_id: 0,
            }],
            ..ThruConnectionParams::default()
        };
        let connection = ThruConnection::create(None, &params)?;
        assert!(spin_until(|| thru.try_next().is_some()));
        drop(connection);
        Ok(())
    }

    pub fn drop_streams_while_notifications_run() -> MidiResult<()> {
        let (streams, pending) =
            mpsc::channel::<(MidiClientNotificationStream, MidiThruConnectionStream)>();
        let dropper = thread::spawn(move || {
            for pair in pending {
                thread::sleep(Duration::from_micros(300));
                drop(pair);
            }
        });
        let publishing = Arc::new(AtomicBool::new(true));
        let still_publishing = Arc::clone(&publishing);
        let publisher = thread::spawn(move || -> MidiResult<usize> {
            let client = MidiClient::new("notification stress publisher")?;
            let mut published = 0;
            while still_publishing.load(Ordering::SeqCst) && published < 60 {
                drop(client.virtual_source(&format!("notification stress {published}"))?);
                published += 1;
                thread::sleep(Duration::from_millis(5));
            }
            Ok(published)
        });

        for index in 0..40 {
            let notifications = MidiClientNotificationStream::subscribe(
                &format!("stress notifications {index}"),
                4,
            )?;
            let thru = MidiThruConnectionStream::subscribe(&format!("stress thru {index}"), 4)?;
            let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(2), false);
            streams
                .send((notifications, thru))
                .expect("dropper thread is running");
            let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(2), false);
        }
        drop(streams);
        assert!(spin_until(|| dropper.is_finished()));
        publishing.store(false, Ordering::SeqCst);
        dropper.join().expect("dropper thread");
        assert!(publisher.join().expect("publisher thread")? > 0);
        spin_for(Duration::from_millis(50));
        Ok(())
    }
}
