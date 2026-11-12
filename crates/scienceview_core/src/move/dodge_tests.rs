//! Dodge tests: offsets and narrowed widths from the group counts.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{SVDodgeOptions, SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence};

/// Applies one dodge to borrowed values, group codes and category codes.
fn sv_apply_dodge_to(
    values: &[f64],
    group_keys: &[u32],
    category_keys: &[u32],
    single_move: SVMove,
) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys)
        .with_category_keys(category_keys);
    SVMoveSequence::single(single_move)
        .expect("a single dodge is a valid sequence")
        .apply(&request)
        .expect("the dodge applies")
}

/// Asserts that two slices agree within the kernel tolerance.
fn assert_slice_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (position, (actual_value, expected_value)) in actual.iter().zip(expected.iter()).enumerate()
    {
        assert!(
            (actual_value - expected_value).abs() < 1e-6,
            "position {position}: {actual_value} != {expected_value}"
        );
    }
}

#[test]
fn three_groups_divide_the_slot_evenly() {
    let values = [1.0; 6];
    let groups = [0, 1, 2, 0, 1, 2];
    let categories = [0, 0, 0, 1, 1, 1];
    let moved = sv_apply_dodge_to(&values, &groups, &categories, SVMove::dodge().into());

    // Default width 0.8 over three groups: 0.8 / 3 per group, centred on 0.
    let slot = 0.8 / 3.0;
    assert_slice_close(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[
            -0.4 + slot * 0.5,
            -0.4 + slot * 1.5,
            -0.4 + slot * 2.5,
            -0.4 + slot * 0.5,
            -0.4 + slot * 1.5,
            -0.4 + slot * 2.5,
        ],
    );
    let widths = moved.dense_widths().expect("a dodge narrows the width");
    assert_slice_close(widths, &[slot; 6]);
}

#[test]
fn the_group_count_is_per_category() {
    let values = [1.0; 5];
    let groups = [0, 1, 2, 0, 1];
    let categories = [0, 0, 0, 1, 1];
    let moved = sv_apply_dodge_to(&values, &groups, &categories, SVMove::dodge().into());

    let widths = moved.dense_widths().expect("a dodge narrows the width");
    assert_slice_close(&widths[0..3], &[0.8 / 3.0; 3]);
    assert_slice_close(&widths[3..5], &[0.8 / 2.0; 2]);
    assert_slice_close(
        &moved
            .dense_horizontal_offsets()
            .expect("a dense displacement")[3..5],
        &[-0.2, 0.2],
    );
}

#[test]
fn the_gap_shrinks_every_group_inside_its_slot() {
    let values = [1.0, 1.0];
    let groups = [0, 1];
    let categories = [0, 0];
    let moved = sv_apply_dodge_to(
        &values,
        &groups,
        &categories,
        SVMove::dodge().gap(0.2).width(0.8).into(),
    );

    let widths = moved.dense_widths().expect("a dodge narrows the width");
    assert_slice_close(widths, &[0.32, 0.32]);
    assert_slice_close(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[-0.2, 0.2],
    );
}

#[test]
fn a_dodge_never_changes_values() {
    let values = [3.0, 4.0];
    let groups = [0, 1];
    let categories = [0, 0];
    let moved = sv_apply_dodge_to(&values, &groups, &categories, SVMove::dodge().into());

    assert!(!moved.values_changed());
    assert!(moved.values().is_none());
    assert!(moved.baselines().is_none());
}

#[test]
fn an_impossible_width_is_rejected_before_any_data_is_read() {
    let result = SVMoveSequence::single(SVMove::Dodge(SVDodgeOptions::new().width(1.5)));
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));

    let result = SVMoveSequence::single(SVMove::Dodge(SVDodgeOptions::new().gap(1.0)));
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}
