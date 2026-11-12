//! Jitter tests: determinism, band and key identity.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{
    SVJitterMode, SVJitterOptions, SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence,
};

/// Applies one jitter to borrowed values and group codes.
fn sv_apply_jitter_to(values: &[f64], group_keys: &[u32], single_move: SVMove) -> SVMoveOutput {
    let request = SVMoveRequest::new(SVColumnView::Dense(values), group_keys);
    SVMoveSequence::single(single_move)
        .expect("a single jitter is a valid sequence")
        .apply(&request)
        .expect("the jitter applies")
}

#[test]
fn a_non_zero_amount_displaces_points() {
    let values = [1.0; 8];
    let groups = [0; 8];
    let moved = sv_apply_jitter_to(&values, &groups, SVMove::jitter().seed(7).into());

    assert!(
        moved
            .dense_horizontal_offsets()
            .expect("a dense displacement")
            .iter()
            .any(|offset| offset.abs() > f64::EPSILON),
        "a jitter with an amount must move at least one point"
    );
}

#[test]
fn the_same_seed_produces_the_same_displacement() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    let groups = [0, 0, 1, 1, 1];
    let jitter: SVMove = SVMove::jitter().seed(42).into();

    let first = sv_apply_jitter_to(&values, &groups, jitter.clone());
    let second = sv_apply_jitter_to(&values, &groups, jitter);

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
fn a_different_seed_produces_a_different_displacement() {
    let values = [1.0; 16];
    let groups = [0; 16];
    let first = sv_apply_jitter_to(&values, &groups, SVMove::jitter().seed(1).into());
    let second = sv_apply_jitter_to(&values, &groups, SVMove::jitter().seed(2).into());

    assert_ne!(
        first
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        second
            .dense_horizontal_offsets()
            .expect("a dense displacement")
    );
}

#[test]
fn the_displacement_stays_within_the_amount_band() {
    let values = [0.0; 512];
    let groups = [0; 512];
    let moved = sv_apply_jitter_to(
        &values,
        &groups,
        SVMove::jitter().amount(0.1).seed(11).into(),
    );

    let band = 0.1 * 0.8;
    for offset in moved
        .dense_horizontal_offsets()
        .expect("a dense displacement")
    {
        assert!(
            offset.abs() <= band + 1e-12,
            "the offset {offset} leaves the documented band {band}"
        );
    }
}

#[test]
fn the_gaussian_mode_is_bounded_and_deterministic() {
    let values = [0.0; 256];
    let groups = [0; 256];
    let options = SVMove::jitter()
        .amount(0.2)
        .mode(SVJitterMode::Gaussian)
        .seed(5);
    let jitter: SVMove = options.into();
    let first = sv_apply_jitter_to(&values, &groups, jitter.clone());
    let second = sv_apply_jitter_to(&values, &groups, jitter);

    assert_eq!(
        first
            .dense_horizontal_offsets()
            .expect("a dense displacement"),
        second
            .dense_horizontal_offsets()
            .expect("a dense displacement")
    );
    let band = 0.2 * 0.8;
    for offset in first
        .dense_horizontal_offsets()
        .expect("a dense displacement")
    {
        assert!(offset.abs() <= band + 1e-12);
    }
}

#[test]
fn the_offset_follows_the_mark_key_not_the_value() {
    let group_keys = [3, 3, 7, 7];
    let first_values = [10.0, 20.0, 30.0, 40.0];
    let second_values = [-1.0, 5.0, 0.0, 2.0];
    let jitter: SVMove = SVMove::jitter().seed(9).into();

    let first = sv_apply_jitter_to(&first_values, &group_keys, jitter.clone());
    let second = sv_apply_jitter_to(&second_values, &group_keys, jitter);

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
fn a_jitter_never_changes_values_widths_or_the_vertical_axis() {
    let values = [1.0, 2.0];
    let groups = [0, 1];
    let moved = sv_apply_jitter_to(&values, &groups, SVMove::jitter().seed(3).into());

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
fn a_negative_amount_is_rejected() {
    let result = SVMoveSequence::single(SVMove::Jitter(SVJitterOptions::new().amount(-0.1)));
    assert!(matches!(result, Err(SVError::InvalidConfiguration { .. })));
}
