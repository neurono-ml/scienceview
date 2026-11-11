//! Companion tests for domain derivation and explicit domains.

use crate::sv_data::{
    SVColumnView, SVRepresentationParityHarness, SVSparseView, SVStreamingView,
};
use crate::sv_error::SVError;
use crate::sv_scale::{
    SVDomain, SVDomainSource, SVScale, SVScaleBuilder, SVScaleKind, SVScaleTransform, SVScaleValue,
};

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
            .map(|value| scale.position(SVScaleValue::Numeric(value)).expect("finite"))
            .collect()
    });
}

#[test]
fn an_explicit_domain_overrides_the_derived_one() {
    let values = [1.0, 2.0, 3.0];
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(SVColumnView::Dense(&values))
        .domain(-5.0, 5.0)
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.domain(), SVDomain::new(-5.0, 5.0).expect("valid"));
    assert_eq!(scale.domain_source(), SVDomainSource::Explicit);
}

#[test]
fn a_derived_domain_records_its_provenance() {
    let values = [1.0, 2.0, 3.0];
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(SVColumnView::Dense(&values))
        .nice(false)
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.domain(), SVDomain::new(1.0, 3.0).expect("valid"));
    assert_eq!(scale.domain_source(), SVDomainSource::Derived);
}

#[test]
fn a_derived_domain_is_nice_by_default() {
    let values = [0.3, 4.2, 9.7];
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(SVColumnView::Dense(&values))
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.domain(), SVDomain::new(0.0, 10.0).expect("valid"));
}

#[test]
fn a_logarithmic_scale_places_values_in_decades() {
    let values = [1.0, 10.0, 100.0];
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .data_view(SVColumnView::Dense(&values))
        .transform(SVScaleTransform::Logarithmic)
        .range(0.0, 200.0)
        .build()
        .expect("the positive data fits the logarithm");

    assert_eq!(scale.domain(), SVDomain::new(1.0, 100.0).expect("valid"));
    assert_eq!(
        scale.position(SVScaleValue::Numeric(10.0)),
        Ok(100.0),
        "one decade of two lands in the middle of the range"
    );
}

#[test]
fn a_logarithmic_scale_rejects_a_domain_that_crosses_zero() {
    let error = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 100.0)
        .transform(SVScaleTransform::Logarithmic)
        .build()
        .expect_err("the logarithm has no value at zero");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_square_root_scale_rejects_a_negative_domain() {
    let error = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(-1.0, 100.0)
        .transform(SVScaleTransform::SquareRoot)
        .build()
        .expect_err("the square root has no real value below zero");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn an_inverse_position_restores_the_value() {
    for transform in [
        SVScaleTransform::Linear,
        SVScaleTransform::Logarithmic,
        SVScaleTransform::SymmetricLogarithmic,
        SVScaleTransform::SquareRoot,
    ] {
        let domain = match transform {
            SVScaleTransform::Linear | SVScaleTransform::SymmetricLogarithmic => (-10.0, 10.0),
            _ => (0.1, 10.0),
        };
        let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.0, domain.1)
            .transform(transform)
            .range(0.0, 500.0)
            .build()
            .expect("the domain fits the transform");

        let continuous = scale
            .as_continuous()
            .expect("the builder produced a continuous scale");
        let position = continuous
            .position(domain.0 + (domain.1 - domain.0) * 0.4)
            .expect("the value is finite");
        let restored = continuous
            .inverse_position(position)
            .expect("the position is finite");

        assert!(
            (restored - (domain.0 + (domain.1 - domain.0) * 0.4)).abs() <= 1e-9,
            "{transform} did not restore the value: {restored}"
        );
    }
}

#[test]
fn a_temporal_scale_derives_its_domain_from_a_data_view() {
    let timestamps = [1_000.0, 5_000.0];
    let scale = SVScaleBuilder::new(SVScaleKind::Temporal)
        .data_view(SVColumnView::Dense(&timestamps))
        .nice(false)
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.domain(), SVDomain::new(1_000.0, 5_000.0).expect("valid"));
    assert_eq!(scale.domain_source(), SVDomainSource::Derived);
}

#[test]
fn a_temporal_scale_rejects_a_non_linear_transform() {
    let error = SVScaleBuilder::new(SVScaleKind::Temporal)
        .domain(1_000.0, 5_000.0)
        .transform(SVScaleTransform::Logarithmic)
        .build()
        .expect_err("a time axis interpolates linearly");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_nominal_scale_rejects_a_data_view() {
    let values = [1.0, 2.0];
    let error = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&["alpha"])
        .data_view(SVColumnView::Dense(&values))
        .build()
        .expect_err("a numeric view is not a category column");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_boolean_scale_rejects_a_data_view() {
    let values = [0.0, 1.0];
    let error = SVScaleBuilder::new(SVScaleKind::Boolean)
        .data_view(SVColumnView::Dense(&values))
        .build()
        .expect_err("a boolean scale owns its two slots");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_built_scale_exposes_its_transform() {
    let scale: SVScale<'static> = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(1.0, 100.0)
        .transform(SVScaleTransform::Logarithmic)
        .build()
        .expect("the configuration is complete");

    assert_eq!(scale.transform(), Some(SVScaleTransform::Logarithmic));
}
