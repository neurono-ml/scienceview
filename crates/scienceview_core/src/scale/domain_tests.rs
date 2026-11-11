//! Companion tests for domain derivation and explicit domains.

use crate::sv_data::{SVColumnView, SVRepresentationParityHarness, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_scale::{SVDomain, SVScaleBuilder, SVScaleKind, SVScaleValue};

const DENSE_DATASET: [f64; 5] = [1.0, 2.0, 0.0, 4.0, 5.0];

#[test]
fn a_dense_column_derives_its_extent() {
    let values = [3.5, -1.5, 2.0];
    let domain =
        SVDomain::from_column_view(SVColumnView::Dense(&values)).expect("the column has values");

    assert_eq!(domain, SVDomain::new(-1.5, 3.5).expect("valid"));
}

#[test]
fn derivation_ignores_non_finite_values() {
    let values = [1.0, f64::NAN, 4.0, f64::INFINITY];
    let domain =
        SVDomain::from_column_view(SVColumnView::Dense(&values)).expect("two values are finite");

    assert_eq!(domain, SVDomain::new(1.0, 4.0).expect("valid"));
}

#[test]
fn a_column_without_a_finite_value_has_no_domain() {
    let values = [f64::NAN, f64::INFINITY];
    let error = SVDomain::from_column_view(SVColumnView::Dense(&values))
        .expect_err("a domain cannot be derived from non-finite values only");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_sparse_column_accounts_for_its_implicit_zeros_without_densifying() {
    let indices = [0_usize, 2];
    let stored = [4.0_f64, 9.0];
    let sparse = SVSparseView::new(&indices, &stored, 4).expect("valid sparse view");

    let domain = SVDomain::from_column_view(SVColumnView::Sparse(sparse))
        .expect("the sparse column has values");

    assert_eq!(
        domain,
        SVDomain::new(0.0, 9.0).expect("valid"),
        "an implicit zero participates in the domain just like a stored zero"
    );
    assert_eq!(
        sparse.number_of_stored_values(),
        2,
        "the sparse storage was not expanded"
    );
}

#[test]
fn a_fully_stored_sparse_column_matches_its_dense_twin() {
    let indices = [0_usize, 1, 2];
    let stored = [4.0_f64, 9.0, 12.0];
    let sparse = SVSparseView::new(&indices, &stored, 3).expect("valid sparse view");

    let domain = SVDomain::from_column_view(SVColumnView::Sparse(sparse)).expect("values");

    assert_eq!(domain, SVDomain::new(4.0, 12.0).expect("valid"));
}

#[test]
fn a_streaming_column_derives_from_its_bounded_snapshot() {
    let snapshot = [7.0_f64, 8.0];
    let streaming = SVStreamingView::new(&snapshot, 42, 6);
    let domain = SVDomain::from_column_view(SVColumnView::Streaming(streaming))
        .expect("the snapshot has values");

    assert_eq!(domain, SVDomain::new(7.0, 8.0).expect("valid"));
}

#[test]
fn the_first_real_kernel_proves_representation_parity() {
    let harness = SVRepresentationParityHarness::new(&DENSE_DATASET);

    harness.assert_kernel_parity(|view| {
        let domain = SVDomain::from_column_view(view).expect("the dataset has values");
        vec![domain.lower(), domain.upper()]
    });
}

#[test]
fn positions_agree_across_representations() {
    let harness = SVRepresentationParityHarness::new(&DENSE_DATASET);
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 5.0)
        .range(0.0, 100.0)
        .build()
        .expect("the configuration is complete");

    harness.assert_kernel_parity(|view| {
        (0..view.length())
            .filter_map(|index| view.value_at(index))
            .map(|value| {
                scale
                    .position(SVScaleValue::Numeric(value))
                    .expect("finite")
            })
            .collect()
    });
}
