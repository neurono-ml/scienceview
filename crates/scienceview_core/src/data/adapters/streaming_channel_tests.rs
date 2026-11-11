//! Companion tests for the standard-library channel adapter.

use crate::sv_error::SVError;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::sv_data::adapters::SVChannelColumn;
use crate::sv_data::{SVDataSource, SVReactivity, SVRepresentation};

#[test]
fn a_read_drains_every_sample_already_available() {
    let (sender, receiver) = mpsc::channel();
    let source = SVChannelColumn::with_capacity("value", receiver, 8).expect("valid capacity");

    sender.send(1.0).expect("the source is alive");
    sender.send(2.0).expect("the source is alive");

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Streaming);
    assert_eq!(snapshot.column_view().dense_values().unwrap(), &[1.0, 2.0]);
}

#[test]
fn a_read_returns_the_current_window_when_no_data_arrives() {
    let (_sender, receiver) = mpsc::channel::<f64>();
    let source = SVChannelColumn::with_capacity("value", receiver, 4).expect("valid capacity");

    let start = Instant::now();
    let mut snapshot = source.column("value").expect("the column exists");
    let values = snapshot.column_view().length();

    assert_eq!(values, 0);
    assert!(
        start.elapsed() < Duration::from_millis(50),
        "a streaming read must never wait for new data, took {:?}",
        start.elapsed()
    );
}

#[test]
fn an_overflowing_window_drops_the_oldest_samples() {
    let (sender, receiver) = mpsc::channel();
    let source = SVChannelColumn::with_capacity("value", receiver, 3).expect("valid capacity");

    for sample in 0..10 {
        sender.send(f64::from(sample)).expect("the source is alive");
    }

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(
        snapshot.column_view().dense_values().unwrap(),
        &[7.0, 8.0, 9.0]
    );
}

#[test]
fn a_channel_source_declares_event_driven_reactivity() {
    let (_sender, receiver) = mpsc::channel::<f64>();
    let source = SVChannelColumn::with_capacity("value", receiver, 4).expect("valid capacity");

    assert_eq!(source.reactivity(), SVReactivity::EventDriven);
    assert_eq!(
        source.schema().column("value").map(|value| value.name()),
        Some("value")
    );
}

#[test]
fn a_channel_source_rejects_an_unknown_column() {
    let (_sender, receiver) = mpsc::channel::<f64>();
    let source = SVChannelColumn::with_capacity("value", receiver, 4).expect("valid capacity");

    assert_eq!(
        source.column("other").err(),
        Some(SVError::ColumnNotFound {
            name: "other".to_owned()
        })
    );
}

#[test]
fn a_channel_source_rejects_a_zero_capacity() {
    let (_sender, receiver) = mpsc::channel::<f64>();

    assert_eq!(
        SVChannelColumn::with_capacity("value", receiver, 0).err(),
        Some(SVError::InvalidWindowCapacity { capacity: 0 })
    );
}

#[test]
fn a_producer_thread_and_a_consumer_thread_never_deadlock() {
    let (sender, receiver) = mpsc::channel();
    let source = SVChannelColumn::with_capacity("value", receiver, 16).expect("valid capacity");

    thread::scope(|scope| {
        let producer = scope.spawn(move || {
            for sample in 0..2_000 {
                sender.send(f64::from(sample)).expect("the source is alive");
            }
        });

        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let mut snapshot = source.column("value").expect("the column exists");
            let length = snapshot.column_view().length();
            assert!(length <= 16, "the window must stay bounded");

            if producer.is_finished() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "the consumer must not wait forever for the producer"
            );
        }
        producer.join().expect("the producer did not panic");
    });

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(
        snapshot.column_view().length(),
        16,
        "the window keeps the last samples after the producer disconnects"
    );
}
