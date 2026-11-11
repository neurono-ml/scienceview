//! Companion tests for explicit, derived and rounded domains.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{
    SVDomain, SVDomainSource, SVScale, SVScaleBuilder, SVScaleKind, SVScaleTransform, SVScaleValue,
};

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
        let expected = domain.0 + (domain.1 - domain.0) * 0.4;
        let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
            .domain(domain.0, domain.1)
            .transform(transform)
            .range(0.0, 500.0)
            .build()
            .expect("the domain fits the transform");
        let continuous = scale
            .as_continuous()
            .expect("the builder produced a continuous scale");

        let position = continuous.position(expected).expect("the value is finite");
        let restored = continuous
            .inverse_position(position)
            .expect("the position is finite");

        assert!(
            (restored - expected).abs() <= 1e-9,
            "{transform} did not restore {expected}: received {restored}"
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

    assert_eq!(
        scale.domain(),
        SVDomain::new(1_000.0, 5_000.0).expect("valid")
    );
    assert_eq!(scale.domain_source(), SVDomainSource::Derived);
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
