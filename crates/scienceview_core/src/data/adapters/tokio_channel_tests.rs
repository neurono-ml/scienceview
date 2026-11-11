//! Companion tests for the Tokio channel adapter.

use std::time::{Duration, Instant};

use tokio::sync::mpsc;

use crate::data::adapters::SVTokioChannelColumn;
use crate::data::{SVDataError, SVDataSource, SVReactivity, SVRepresentation};

#[test]
fn a_tokio_read_drains_the_batches_already_available() {
    let (sender, receiver) = mpsc::channel(8);
    let source = SVTokioChannelColumn::with_capacity("value", receiver, 4).expect("valid capacity");

    sender.try_send(1.5).expect("the channel has room");
    sender.try_send(2.5).expect("the channel has room");

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(snapshot.representation(), SVRepresentation::Streaming);
    assert_eq!(snapshot.column_view().dense_values().unwrap(), &[1.5, 2.5]);
}

#[test]
fn a_tokio_read_never_waits_for_an_empty_channel() {
    let (_sender, receiver) = mpsc::channel::<f64>(4);
    let source = SVTokioChannelColumn::with_capacity("value", receiver, 4).expect("valid capacity");

    let start = Instant::now();
    let mut snapshot = source.column("value").expect("the column exists");
    assert!(snapshot.column_view().is_empty());
    assert!(
        start.elapsed() < Duration::from_millis(50),
        "a streaming read must never wait for new data, took {:?}",
        start.elapsed()
    );
}

#[test]
fn a_tokio_window_stays_bounded() {
    let (sender, receiver) = mpsc::channel(16);
    let source = SVTokioChannelColumn::with_capacity("value", receiver, 3).expect("valid capacity");

    for sample in 0..9 {
        sender
            .try_send(f64::from(sample))
            .expect("the channel has room");
    }

    let mut snapshot = source.column("value").expect("the column exists");
    assert_eq!(
        snapshot.column_view().dense_values().unwrap(),
        &[6.0, 7.0, 8.0]
    );
}

#[test]
fn a_tokio_source_declares_event_driven_reactivity() {
    let (_sender, receiver) = mpsc::channel::<f64>(1);
    let source = SVTokioChannelColumn::with_capacity("value", receiver, 2).expect("valid capacity");

    assert_eq!(source.reactivity(), SVReactivity::EventDriven);
}

#[test]
fn a_tokio_source_reports_an_unknown_column() {
    let (_sender, receiver) = mpsc::channel::<f64>(1);
    let source = SVTokioChannelColumn::with_capacity("value", receiver, 2).expect("valid capacity");

    assert_eq!(
        source.column("other").err(),
        Some(SVDataError::ColumnNotFound {
            name: "other".to_owned()
        })
    );
}

#[test]
fn a_tokio_source_rejects_a_zero_capacity() {
    let (_sender, receiver) = mpsc::channel::<f64>(1);

    assert_eq!(
        SVTokioChannelColumn::with_capacity("value", receiver, 0).err(),
        Some(SVDataError::InvalidWindowCapacity { capacity: 0 })
    );
}
