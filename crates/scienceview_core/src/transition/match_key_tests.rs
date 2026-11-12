//! Executable specification of match-key resolution and duplicate detection.

use crate::sv_data::{SVColumnView, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_transition::{SVMatchKey, SVMatchKeyValue, sv_resolve_match_keys};

#[test]
fn category_keys_are_resolved_in_column_order() {
    let categories = [2.0, 0.0, 1.0];

    let keys = sv_resolve_match_keys(SVColumnView::Dense(&categories), SVMatchKey::Category)
        .expect("every code is finite");

    assert_eq!(
        keys,
        vec![
            SVMatchKeyValue::category(2.0).expect("finite"),
            SVMatchKeyValue::category(0.0).expect("finite"),
            SVMatchKeyValue::category(1.0).expect("finite"),
        ]
    );
}

#[test]
fn a_category_key_never_equals_an_identity_key_with_the_same_code() {
    let category = SVMatchKeyValue::category(7.0).expect("finite");
    let identity = SVMatchKeyValue::identity(7.0).expect("finite");

    assert_ne!(category, identity);
    assert_eq!(category.as_str(), "category");
    assert_eq!(identity.as_str(), "identity");
}

#[test]
fn index_keys_cover_every_logical_position_without_reading_values() {
    let view = SVColumnView::Dense(&[9.0, 9.0, 9.0]);

    let keys = sv_resolve_match_keys(view, SVMatchKey::Index).expect("index needs no value");

    assert_eq!(
        keys,
        vec![
            SVMatchKeyValue::index(0),
            SVMatchKeyValue::index(1),
            SVMatchKeyValue::index(2),
        ]
    );
}

#[test]
fn derived_keys_are_deterministic_and_follow_their_seed() {
    let values = [1.0, 2.0, 3.0];
    let view = SVColumnView::Dense(&values);

    let first = sv_resolve_match_keys(view, SVMatchKey::Derived { seed: 11 })
        .expect("derived keys resolve");
    let again = sv_resolve_match_keys(view, SVMatchKey::Derived { seed: 11 })
        .expect("derived keys resolve");
    let other_seed = sv_resolve_match_keys(view, SVMatchKey::Derived { seed: 12 })
        .expect("derived keys resolve");

    assert_eq!(first, again, "the same seed must give the same keys");
    assert_ne!(
        first, other_seed,
        "a different seed must give different keys"
    );
    assert!(first.iter().all(|key| key.as_str() == "derived"));
}

#[test]
fn duplicate_category_keys_are_rejected() {
    let categories = [1.0, 2.0, 1.0];

    let error = sv_resolve_match_keys(SVColumnView::Dense(&categories), SVMatchKey::Category)
        .expect_err("two marks cannot share a key");

    match error {
        SVError::InvalidConfiguration { message } => {
            assert!(
                message.contains("duplicate"),
                "the message must name the problem: {message}"
            );
        }
        other => panic!("expected a configuration error, got {other:?}"),
    }
}

#[test]
fn a_sparse_column_exposes_only_its_stored_observations() {
    let indices = [1, 4];
    let values = [3.0, 5.0];
    let sparse = SVSparseView::new(&indices, &values, 7).expect("the sparse view is valid");
    let view = SVColumnView::Sparse(sparse);

    let keys = sv_resolve_match_keys(view, SVMatchKey::Category).expect("stored codes are finite");

    assert_eq!(
        keys,
        vec![
            SVMatchKeyValue::category(3.0).expect("finite"),
            SVMatchKeyValue::category(5.0).expect("finite"),
        ],
        "an implicit zero is not an observation and carries no key"
    );
}

#[test]
fn a_streaming_column_resolves_the_keys_of_its_window_snapshot() {
    let window = [4.0, 5.0];
    let streaming = SVStreamingView::new(&window, 12, 3);
    let view = SVColumnView::Streaming(streaming);

    let keys = sv_resolve_match_keys(view, SVMatchKey::Identity).expect("codes are finite");

    assert_eq!(
        keys,
        vec![
            SVMatchKeyValue::identity(4.0).expect("finite"),
            SVMatchKeyValue::identity(5.0).expect("finite"),
        ]
    );
}

#[test]
fn a_non_finite_key_code_is_rejected_with_its_position() {
    let categories = [1.0, f64::NAN];

    let error = sv_resolve_match_keys(SVColumnView::Dense(&categories), SVMatchKey::Category)
        .expect_err("a NaN is not an identity");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "transition.match_key",
            position: 1,
        }
    );
}

#[test]
fn duplicate_detection_sees_an_identity_repeated_after_a_gap() {
    let identities = [1.0, 2.0, 2.0, 3.0];

    let error = sv_resolve_match_keys(SVColumnView::Dense(&identities), SVMatchKey::Identity)
        .expect_err("the second identity repeats");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
