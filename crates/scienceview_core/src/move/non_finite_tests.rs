//! Non-finite inputs: every move rejects NaN and both infinities up front.
//!
//! A stack or a normalisation would otherwise propagate a NaN silently through
//! every top and baseline it touches. The move sequence validates the stored
//! values once, before any move reads them, and returns the typed
//! configuration error naming the position and the value.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence};

/// Builds one move of each kind, with the options each needs to be valid.
fn sv_moves() -> [SVMove; 6] {
    [
        SVMove::stack().into(),
        SVMove::dodge().into(),
        SVMove::jitter().seed(1).into(),
        SVMove::shift().amount(0.25).into(),
        SVMove::swarm().marker_extent(1.0).into(),
        SVMove::normalize().into(),
    ]
}

/// Asserts that one move rejects a non-finite value at the known position.
fn assert_rejected(single_move: &SVMove, value: f64, expected_position: usize) {
    let values = [1.0, value, 3.0];
    let groups = [0, 0, 0];
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
    let result = SVMoveSequence::single(single_move.clone())
        .expect("the move configuration is valid")
        .apply(&request);

    match result {
        Err(SVError::InvalidConfiguration { message }) => {
            assert!(
                message.contains(&format!("position {expected_position}")),
                "the message names the position: {message}"
            );
        }
        other => panic!("expected a typed configuration error for {value}, received {other:?}"),
    }
}

#[test]
fn every_move_rejects_nan() {
    for single_move in sv_moves() {
        assert_rejected(&single_move, f64::NAN, 1);
    }
}

#[test]
fn every_move_rejects_positive_infinity() {
    for single_move in sv_moves() {
        assert_rejected(&single_move, f64::INFINITY, 1);
    }
}

#[test]
fn every_move_rejects_negative_infinity() {
    for single_move in sv_moves() {
        assert_rejected(&single_move, f64::NEG_INFINITY, 1);
    }
}

#[test]
fn a_sparse_move_rejects_a_non_finite_stored_value_and_names_it() {
    let indices = [1_usize];
    let values = [f64::INFINITY];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("the sparse view is valid");
    let groups = [0, 0, 0, 0];
    let request = SVMoveRequest::new(SVColumnView::Sparse(sparse), &groups);
    let result = SVMoveSequence::single(SVMove::stack().into())
        .expect("the configuration is valid")
        .apply(&request);

    match result {
        Err(SVError::InvalidConfiguration { message }) => {
            assert!(message.contains("position 1"), "{message}");
        }
        other => panic!("expected a typed configuration error, received {other:?}"),
    }
}

#[test]
fn a_finite_zero_is_accepted_by_every_move() {
    for single_move in sv_moves() {
        let values = [1.0, 0.0, 3.0];
        let groups = [0, 0, 0];
        let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
        SVMoveSequence::single(single_move)
            .expect("the move configuration is valid")
            .apply(&request)
            .expect("a finite zero is a legal value");
    }
}
