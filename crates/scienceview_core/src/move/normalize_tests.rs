//! Normalize tests: the group total, the percentage target and the stacked case.

use crate::sv_data::SVColumnView;
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence, SVNormalisationTarget};

/// Applies one sequence to borrowed values and group codes.
fn sv_apply(values: &[f64], group_keys: &[u32], moves: Vec<SVMove>) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys);
    SVMoveSequence::new(moves)
        .expect("the sequence is ordered")
        .apply(&request)
        .expect("the sequence applies")
}

/// Asserts that two slices agree within one part in `10^12`.
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
fn one_group_sums_to_one() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let groups = [0, 0, 0, 0];
    let moved = sv_apply(&values, &groups, vec![SVMove::normalize().into()]);

    let normalised = moved.values().expect("normalize writes values");
    assert_slice_close(normalised, &[0.1, 0.2, 0.3, 0.4]);
    let total: f64 = normalised.iter().sum();
    assert!((total - 1.0).abs() < 1e-12);
}

#[test]
fn groups_are_normalised_separately() {
    let values = [1.0, 1.0, 2.0, 6.0];
    let groups = [0, 0, 1, 1];
    let moved = sv_apply(&values, &groups, vec![SVMove::normalize().into()]);

    assert_slice_close(
        moved.values().expect("normalize writes values"),
        &[0.5, 0.5, 0.25, 0.75],
    );
}

#[test]
fn the_percentage_target_totals_to_one_hundred() {
    let values = [1.0, 3.0];
    let groups = [0, 0];
    let moved = sv_apply(
        &values,
        &groups,
        vec![
            SVMove::normalize()
                .target(SVNormalisationTarget::Percent)
                .into(),
        ],
    );

    assert_slice_close(
        moved.values().expect("normalize writes values"),
        &[25.0, 75.0],
    );
}

#[test]
fn a_stack_then_a_normalize_reaches_the_whole_group() {
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    let moved = sv_apply(
        &values,
        &groups,
        vec![SVMove::stack().into(), SVMove::normalize().into()],
    );

    assert_slice_close(
        moved.baselines().expect("a stack writes baselines"),
        &[0.0, 1.0 / 6.0, 3.0 / 6.0],
    );
    assert_slice_close(
        moved.values().expect("a stack writes values"),
        &[1.0 / 6.0, 3.0 / 6.0, 1.0],
    );
}

#[test]
fn a_diverging_stack_normalises_both_directions() {
    let values = [1.0, 2.0, -1.0, -3.0];
    let groups = [0, 0, 0, 0];
    let moved = sv_apply(
        &values,
        &groups,
        vec![SVMove::stack().into(), SVMove::normalize().into()],
    );

    assert_slice_close(
        moved.values().expect("a stack writes values"),
        &[1.0 / 3.0, 1.0, -0.25, -1.0],
    );
    assert_slice_close(
        moved.baselines().expect("a stack writes baselines"),
        &[0.0, 1.0 / 3.0, 0.0, -0.25],
    );
}

#[test]
fn a_zero_group_stays_zero() {
    let values = [0.0, 0.0];
    let groups = [0, 0];
    let moved = sv_apply(&values, &groups, vec![SVMove::normalize().into()]);

    assert_slice_close(
        moved.values().expect("normalize writes values"),
        &[0.0, 0.0],
    );
}

#[test]
fn normalising_leaves_offsets_and_widths_untouched() {
    let values = [2.0, 2.0];
    let groups = [0, 0];
    let moved = sv_apply(&values, &groups, vec![SVMove::normalize().into()]);

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0]);
    assert_eq!(moved.vertical_offsets(), &[0.0, 0.0]);
    assert!(moved.widths().is_none());
}
