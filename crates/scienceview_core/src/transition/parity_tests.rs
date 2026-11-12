//! Representation parity: the same marks resolve the same keys and produce the
//! same frames whether the key column is dense, sparse or streaming.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_mark::SVMarkPoint;
use crate::sv_transition::{
    SVEasing, SVMarkPropertyValue, SVMarkState, SVMatchKey, SVMatchKeyValue, SVTransition,
    sv_resolve_match_keys,
};

#[test]
fn every_representation_resolves_the_same_category_keys() {
    let values = [1.0, 2.0, 3.0];
    let indices = [0, 1, 2];
    let sparse = SVSparseView::new(&indices, &values, 3).expect("the sparse view is valid");
    let streaming = SVStreamingView::new(&values, 3, 0);

    let dense_keys = sv_resolve_match_keys(SVColumnView::Dense(&values), SVMatchKey::Category)
        .expect("unique keys");
    let sparse_keys = sv_resolve_match_keys(SVColumnView::Sparse(sparse), SVMatchKey::Category)
        .expect("unique keys");
    let streaming_keys =
        sv_resolve_match_keys(SVColumnView::Streaming(streaming), SVMatchKey::Category)
            .expect("unique keys");

    assert_eq!(dense_keys, sparse_keys);
    assert_eq!(dense_keys, streaming_keys);
}

#[test]
fn a_sparse_window_keeps_its_implicit_zeros_out_of_the_keys() {
    let indices = [0, 2];
    let values = [1.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 3).expect("the sparse view is valid");

    let keys = sv_resolve_match_keys(SVColumnView::Sparse(sparse), SVMatchKey::Category)
        .expect("unique keys");

    assert_eq!(
        keys,
        vec![
            SVMatchKeyValue::category(1.0).expect("finite"),
            SVMatchKeyValue::category(3.0).expect("finite"),
        ]
    );
}

#[test]
fn index_keys_are_identical_for_every_representation() {
    let values = [7.0, 8.0, 9.0];
    let sparse = SVSparseView::new(&[0, 2], &[7.0, 9.0], 3).expect("valid");
    let streaming = SVStreamingView::new(&values, 1, 0);

    let dense =
        sv_resolve_match_keys(SVColumnView::Dense(&values), SVMatchKey::Index).expect("index keys");
    let sparse_keys =
        sv_resolve_match_keys(SVColumnView::Sparse(sparse), SVMatchKey::Index).expect("index keys");
    let streamed = sv_resolve_match_keys(SVColumnView::Streaming(streaming), SVMatchKey::Index)
        .expect("index keys");

    assert_eq!(dense, sparse_keys);
    assert_eq!(dense, streamed);
}

#[test]
fn a_plan_built_from_resolved_keys_is_representation_blind() {
    let previous_values = [1.0, 2.0];
    let next_values = [1.0, 2.0];
    let keys = sv_resolve_match_keys(SVColumnView::Dense(&previous_values), SVMatchKey::Category)
        .expect("unique keys");

    let previous: Vec<SVMarkState> = keys
        .iter()
        .map(|key| {
            SVMarkState::new(*key)
                .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(0.0, 0.0)))
        })
        .collect();
    let next: Vec<SVMarkState> = sv_resolve_match_keys(
        SVColumnView::Streaming(SVStreamingView::new(&next_values, 2, 0)),
        SVMatchKey::Category,
    )
    .expect("unique keys")
    .iter()
    .map(|key| {
        SVMarkState::new(*key)
            .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(10.0, 0.0)))
    })
    .collect();

    let plan = SVTransition::new()
        .easing(SVEasing::Linear)
        .plan(&previous, &next)
        .expect("keys are unique");

    assert_eq!(plan.update().len(), 2);
    assert!(plan.enter().is_empty());
    assert!(plan.exit().is_empty());
}
