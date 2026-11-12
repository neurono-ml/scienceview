//! Column tests: the public shape accessors of a produced column.
//!
//! A consumer reads a produced column through [`SVMoveColumn`] and, when it is
//! sparse, through the [`SVMoveSparseColumn`] accessors that describe the
//! stored entries and the one implicit contribution. These tests read every
//! public accessor on a column a move actually produced, dense and sparse.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_move::{SVMove, SVMoveColumn, SVMoveRequest, SVMoveSequence};

/// Applies one sequence to a dense request and returns the horizontal offsets.
fn sv_dense_offsets(values: &[f64], moves: Vec<SVMove>) -> SVMoveColumn {
    let groups = vec![0_u32; values.len()];
    let request = SVMoveRequest::new(SVColumnView::Dense(values), &groups);
    let moved = SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(&request)
        .expect("the sequence applies");
    moved.horizontal_offsets().clone()
}

/// Applies one sequence to a sparse request and returns the horizontal offsets.
fn sv_sparse_offsets(
    indices: &[usize],
    values: &[f64],
    length: usize,
    moves: Vec<SVMove>,
) -> SVMoveColumn {
    let sparse = SVSparseView::new(indices, values, length).expect("the sparse view is valid");
    let groups = vec![0_u32; length];
    let request = SVMoveRequest::new(SVColumnView::Sparse(sparse), &groups);
    let moved = SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(&request)
        .expect("the sequence applies");
    moved.horizontal_offsets().clone()
}

#[test]
fn a_dense_column_reports_its_dense_shape() {
    let offsets = sv_dense_offsets(&[1.0, 2.0, 3.0], vec![SVMove::shift().amount(0.5).into()]);

    assert!(offsets.is_dense());
    assert!(!offsets.is_sparse());
    assert_eq!(offsets.length(), 3);
    assert_eq!(offsets.implicit_value(), 0.0);
    assert_eq!(offsets.dense_values(), Some(&[0.5, 0.5, 0.5][..]));
    assert_eq!(offsets.stored_entries().count(), 3);
    // A dense column stores every position, so the stored entries cover the
    // whole length and the value at each of them is the produced offset.
    for (index, value) in offsets.stored_entries() {
        assert_eq!(Some(value), offsets.value_at(index));
    }
}

#[test]
fn a_sparse_column_reports_its_stored_and_implicit_shape() {
    let offsets = sv_sparse_offsets(
        &[0, 2, 4],
        &[1.0, 2.0, 3.0],
        6,
        vec![SVMove::jitter().seed(5).into()],
    );

    assert!(offsets.is_sparse());
    assert!(!offsets.is_dense());
    assert_eq!(offsets.length(), 6);
    assert_eq!(
        offsets.dense_values(),
        None,
        "a sparse column is never expanded"
    );
    assert_eq!(offsets.implicit_value(), 0.0);

    let SVMoveColumn::Sparse(sparse) = &offsets else {
        panic!("a sparse input produces a sparse offset column");
    };
    assert_eq!(sparse.length(), 6);
    assert_eq!(sparse.indices(), &[0, 2, 4]);
    assert_eq!(sparse.number_of_stored_values(), 3);
    assert_eq!(sparse.implicit_value(), 0.0);
    // The values accessor lines up with the indices, one offset per stored mark.
    for ((index, value), stored) in sparse
        .indices()
        .iter()
        .copied()
        .zip(sparse.values().iter().copied())
        .zip(offsets.stored_entries())
    {
        assert_eq!(stored, (index, value));
        assert_eq!(Some(value), offsets.value_at(index));
    }
    // An unstored position reads the single implicit contribution.
    assert_eq!(sparse.value_at(1), Some(0.0));
    assert_eq!(sparse.value_at(5), Some(0.0));
}

#[test]
fn a_sparse_shift_has_no_stored_entry_and_one_implicit_contribution() {
    let offsets = sv_sparse_offsets(&[1], &[2.0], 4, vec![SVMove::shift().amount(2.5).into()]);

    let SVMoveColumn::Sparse(sparse) = &offsets else {
        panic!("a sparse input produces a sparse offset column");
    };
    assert_eq!(sparse.number_of_stored_values(), 0);
    assert!(sparse.indices().is_empty());
    assert!(sparse.values().is_empty());
    assert_eq!(sparse.implicit_value(), 2.5);
    assert_eq!(sparse.length(), 4);
    assert_eq!(offsets.value_at(0), Some(2.5));
    assert_eq!(offsets.value_at(3), Some(2.5));
}
