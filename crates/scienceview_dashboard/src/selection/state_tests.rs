//! The shared filter state: idempotence, cascades and clearing.

use crate::sv_control::SVControlRange;
use crate::sv_error::SVError;
use crate::sv_selection::{SVLinkedSelectionState, SVSelectionConstraint, SVSelectionFilter};

fn sv_filter() -> SVSelectionFilter {
    SVSelectionFilter::new(
        "measurements",
        SVSelectionConstraint::new(
            "x",
            SVControlRange::new(1.0, 3.0).expect("an ordered range"),
        )
        .expect("a column"),
        None,
    )
    .expect("the filter is well formed")
}

fn sv_filter_with_horizontal_range(lower: f64, upper: f64) -> SVSelectionFilter {
    SVSelectionFilter::new(
        "measurements",
        SVSelectionConstraint::new(
            "x",
            SVControlRange::new(lower, upper).expect("an ordered range"),
        )
        .expect("a column"),
        None,
    )
    .expect("the filter is well formed")
}

#[test]
fn a_repeated_filter_is_a_no_op_and_never_reaches_the_host_twice() {
    let mut state = SVLinkedSelectionState::new(4);
    let filter = sv_filter();

    let first = state
        .apply(filter.clone())
        .expect("the first filter applies");
    let second = state.apply(filter).expect("the repeated filter is a no-op");

    assert!(first.changed());
    assert!(!second.changed());
    assert_eq!(state.number_of_filters(), 1);
}

#[test]
fn a_cascade_deeper_than_the_budget_is_stopped_with_a_cycle_error() {
    let mut state = SVLinkedSelectionState::new(2);
    let chain = [
        sv_filter(),
        sv_filter_with_horizontal_range(2.0, 4.0),
        sv_filter_with_horizontal_range(3.0, 5.0),
    ];

    let error = state
        .apply_cascade(&chain)
        .expect_err("the third step exceeds the budget");

    assert!(matches!(
        error,
        SVError::LinkedSelectionCycle {
            depth: 3,
            budget: 2
        }
    ));
}

#[test]
fn a_reappearing_filter_ends_the_cascade_before_the_budget() {
    let mut state = SVLinkedSelectionState::new(8);
    let first = sv_filter();
    let looped = [
        first.clone(),
        sv_filter_with_horizontal_range(2.0, 4.0),
        first,
    ];

    let cascade = state
        .apply_cascade(&looped)
        .expect("the cycle is suppressed");

    assert_eq!(cascade.number_of_applied_steps(), 2);
}

#[test]
fn clearing_removes_the_filter_from_the_exposed_state() {
    let mut state = SVLinkedSelectionState::new(4);
    state.apply(sv_filter()).expect("the filter applies");

    let cleared = state.clear("measurements");

    assert!(cleared.changed());
    assert!(state.filter_for("measurements").is_none());
    assert!(state.is_empty());
}
