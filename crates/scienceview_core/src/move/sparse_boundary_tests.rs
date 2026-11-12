//! Sparse boundary tests: a move never allocates one slot per logical point.
//!
//! A sparse column of huge logical length with a handful of stored entries is
//! moved through the sequence, and the produced value, baseline and
//! displacement columns are inspected: each keeps the stored entries and one
//! implicit contribution, never a dense buffer of the logical length.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_move::working_state::{sv_materialisation_count, sv_reset_materialisation_count};
use crate::sv_move::{SVMove, SVMoveColumn, SVMoveRequest, SVMoveSequence};
use std::time::{Duration, Instant};

/// A logical length far larger than any working buffer a move may allocate.
const LOGICAL_LENGTH: usize = 10_000_000;
/// The entries the sparse column actually stores.
const STORED_INDICES: [usize; 4] = [0, 1_000, 5_000_000, 9_999_999];
/// The stored values: two positive, one negative and one beyond the group.
const STORED_VALUES: [f64; 4] = [2.0, 3.0, -1.0, 4.0];
/// A tight cluster of stored values, so a swarm must actually displace them.
const CLUSTERED_VALUES: [f64; 4] = [0.4, 0.45, 0.5, 0.55];
/// The longest one sparse move over [`LOGICAL_LENGTH`] may take.
const MAXIMUM_ELAPSED: Duration = Duration::from_secs(5);

/// Asserts that a move over the huge sparse column finished in bounded time.
fn sv_assert_bounded_run_time(started: Instant, label: &str) {
    let elapsed = started.elapsed();
    assert!(
        elapsed < MAXIMUM_ELAPSED,
        "{label} took {elapsed:?}, longer than the {MAXIMUM_ELAPSED:?} bound: it visited logical positions instead of stored entries"
    );
}

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

/// Builds group codes that cycle over three groups across the logical length.
fn sv_patterned_group_keys() -> Vec<u32> {
    (0..LOGICAL_LENGTH)
        .map(|index| (index % 3) as u32)
        .collect()
}

#[test]
fn a_sparse_shift_expresses_the_uniform_displacement_implicitly() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence =
        SVMoveSequence::single(SVMove::shift().amount(2.5).into()).expect("a valid sequence");

    let started = Instant::now();
    let moved = sequence.apply(&request).expect("the shift applies");
    sv_assert_bounded_run_time(started, "a sparse shift");

    let offsets = moved.horizontal_offsets();
    assert!(offsets.is_sparse());
    assert_eq!(
        sv_stored_count(offsets),
        0,
        "a uniform displacement costs no stored slot"
    );
    assert_eq!(
        offsets.implicit_value(),
        2.5,
        "the displacement lives in the implicit contribution"
    );
    // Every logical position, stored or implicit, carries the displacement.
    assert_eq!(offsets.value_at(7), Some(2.5));
    assert_eq!(offsets.value_at(STORED_INDICES[2]), Some(2.5));
    assert_eq!(moved.horizontal_offset(LOGICAL_LENGTH - 2), Some(2.5));
    assert_eq!(
        offsets.length(),
        LOGICAL_LENGTH,
        "the shift keeps the logical length"
    );
}

#[test]
fn a_sparse_jitter_displaces_only_the_stored_entries() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = sv_patterned_group_keys();
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence =
        SVMoveSequence::single(SVMove::jitter().seed(5).into()).expect("a valid sequence");

    let started = Instant::now();
    let moved = sequence.apply(&request).expect("the jitter applies");
    sv_assert_bounded_run_time(started, "a sparse jitter");

    let offsets = moved.horizontal_offsets();
    assert!(offsets.is_sparse());
    assert_eq!(
        sv_stored_count(offsets),
        STORED_VALUES.len(),
        "only the stored entries are displaced"
    );
    assert_eq!(
        offsets.implicit_value(),
        0.0,
        "an unstored position is not a mark and keeps the zero displacement"
    );
    assert_eq!(offsets.value_at(7), Some(0.0));
}

#[test]
fn a_sparse_dodge_narrows_only_the_stored_entries() {
    let sparse = SVSparseView::new(&STORED_INDICES, &STORED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = sv_patterned_group_keys();
    let category_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys).with_category_keys(&category_keys);
    let sequence = SVMoveSequence::single(SVMove::dodge().into()).expect("a valid sequence");

    let started = Instant::now();
    let moved = sequence.apply(&request).expect("the dodge applies");
    sv_assert_bounded_run_time(started, "a sparse dodge");

    let offsets = moved.horizontal_offsets();
    let widths = moved.widths().expect("a dodge narrows the width");
    assert!(offsets.is_sparse());
    assert!(widths.is_sparse());
    assert_eq!(sv_stored_count(offsets), STORED_VALUES.len());
    assert_eq!(sv_stored_count(widths), STORED_VALUES.len());
    // Three groups in one category split the default 0.8 slot.
    let slot = 0.8 / 3.0;
    assert!((widths.value_at(STORED_INDICES[0]).expect("stored width") - slot).abs() < 1e-6);
    assert!(
        (offsets.value_at(STORED_INDICES[0]).expect("stored offset") - (-0.4 + slot * 0.5)).abs()
            < 1e-6
    );
    assert!(
        (offsets.value_at(STORED_INDICES[1]).expect("stored offset") - (-0.4 + slot * 1.5)).abs()
            < 1e-6
    );
    // An unstored position is not a mark: no offset and no narrowed width.
    assert_eq!(offsets.value_at(7), Some(0.0));
    assert_eq!(widths.value_at(7), Some(0.0));
}

#[test]
fn a_sparse_swarm_lays_out_only_the_stored_entries() {
    let sparse = SVSparseView::new(&STORED_INDICES, &CLUSTERED_VALUES, LOGICAL_LENGTH)
        .expect("the sparse view is valid");
    let group_keys = vec![0_u32; LOGICAL_LENGTH];
    let request = sv_huge_sparse_request(&sparse, &group_keys);
    let sequence = SVMoveSequence::single(SVMove::swarm().marker_extent(0.2).into())
        .expect("a valid sequence");

    let started = Instant::now();
    let moved = sequence.apply(&request).expect("the swarm applies");
    sv_assert_bounded_run_time(started, "a sparse swarm");

    let offsets = moved.horizontal_offsets();
    assert!(offsets.is_sparse());
    assert_eq!(
        sv_stored_count(offsets),
        CLUSTERED_VALUES.len(),
        "only the stored entries are placed"
    );
    // The implicit zero keeps its own slot and is never displaced itself.
    assert_eq!(offsets.value_at(7), Some(0.0));
    // The cluster is spread: at least one stored entry leaves the zero offset.
    let number_of_displacements = STORED_INDICES
        .iter()
        .filter(|index| {
            offsets
                .value_at(**index)
                .is_some_and(|offset| offset.abs() > 0.0)
        })
        .count();
    assert!(
        number_of_displacements > 0,
        "a swarm over a tight cluster must displace stored entries"
    );
}
