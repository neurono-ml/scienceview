//! Lazy buffers: the purely positional moves never copy the value column.
//!
//! Dodge, jitter and shift read the request's own column through its
//! representation-aware accessor and share it; they materialise only the
//! displacement or width they produce. The test-only counters make that a
//! property the suite can assert instead of a comment.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_move::working_state::{
    sv_materialisation_count, sv_reset_materialisation_count, sv_resolution_count,
};
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence};

/// Applies one sequence to a dense request and returns the output.
fn sv_apply_dense(values: &[f64], group_keys: &[u32], moves: Vec<SVMove>) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys);
    SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(&request)
        .expect("the sequence applies")
}

/// Applies one sequence to a sparse request and returns the output.
fn sv_apply_sparse(
    sparse: &SVSparseView<'_>,
    group_keys: &[u32],
    moves: Vec<SVMove>,
) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Sparse(*sparse), group_keys);
    SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(&request)
        .expect("the sequence applies")
}

/// Asserts that a purely positional move copied no value column.
fn assert_no_value_copy(output: &SVMoveOutput, number_of_points: usize) {
    assert_eq!(
        sv_materialisation_count(),
        0,
        "a positional move must share the borrowed value column"
    );
    assert_eq!(
        sv_resolution_count(),
        1,
        "the representation is resolved exactly once"
    );
    assert!(output.values().is_none());
    assert!(output.baselines().is_none());
    assert_eq!(output.number_of_points(), number_of_points);
}

#[test]
fn a_dodge_shares_the_borrowed_dense_column() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let groups = [0, 1, 0, 1];
    let categories = [0, 0, 1, 1];
    sv_reset_materialisation_count();

    let request =
        SVMoveRequest::new(SVColumnView::Dense(&values), &groups).with_category_keys(&categories);
    let moved = SVMoveSequence::single(SVMove::dodge().into())
        .expect("a valid sequence")
        .apply(&request)
        .expect("the dodge applies");

    assert_no_value_copy(&moved, values.len());
    // Only the displacement and the width are materialised, one per mark.
    assert_eq!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement")
            .len(),
        values.len()
    );
    assert_eq!(
        moved.dense_widths().expect("a dense width").len(),
        values.len()
    );
}

#[test]
fn a_dodge_shares_the_borrowed_sparse_column() {
    let values = [2.0, 3.0];
    let indices = [1_usize, 3];
    let sparse = SVSparseView::new(&indices, &values, 5).expect("the sparse view is valid");
    let groups = [0, 0, 1, 1, 0];
    sv_reset_materialisation_count();

    let moved = sv_apply_sparse(&sparse, &groups, vec![SVMove::dodge().into()]);

    assert_no_value_copy(&moved, 5);
}

#[test]
fn a_jitter_shares_the_borrowed_dense_column() {
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 1];
    sv_reset_materialisation_count();

    let moved = sv_apply_dense(&values, &groups, vec![SVMove::jitter().seed(5).into()]);

    assert_no_value_copy(&moved, values.len());
}

#[test]
fn a_jitter_shares_the_borrowed_sparse_column() {
    let values = [2.0, 3.0];
    let indices = [1_usize, 3];
    let sparse = SVSparseView::new(&indices, &values, 5).expect("the sparse view is valid");
    let groups = [0, 0, 1, 1, 0];
    sv_reset_materialisation_count();

    let moved = sv_apply_sparse(&sparse, &groups, vec![SVMove::jitter().seed(5).into()]);

    assert_no_value_copy(&moved, 5);
}

#[test]
fn a_shift_shares_the_borrowed_dense_column() {
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    sv_reset_materialisation_count();

    let moved = sv_apply_dense(&values, &groups, vec![SVMove::shift().amount(0.5).into()]);

    assert_no_value_copy(&moved, values.len());
    assert_eq!(
        moved.dense_horizontal_offsets().expect("a dense shift"),
        &[0.5, 0.5, 0.5]
    );
}

#[test]
fn a_vertical_shift_shares_the_borrowed_sparse_column() {
    let values = [2.0];
    let indices = [2_usize];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("the sparse view is valid");
    let groups = [0, 0, 0, 0];
    sv_reset_materialisation_count();

    let moved = sv_apply_sparse(
        &sparse,
        &groups,
        vec![
            SVMove::shift()
                .amount(-1.0)
                .direction(crate::sv_move::SVShiftDirection::Vertical)
                .into(),
        ],
    );

    assert_no_value_copy(&moved, 4);
    // The untouched horizontal column keeps the sparse shape of the input.
    assert!(moved.horizontal_offsets().is_sparse());
    assert_eq!(moved.horizontal_offsets().stored_entries().count(), 0);
}

#[test]
fn the_empty_sequence_materialises_nothing() {
    let values = [1.0, 2.0];
    let groups = [0, 0];
    sv_reset_materialisation_count();

    let moved = sv_apply_dense(&values, &groups, Vec::new());

    assert_no_value_copy(&moved, values.len());
}
