//! Swarm tests: non-overlap, determinism and a finite run time.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence};
use std::time::Instant;

/// Applies one swarm to borrowed values and group codes.
fn sv_apply_swarm_to(values: &[f64], group_keys: &[u32], single_move: SVMove) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys);
    SVMoveSequence::single(single_move)
        .expect("a single swarm is a valid sequence")
        .apply(&request)
        .expect("the swarm applies")
}

/// Returns `true` when two placed points are closer than one marker in both
/// axes, which is the documented definition of an overlap.
fn sv_overlaps(
    left_value: f64,
    left_offset: f64,
    right_value: f64,
    right_offset: f64,
    marker_extent: f64,
) -> bool {
    (left_value - right_value).abs() < marker_extent
        && (left_offset - right_offset).abs() < marker_extent
}

#[test]
fn well_separated_values_stay_on_the_axis() {
    let values = [0.0, 10.0, 20.0, 30.0];
    let groups = [0; 4];
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(1.0).into());

    assert_eq!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.0, 0.0, 0.0]
    );
}

#[test]
fn identical_values_are_spread_apart() {
    let values = [5.0, 5.0, 5.0, 5.0];
    let groups = [0; 4];
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(1.0).into());

    let offsets = moved
        .dense_horizontal_offsets()
        .expect("a dense displacement");
    assert_eq!(offsets, &[0.0, 1.0, -1.0, 2.0]);
}

#[test]
fn no_two_points_of_a_category_overlap() {
    let values: Vec<f64> = (0..60).map(|step| f64::from(step) * 0.1).collect();
    let groups = vec![0_u32; values.len()];
    let marker_extent = 0.5;
    let moved = sv_apply_swarm_to(
        &values,
        &groups,
        SVMove::swarm().marker_extent(marker_extent).into(),
    );

    for left in 0..values.len() {
        for right in (left + 1)..values.len() {
            assert!(
                !sv_overlaps(
                    values[left],
                    moved
                        .dense_horizontal_offsets()
                        .expect("a dense displacement")[left],
                    values[right],
                    moved
                        .dense_horizontal_offsets()
                        .expect("a dense displacement")[right],
                    marker_extent,
                ),
                "points {left} and {right} overlap"
            );
        }
    }
}

#[test]
fn categories_are_laid_out_independently() {
    let values = [5.0, 5.0];
    let groups = [0, 1];
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(1.0).into());

    assert_eq!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.0]
    );
}

#[test]
fn the_layout_is_deterministic() {
    let values = [1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 0.5];
    let groups = [0, 0, 0, 0, 0, 0, 0];
    let swarm: SVMove = SVMove::swarm().marker_extent(0.5).into();

    let first = sv_apply_swarm_to(&values, &groups, swarm.clone());
    let second = sv_apply_swarm_to(&values, &groups, swarm);

    assert_eq!(
        first
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        second
            .dense_horizontal_offsets()
            .expect("a dense displacement")
    );
}

#[test]
fn the_width_bounds_the_displacement() {
    // Five identical points need the ranks 0, +1, -1, +2 and -2; a width of two
    // marker extents offers exactly those candidates and no more.
    let values = [5.0; 5];
    let groups = [0; 5];
    let moved = sv_apply_swarm_to(
        &values,
        &groups,
        SVMove::swarm().marker_extent(0.5).width(2.0).into(),
    );

    assert_eq!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.5, -0.5, 1.0, -1.0]
    );
}

#[test]
fn a_width_narrower_than_two_marker_extents_is_rejected() {
    let result = SVMoveSequence::single(SVMove::swarm().marker_extent(0.5).width(0.9).into());

    assert!(
        matches!(
            result,
            Err(crate::sv_error::SVError::InvalidConfiguration { .. })
        ),
        "a width below two marker extents leaves a colliding point no candidate"
    );

    // Exactly two marker extents is the documented narrowest valid bound.
    SVMoveSequence::single(SVMove::swarm().marker_extent(0.5).width(1.0).into())
        .expect("two marker extents are enough");
}

#[test]
fn a_dense_cluster_that_fits_still_succeeds() {
    let values = [5.0; 5];
    let groups = [0; 5];
    let moved = sv_apply_swarm_to(
        &values,
        &groups,
        SVMove::swarm().marker_extent(0.5).width(2.0).into(),
    );

    assert_eq!(moved.number_of_points(), 5);
    for offset in moved
        .dense_horizontal_offsets()
        .expect("a dense displacement")
    {
        assert!(offset.abs() <= 1.0 + 1e-12);
    }
}

#[test]
fn a_cluster_the_bounded_search_cannot_place_is_a_typed_error() {
    // Six identical points need the rank +3, which a width of two marker
    // extents does not offer. The layout must refuse instead of reusing a
    // colliding candidate.
    let values = [5.0; 6];
    let groups = [0; 6];
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
    let result = SVMoveSequence::single(SVMove::swarm().marker_extent(0.5).width(2.0).into())
        .expect("the configuration is valid")
        .apply(&request);

    match result {
        Err(crate::sv_error::SVError::InvalidConfiguration { message }) => {
            assert!(
                message.contains("collision"),
                "the message names the cause: {message}"
            );
            assert!(
                message.contains("group 0"),
                "the message names the group: {message}"
            );
        }
        other => panic!("expected a typed configuration error, received {other:?}"),
    }
}

#[test]
fn the_layout_finishes_in_bounded_time() {
    let mut generator_state = 17_u64;
    let values: Vec<f64> = (0..2000)
        .map(|_| {
            let word = crate::sv_move::sv_split_mix_64(&mut generator_state);
            (word >> 11) as f64 / 9_007_199_254_740_992.0
        })
        .collect();
    let groups = vec![0_u32; values.len()];

    let started = Instant::now();
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(0.01).into());
    let elapsed = started.elapsed();

    assert_eq!(moved.number_of_points(), values.len());
    assert!(
        elapsed.as_secs_f64() < 10.0,
        "the swarm took {elapsed:?}, which leaves the bounded-time budget"
    );
}

#[test]
fn a_swarm_never_changes_values() {
    let values = [1.0, 1.0];
    let groups = [0, 0];
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(1.0).into());

    assert!(!moved.values_changed());
    assert!(moved.values().is_none());
    assert!(moved.widths().is_none());
    assert_eq!(
        moved
            .dense_vertical_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.0]
    );
}

#[test]
fn an_implicit_zero_reserves_its_slot_in_a_sparse_layout() {
    // The unstored position is an implicit zero, and the dense twin lays that
    // zero out. The sparse layout therefore reserves the zero slot: the two
    // stored marks cannot both sit on zero, and the middle position stays the
    // implicit zero without a stored offset of its own.
    let stored_indices = [0_usize, 2];
    let stored_values = [0.4, 0.6];
    let sparse =
        SVSparseView::new(&stored_indices, &stored_values, 3).expect("the sparse view is valid");
    let groups = [0_u32; 3];
    let request = SVMoveRequest::new(SVColumnView::Sparse(sparse), &groups);

    let moved = SVMoveSequence::single(SVMove::swarm().marker_extent(1.0).into())
        .expect("a valid sequence")
        .apply(&request)
        .expect("the swarm applies");

    assert_eq!(moved.horizontal_offset(1), Some(0.0));
    assert_eq!(moved.horizontal_offset(0), Some(1.0));
    assert_eq!(moved.horizontal_offset(2), Some(-1.0));
    assert!(
        moved.horizontal_offsets().is_sparse(),
        "the implicit zero is never stored"
    );
}
