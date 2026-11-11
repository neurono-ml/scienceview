//! Companion tests for the window rollover policy.

use crate::window::{SVWindowOverflow, SVWindowPolicy};

#[test]
fn drop_oldest_policy_keeps_the_newest_samples() {
    let policy = SVWindowPolicy::drop_oldest(16);

    assert_eq!(policy.maximum_number_of_points(), 16);
    assert_eq!(policy.overflow(), SVWindowOverflow::DropOldest);
}

#[test]
fn drop_newest_policy_keeps_the_first_samples() {
    let policy = SVWindowPolicy::drop_newest(8);

    assert_eq!(policy.maximum_number_of_points(), 8);
    assert_eq!(policy.overflow(), SVWindowOverflow::DropNewest);
}

#[test]
fn the_default_policy_is_bounded_and_drops_the_oldest() {
    let policy = SVWindowPolicy::default();

    assert!(policy.maximum_number_of_points() > 0);
    assert_eq!(policy.overflow(), SVWindowOverflow::DropOldest);
}
