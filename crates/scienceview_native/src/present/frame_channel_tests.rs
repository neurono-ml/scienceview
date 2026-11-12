//! The frame channel: one pending frame, the newest wins, and publishing wakes
//! the consumer.

use super::SVFrameChannel;
use crate::canvas::SVNativeFrame;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn sv_solid_frame(value: u8) -> SVNativeFrame {
    SVNativeFrame::new(2, 2, 1.0, vec![value; 2 * 2 * 4])
}

#[test]
fn a_published_frame_waits_for_the_consumer_and_is_taken_once() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    let frame = sv_solid_frame(10);

    publisher.publish(frame.clone());

    assert!(channel.has_pending());
    assert_eq!(channel.take(), Some(frame));
    assert!(!channel.has_pending());
    assert_eq!(channel.take(), None);
}

#[test]
fn publishing_wakes_the_consumer_once_per_frame() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    let wakeups = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&wakeups);
    channel.set_waker(Arc::new(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    }));

    publisher.publish(sv_solid_frame(1));
    publisher.publish(sv_solid_frame(2));

    assert_eq!(wakeups.load(Ordering::SeqCst), 2);
}

#[test]
fn only_the_newest_frame_survives_a_fast_producer() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();

    publisher.publish(sv_solid_frame(1));
    publisher.publish(sv_solid_frame(9));

    let newest = channel.take().expect("the newest frame is pending");
    assert_eq!(newest, sv_solid_frame(9));
    assert_eq!(channel.take(), None);
}

#[test]
fn a_publisher_outlives_the_consumer_handle() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    drop(channel);

    publisher.publish(sv_solid_frame(3));
}
