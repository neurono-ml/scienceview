//! Sparse boundary tests: a move never allocates one slot per logical point.
//!
//! A sparse column of huge logical length with a handful of stored entries is
//! moved through the sequence, and the produced value, baseline and
//! displacement columns are inspected: each keeps the stored entries and one
//! implicit contribution, never a dense buffer of the logical length.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_move::working_state::{sv_materialisation_count, sv_reset_materialisation_count};
use crate::sv_move::{SVMove, SVMoveColumn, SVMoveRequest, SVMoveSequence};

/// A logical length far larger than any working buffer a move may allocate.
const LOGICAL_LENGTH: usize = 10_000_000;
/// The entries the sparse column actually stores.
const STORED_INDICES: [usize; 4] = [0, 1_000, 5_000_000, 9_999_999];
/// The stored values: two positive, one negative and one beyond the group.
const STORED_VALUES: [f64; 4] = [2.0, 3.0, -1.0, 4.0];

/// Builds a sparse request of the huge logical length.
fn sv_huge_sparse_request<'a>(
    sparse: &'a SVSparseView<'a>,
    group_keys: &'a [u32],
) -> SVMoveRequest<'a> {
    SVMoveRequest::new(SVColumnView::Sparse(*sparse), group_keys)
}

/// Returns the number of entries a produced column stores.
fn sv_stored_count(column: &SVMoveColumn) -> usize {
    column.stored_entries().count()
}

#[test]
fn a_sparse_stack_allocates_only_the_stored_entries() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence = SVMoveSequence::single(SVMove::stack().into()).expect("a valid sequence");

    sv_reset_materialisation_count();
    let moved = sequence.apply(&request).expect("the stack applies");

    let values = moved.values().expect("a stack writes values");
    let baselines = moved.baselines().expect("a stack writes baselines");
    assert_eq!(values.length(), LOGICAL_LENGTH);
    assert_eq!(baselines.length(), LOGICAL_LENGTH);
    assert_eq!(sv_stored_count(values), STORED_VALUES.len());
    assert_eq!(sv_stored_count(baselines), STORED_VALUES.len());
    assert!(
        values.is_sparse(),
        "a sparse input keeps a sparse value column"
    );
    assert!(
        baselines.is_sparse(),
        "a sparse input keeps a sparse baseline column"
    );
    assert_eq!(
        moved.horizontal_offsets().dense_values(),
        None,
        "an untouched displacement column is never dense"
    );
    assert_eq!(sv_stored_count(moved.horizontal_offsets()), 0);

    // One value column was materialised, and it describes four entries.
    assert_eq!(sv_materialisation_count(), 1);

    // The diverging stack still accumulates the stored entries correctly: the
    // positives reach 5 and the negative grows downward from zero.
    assert_eq!(values.value_at(STORED_INDICES[0]), Some(2.0));
    assert_eq!(values.value_at(STORED_INDICES[1]), Some(5.0));
    assert_eq!(baselines.value_at(STORED_INDICES[1]), Some(2.0));
    assert_eq!(values.value_at(STORED_INDICES[2]), Some(-1.0));
    assert_eq!(values.value_at(STORED_INDICES[3]), Some(9.0));
    // An unstored logical position stays the implicit zero.
    assert_eq!(values.value_at(7), Some(0.0));
    assert_eq!(baselines.value_at(7), Some(0.0));
}

#[test]
fn a_sparse_normalize_allocates_only_the_stored_entries() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence = SVMoveSequence::single(SVMove::normalize().into()).expect("a valid sequence");

    let moved = sequence.apply(&request).expect("the normalize applies");

    let values = moved.values().expect("a normalize writes values");
    assert!(values.is_sparse());
    assert_eq!(sv_stored_count(values), STORED_VALUES.len());
    // The positive total is 9 and the negative total is 1.
    let first = values.value_at(STORED_INDICES[0]).expect("stored entry");
    assert!((first - 2.0 / 9.0).abs() < 1e-12);
    assert_eq!(values.value_at(STORED_INDICES[2]), Some(-1.0));
    assert_eq!(values.value_at(1), Some(0.0));
}

#[test]
fn a_sparse_stack_and_normalize_pipeline_stays_sparse() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence = SVMoveSequence::new(vec![SVMove::stack().into(), SVMove::normalize().into()])
        .expect("the documented order");

    let moved = sequence.apply(&request).expect("the pipeline applies");

    let values = moved.values().expect("the pipeline writes values");
    assert!(values.is_sparse());
    assert_eq!(sv_stored_count(values), STORED_VALUES.len());
    let last = values
        .value_at(STORED_INDICES[3])
        .expect("the last stored entry");
    assert!((last - 1.0).abs() < 1e-12);
}
