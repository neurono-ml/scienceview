//! Swarm tests: non-overlap, determinism and a finite run time.

use crate::sv_data::SVColumnView;
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

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn identical_values_are_spread_apart() {
    let values = [5.0, 5.0, 5.0, 5.0];
    let groups = [0; 4];
    let moved = sv_apply_swarm_to(&values, &groups, SVMove::swarm().marker_extent(1.0).into());

    let offsets = moved.horizontal_offsets();
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
                    moved.horizontal_offsets()[left],
                    values[right],
                    moved.horizontal_offsets()[right],
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

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0]);
}

#[test]
fn the_layout_is_deterministic() {
    let values = [1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 0.5];
    let groups = [0, 0, 0, 0, 0, 0, 0];
    let swarm: SVMove = SVMove::swarm().marker_extent(0.5).into();

    let first = sv_apply_swarm_to(&values, &groups, swarm.clone());
    let second = sv_apply_swarm_to(&values, &groups, swarm);

    assert_eq!(first.horizontal_offsets(), second.horizontal_offsets());
}

#[test]
fn the_width_bounds_the_displacement() {
    let values = [5.0; 64];
    let groups = [0; 64];
    let moved = sv_apply_swarm_to(
        &values,
        &groups,
        SVMove::swarm().marker_extent(0.5).width(2.0).into(),
    );

    for offset in moved.horizontal_offsets() {
        assert!(
            offset.abs() <= 1.0 + 1e-12,
            "the offset {offset} leaves the half-width 1.0"
        );
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
    assert_eq!(moved.vertical_offsets(), &[0.0, 0.0]);
}
