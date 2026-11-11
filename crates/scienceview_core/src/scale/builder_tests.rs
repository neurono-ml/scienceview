//! Companion tests for the builder's validation rules.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{SVDomain, SVScaleBuilder, SVScaleKind, SVScaleRange, SVScaleTransform};

#[test]
fn a_nominal_scale_requires_categories() {
    let error = SVScaleBuilder::new(SVScaleKind::Nominal)
        .build()
        .expect_err("a nominal scale without categories has no domain");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_nominal_scale_rejects_an_explicit_numeric_domain() {
    let error = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&["alpha"])
        .domain(0.0, 3.0)
        .build()
        .expect_err("a nominal domain is its category list, not a numeric interval");

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
fn a_continuous_scale_requires_a_domain_or_a_data_view() {
    let error = SVScaleBuilder::new(SVScaleKind::Continuous)
        .build()
        .expect_err("a continuous scale needs an explicit domain or a data view");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
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
fn a_transform_is_rejected_by_a_nominal_scale() {
    let error = SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&["alpha"])
        .transform(SVScaleTransform::Linear)
        .build()
        .expect_err("a nominal scale has no numeric transform");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_range_rejects_non_finite_bounds() {
    let error =
        SVScaleRange::new(0.0, f64::NAN).expect_err("a non-finite range cannot place a mark");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_domain_rejects_inverted_bounds() {
    let error = SVDomain::new(10.0, 0.0)
        .expect_err("an inverted domain is a configuration failure, not a reversed axis");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
