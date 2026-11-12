//! Stacking tests: diverging baselines, grouping, centring and zero fidelity.

use crate::sv_data::SVColumnView;
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence, SVStackMode};

/// Applies one stack to borrowed values and group codes.
fn sv_apply_stack_to(values: &[f64], group_keys: &[u32], single_move: SVMove) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys);
    SVMoveSequence::single(single_move)
        .expect("a single stack is a valid sequence")
        .apply(&request)
        .expect("the stack applies")
}

/// Asserts that two slices agree within the kernel tolerance.
fn assert_slice_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (position, (actual_value, expected_value)) in actual.iter().zip(expected.iter()).enumerate()
    {
        assert!(
            (actual_value - expected_value).abs() < 1e-12,
            "position {position}: {actual_value} != {expected_value}"
        );
    }
}

#[test]
fn positive_values_stack_upward_from_zero() {
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    assert_eq!(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[0.0, 1.0, 3.0]
    );
    assert_eq!(
        moved.dense_values().expect("a stack writes values"),
        &[1.0, 3.0, 6.0]
    );
    assert!(moved.values_changed());
}

#[test]
fn negative_values_stack_downward_from_zero() {
    let values = [-1.0, -2.0, -4.0];
    let groups = [0, 0, 0];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    assert_slice_close(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[0.0, -1.0, -3.0],
    );
    assert_slice_close(
        moved.dense_values().expect("a stack writes values"),
        &[-1.0, -3.0, -7.0],
    );
}

#[test]
fn mixed_signs_diverge_from_the_same_baseline() {
    let values = [2.0, -1.0, 3.0, -2.0];
    let groups = [0, 0, 0, 0];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    // Positives accumulate upward, negatives downward, independently.
    assert_slice_close(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[0.0, 0.0, 2.0, -1.0],
    );
    assert_slice_close(
        moved.dense_values().expect("a stack writes values"),
        &[2.0, -1.0, 5.0, -3.0],
    );
}

#[test]
fn groups_accumulate_independently() {
    let values = [1.0, 2.0, 10.0, 20.0];
    let groups = [0, 0, 1, 1];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    assert_slice_close(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[0.0, 1.0, 0.0, 10.0],
    );
    assert_slice_close(
        moved.dense_values().expect("a stack writes values"),
        &[1.0, 3.0, 10.0, 30.0],
    );
}

#[test]
fn centring_moves_the_group_midpoint_onto_zero() {
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    let moved = sv_apply_stack_to(
        &values,
        &groups,
        SVMove::stack().mode(SVStackMode::Center).into(),
    );

    // The full stack spans 0..6, so the centre mode shifts everything by -3.
    assert_slice_close(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[-3.0, -2.0, 0.0],
    );
    assert_slice_close(
        moved.dense_values().expect("a stack writes values"),
        &[-2.0, 0.0, 3.0],
    );
}

#[test]
fn a_zero_keeps_its_own_baseline_and_does_not_advance_the_stack() {
    let values = [1.0, 0.0, 2.0];
    let groups = [0, 0, 0];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    assert_slice_close(
        moved.dense_baselines().expect("a stack writes baselines"),
        &[0.0, 0.0, 1.0],
    );
    assert_slice_close(
        moved.dense_values().expect("a stack writes values"),
        &[1.0, 0.0, 3.0],
    );
}

#[test]
fn a_stack_never_touches_widths_or_offsets() {
    let values = [1.0, 2.0];
    let groups = [0, 0];
    let moved = sv_apply_stack_to(&values, &groups, SVMove::stack().into());

    assert!(moved.widths().is_none());
    assert_eq!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.0]
    );
    assert_eq!(
        moved
            .dense_vertical_offsets()
            .expect("a dense displacement"),
        &[0.0, 0.0]
    );
}
