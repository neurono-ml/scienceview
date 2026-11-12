//! Option defaults, kinds, stages and the generator's reference vector.

use crate::sv_move::{
    SVDodgeOptions, SVJitterMode, SVJitterOptions, SVMove, SVMoveKind, SVMoveStage,
    SVNormalisationTarget, SVNormalizeOptions, SVShiftDirection, SVShiftOptions, SVStackMode,
    SVStackOptions, SVSwarmOptions, sv_split_mix_64,
};

#[test]
fn every_option_struct_reports_its_documented_default() {
    assert_eq!(SVStackOptions::default().mode_value(), SVStackMode::Full);
    assert_eq!(SVDodgeOptions::default().gap_value(), 0.0);
    assert_eq!(SVDodgeOptions::default().width_value(), 0.8);
    assert_eq!(SVJitterOptions::default().amount_value(), 0.1);
    assert_eq!(
        SVJitterOptions::default().mode_value(),
        SVJitterMode::Uniform
    );
    assert_eq!(SVJitterOptions::default().seed_value(), None);
    assert_eq!(SVShiftOptions::default().amount_value(), 0.0);
    assert_eq!(
        SVShiftOptions::default().direction_value(),
        SVShiftDirection::Horizontal
    );
    assert_eq!(SVSwarmOptions::default().width_value(), None);
    assert_eq!(SVSwarmOptions::default().marker_extent_value(), 1.0);
    assert_eq!(
        SVNormalizeOptions::default().target_value(),
        SVNormalisationTarget::Fraction
    );
}

#[test]
fn the_kind_set_is_closed_and_exhaustive() {
    let moves = [
        SVMove::stack().into(),
        SVMove::dodge().into(),
        SVMove::jitter().into(),
        SVMove::shift().into(),
        SVMove::swarm().into(),
        SVMove::normalize().into(),
    ];
    let kinds: Vec<SVMoveKind> = moves.iter().map(SVMove::kind).collect();

    assert_eq!(kinds, SVMoveKind::ALL.to_vec());
    assert_eq!(
        SVMoveKind::ALL.map(|kind| kind.as_str()),
        ["stack", "dodge", "jitter", "shift", "swarm", "normalize"]
    );
}

#[test]
fn moves_map_to_their_composition_stage() {
    let moves: [SVMove; 6] = [
        SVMove::stack().into(),
        SVMove::dodge().into(),
        SVMove::jitter().into(),
        SVMove::swarm().into(),
        SVMove::shift().into(),
        SVMove::normalize().into(),
    ];
    assert_eq!(
        moves.map(|single_move| single_move.stage()),
        [
            SVMoveStage::Displacement,
            SVMoveStage::Displacement,
            SVMoveStage::Displacement,
            SVMoveStage::Displacement,
            SVMoveStage::Shift,
            SVMoveStage::Normalisation,
        ]
    );
}

#[test]
fn the_generator_matches_its_reference_vector() {
    let mut state = 0_u64;
    assert_eq!(sv_split_mix_64(&mut state), 0xE220_A839_7B1D_CDAF);

    let mut repeated = 0_u64;
    let first_run: Vec<u64> = (0..8).map(|_| sv_split_mix_64(&mut repeated)).collect();
    let mut repeated_again = 0_u64;
    let second_run: Vec<u64> = (0..8)
        .map(|_| sv_split_mix_64(&mut repeated_again))
        .collect();
    assert_eq!(first_run, second_run);
}

#[test]
fn the_stage_enumeration_lists_the_composition_order() {
    assert_eq!(
        SVMoveStage::ALL,
        [
            SVMoveStage::Displacement,
            SVMoveStage::Shift,
            SVMoveStage::Normalisation,
        ]
    );
    assert_eq!(
        SVMoveStage::ALL.map(|stage| stage.as_str()),
        ["displacement", "shift", "normalisation"]
    );
    // Ordering is the contract every sequence validation relies on.
    assert!(SVMoveStage::Displacement < SVMoveStage::Shift);
    assert!(SVMoveStage::Shift < SVMoveStage::Normalisation);
}

#[test]
fn the_output_accessors_answer_one_point_at_a_time() {
    use crate::sv_data::SVColumnView;
    use crate::sv_move::{SVMoveRequest, SVMoveSequence, SVShiftDirection};

    let values = [1.0, 2.0];
    let groups = [0, 1];
    let categories = [0, 0];
    let sequence = SVMoveSequence::new(vec![
        SVMove::dodge().into(),
        SVMove::shift()
            .amount(0.5)
            .direction(SVShiftDirection::Vertical)
            .into(),
    ])
    .expect("dodge then shift is the documented order");
    let request =
        SVMoveRequest::new(SVColumnView::Dense(&values), &groups).with_category_keys(&categories);
    let moved = sequence.apply(&request).expect("the sequence applies");

    let close = |actual: Option<f64>, expected: f64| {
        assert!(
            actual.is_some_and(|value| (value - expected).abs() < 1e-6),
            "{actual:?} != {expected}"
        );
    };
    close(moved.horizontal_offset(0), -0.2);
    close(moved.horizontal_offset(1), 0.2);
    close(moved.vertical_offset(0), 0.5);
    close(moved.width_at(0), 0.4);
    close(moved.width_at(1), 0.4);
    // A position past the logical length has no displacement and no width.
    assert_eq!(moved.horizontal_offset(2), None);
    assert_eq!(moved.vertical_offset(2), None);
    assert_eq!(moved.width_at(2), None);
}
