//! Shift tests: the displacement axis and the untouched data.

use crate::sv_data::SVColumnView;
use crate::sv_move::{SVMove, SVMoveOutput, SVMoveRequest, SVMoveSequence, SVShiftDirection};

/// Applies one shift to borrowed values.
fn sv_apply_shift_to(values: &[f64], single_move: SVMove) -> SVMoveOutput {
    let groups = vec![0_u32; values.len()];
    let request = SVMoveRequest::new(SVColumnView::Dense(values), &groups);
    SVMoveSequence::single(single_move)
        .expect("a single shift is a valid sequence")
        .apply(&request)
        .expect("the shift applies")
}

#[test]
fn a_horizontal_shift_displaces_the_categorical_axis() {
    let values = [1.0, 2.0, 3.0];
    let moved = sv_apply_shift_to(&values, SVMove::shift().amount(0.25).into());

    assert_eq!(moved.horizontal_offsets(), &[0.25, 0.25, 0.25]);
    assert_eq!(moved.vertical_offsets(), &[0.0, 0.0, 0.0]);
}

#[test]
fn a_vertical_shift_displaces_the_value_axis() {
    let values = [1.0, 2.0, 3.0];
    let moved = sv_apply_shift_to(
        &values,
        SVMove::shift()
            .amount(-1.5)
            .direction(SVShiftDirection::Vertical)
            .into(),
    );

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0, 0.0]);
    assert_eq!(moved.vertical_offsets(), &[-1.5, -1.5, -1.5]);
}

#[test]
fn the_default_shift_is_the_identity() {
    let values = [1.0, 2.0];
    let moved = sv_apply_shift_to(&values, SVMove::shift().into());

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0]);
    assert_eq!(moved.vertical_offsets(), &[0.0, 0.0]);
    assert!(!moved.values_changed());
    assert!(moved.widths().is_none());
}
