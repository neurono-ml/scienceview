//! Composition-order tests: the normative pipeline and its typed rejections.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_move::{SVMove, SVMoveRequest, SVMoveSequence, SVMoveStage};

#[test]
fn displacement_then_shift_then_normalize_is_accepted() {
    let sequence = SVMoveSequence::new(vec![
        SVMove::dodge().into(),
        SVMove::shift().amount(0.1).into(),
        SVMove::normalize().into(),
    ])
    .expect("the documented order is accepted");

    assert_eq!(sequence.number_of_moves(), 3);
    assert_eq!(
        sequence
            .moves()
            .iter()
            .map(SVMove::stage)
            .collect::<Vec<SVMoveStage>>(),
        vec![
            SVMoveStage::Displacement,
            SVMoveStage::Shift,
            SVMoveStage::Normalisation
        ]
    );
}

#[test]
fn a_normalize_before_a_dodge_is_rejected() {
    let result = SVMoveSequence::new(vec![SVMove::normalize().into(), SVMove::dodge().into()]);

    assert_eq!(
        result,
        Err(SVError::MoveOrderViolation {
            position: 1,
            preceding_stage: SVMoveStage::Normalisation,
            found_stage: SVMoveStage::Displacement,
        })
    );
}

#[test]
fn a_shift_before_a_stack_is_rejected() {
    let result = SVMoveSequence::new(vec![
        SVMove::shift().amount(1.0).into(),
        SVMove::stack().into(),
    ]);

    assert!(matches!(
        result,
        Err(SVError::MoveOrderViolation {
            position: 1,
            preceding_stage: SVMoveStage::Shift,
            found_stage: SVMoveStage::Displacement,
        })
    ));
}

#[test]
fn two_displacement_moves_conflict() {
    let result = SVMoveSequence::new(vec![SVMove::stack().into(), SVMove::dodge().into()]);

    assert_eq!(
        result,
        Err(SVError::ConflictingMoveStage {
            stage: SVMoveStage::Displacement,
        })
    );
}

#[test]
fn two_shifts_conflict() {
    let result = SVMoveSequence::new(vec![
        SVMove::shift().amount(1.0).into(),
        SVMove::shift().amount(2.0).into(),
    ]);

    assert_eq!(
        result,
        Err(SVError::ConflictingMoveStage {
            stage: SVMoveStage::Shift,
        })
    );
}

#[test]
fn two_normalizes_conflict() {
    let result = SVMoveSequence::new(vec![SVMove::normalize().into(), SVMove::normalize().into()]);

    assert_eq!(
        result,
        Err(SVError::ConflictingMoveStage {
            stage: SVMoveStage::Normalisation,
        })
    );
}

#[test]
fn the_empty_sequence_is_the_identity() {
    let sequence = SVMoveSequence::empty();
    let values = [1.0, 2.0, 3.0];
    let groups = [0, 0, 0];
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &groups);
    let moved = sequence.apply(&request).expect("the identity applies");

    assert_eq!(moved.horizontal_offsets(), &[0.0, 0.0, 0.0]);
    assert_eq!(moved.vertical_offsets(), &[0.0, 0.0, 0.0]);
    assert!(!moved.values_changed());
    assert!(moved.widths().is_none());
}

#[test]
fn a_jitter_before_a_normalize_is_accepted() {
    let sequence = SVMoveSequence::new(vec![
        SVMove::jitter().seed(4).into(),
        SVMove::normalize().into(),
    ])
    .expect("jitter then normalize is the documented order");

    assert_eq!(sequence.number_of_moves(), 2);
}

#[test]
fn key_columns_must_match_the_values() {
    let values = [1.0, 2.0];
    let too_short = [0];
    let request = SVMoveRequest::new(SVColumnView::Dense(&values), &too_short);
    let sequence =
        SVMoveSequence::single(SVMove::stack().into()).expect("a single stack is a valid sequence");

    assert!(matches!(
        sequence.apply(&request),
        Err(SVError::MismatchedLengths { .. })
    ));
}
