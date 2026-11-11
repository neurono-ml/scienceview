//! Companion tests for the bounded rolling window.

use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use crate::sv_data::SVDataError;
use crate::sv_window::{SVRollingWindow, SVWindowPolicy, SVWindowReadGuard};

fn sv_lock<Value>(mutex: &Mutex<Value>) -> MutexGuard<'_, Value> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[test]
fn a_window_rejects_a_capacity_of_zero() {
    assert_eq!(
        SVRollingWindow::with_capacity(0).err(),
        Some(SVDataError::InvalidWindowCapacity { capacity: 0 })
    );
}

#[test]
fn a_window_retains_samples_in_arrival_order() {
    let mut window = SVRollingWindow::with_capacity(4).expect("valid capacity");
    window.extend([1.0, 2.0, 3.0]);

    assert_eq!(window.snapshot(), &[1.0, 2.0, 3.0]);
    assert_eq!(window.length(), 3);
    assert_eq!(window.sequence(), 3);
    assert_eq!(window.dropped_samples(), 0);
}

#[test]
fn the_drop_oldest_policy_evicts_the_oldest_sample() {
    let mut window = SVRollingWindow::with_capacity(3).expect("valid capacity");
    window.extend([1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_eq!(window.snapshot(), &[3.0, 4.0, 5.0]);
    assert_eq!(window.dropped_samples(), 2);
    assert_eq!(window.total_samples(), 5);
}

#[test]
fn the_drop_newest_policy_rejects_the_newest_sample() {
    let mut window = SVRollingWindow::new(SVWindowPolicy::drop_newest(3)).expect("valid policy");
    window.extend([1.0, 2.0, 3.0, 4.0]);

    assert_eq!(window.snapshot(), &[1.0, 2.0, 3.0]);
    assert_eq!(window.dropped_samples(), 1);
    assert_eq!(window.total_samples(), 3);
}

#[test]
fn a_wrapped_window_still_snapshots_contiguously() {
    let mut window = SVRollingWindow::with_capacity(3).expect("valid capacity");
    window.extend([1.0, 2.0, 3.0, 4.0]);

    assert_eq!(window.snapshot(), &[2.0, 3.0, 4.0]);
}

#[test]
fn a_read_returns_the_current_window_when_nothing_arrives() {
    let shared = Arc::new(Mutex::new(
        SVRollingWindow::with_capacity(4).expect("valid capacity"),
    ));
    sv_lock(&shared).push(7.0);

    let mut read = SVWindowReadGuard::new(sv_lock(&shared));
    assert_eq!(read.values(), &[7.0]);
    assert_eq!(read.sequence(), 1);
    assert_eq!(read.chunks(), (&[7.0][..], &[][..]));
}

#[test]
fn a_read_never_blocks_when_the_producer_is_idle() {
    let shared = Arc::new(Mutex::new(
        SVRollingWindow::with_capacity(4).expect("valid capacity"),
    ));
    sv_lock(&shared).push(1.0);

    let start = Instant::now();
    let mut read = SVWindowReadGuard::new(sv_lock(&shared));
    let values = read.values().to_vec();

    assert_eq!(values, vec![1.0]);
    assert!(
        start.elapsed() < Duration::from_millis(50),
        "a window read must return immediately, took {:?}",
        start.elapsed()
    );
}

#[test]
fn a_producer_thread_and_a_consumer_thread_share_a_window_without_deadlock() {
    let shared = Arc::new(Mutex::new(
        SVRollingWindow::with_capacity(32).expect("valid capacity"),
    ));
    let (finished_sender, finished_receiver) = mpsc::channel();

    let producer = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            for sample in 0..1_000 {
                sv_lock(&shared).push(f64::from(sample));
            }
            finished_sender.send(()).expect("the consumer is alive");
        })
    };

    let consumer = {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let mut reads = 0_u32;
            let mut last_sequence = 0_u64;
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                let mut read = SVWindowReadGuard::new(sv_lock(&shared));
                let sequence = read.sequence();
                let length = read.values().len();
                assert!(length <= 32, "the window must stay bounded");
                assert!(sequence >= last_sequence, "the sequence never decreases");
                last_sequence = sequence;
                reads += 1;

                if finished_receiver.try_recv().is_ok() {
                    break;
                }
                assert!(
                    Instant::now() < deadline,
                    "the consumer must not wait forever for the producer"
                );
            }
            reads
        })
    };

    producer.join().expect("the producer did not panic");
    let reads = consumer.join().expect("the consumer did not panic");
    assert!(reads > 0, "the consumer must have observed the window");
    assert_eq!(sv_lock(&shared).total_samples(), 1_000);
    assert!(sv_lock(&shared).dropped_samples() > 0);
}

#[test]
fn many_threads_read_the_same_window_concurrently() {
    let shared = Arc::new(Mutex::new(
        SVRollingWindow::with_capacity(8).expect("valid capacity"),
    ));
    sv_lock(&shared).extend([1.0, 2.0, 3.0]);

    let readers: Vec<_> = (0..8)
        .map(|_| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let mut read = SVWindowReadGuard::new(sv_lock(&shared));
                read.values().to_vec()
            })
        })
        .collect();

    for reader in readers {
        assert_eq!(
            reader.join().expect("the reader did not panic"),
            [1.0, 2.0, 3.0]
        );
    }
}
