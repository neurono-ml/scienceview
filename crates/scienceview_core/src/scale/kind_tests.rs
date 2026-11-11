//! Companion tests for the four scale kinds and the domain/range model.

use crate::sv_error::SVError;
use crate::sv_scale::{
    SVCategoryOrder, SVDomain, SVDomainSource, SVScale, SVScaleBuilder, SVScaleKind, SVScaleRange,
    SVScaleValue,
};

const NOMINAL_CATEGORIES: [&str; 3] = ["gamma", "alpha", "beta"];

fn continuous_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 10.0)
        .range(100.0, 200.0)
        .build()
        .expect("the continuous configuration is complete")
}

fn nominal_scale(order: SVCategoryOrder) -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Nominal)
        .categories(&NOMINAL_CATEGORIES)
        .category_order(order)
        .range(0.0, 100.0)
        .build()
        .expect("the nominal configuration is complete")
}

fn temporal_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Temporal)
        .domain(0.0, 1_000.0)
        .range(0.0, 400.0)
        .build()
        .expect("the temporal configuration is complete")
}

fn boolean_scale() -> SVScale<'static> {
    SVScaleBuilder::new(SVScaleKind::Boolean)
        .range(0.0, 10.0)
        .build()
        .expect("the boolean configuration is complete")
}

#[test]
fn continuous_scale_maps_its_domain_onto_its_range() {
    let scale = continuous_scale();

    assert_eq!(scale.kind(), SVScaleKind::Continuous);
    assert_eq!(
        scale.position(SVScaleValue::Numeric(0.0)),
        Ok(100.0),
        "the lower domain bound maps to the start of the range"
    );
    assert_eq!(
        scale.position(SVScaleValue::Numeric(10.0)),
        Ok(200.0),
        "the upper domain bound maps to the end of the range"
    );
    assert_eq!(
        scale.position(SVScaleValue::Numeric(5.0)),
        Ok(150.0),
        "the middle of the domain maps to the middle of the range"
    );
}

#[test]
fn continuous_scale_honours_a_reversed_range() {
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(0.0, 10.0)
        .range(200.0, 100.0)
        .build()
        .expect("a reversed range is a valid inverted axis");

    assert_eq!(scale.position(SVScaleValue::Numeric(0.0)), Ok(200.0));
    assert_eq!(scale.position(SVScaleValue::Numeric(10.0)), Ok(100.0));
}

#[test]
fn continuous_scale_maps_a_degenerate_domain_to_the_range_middle() {
    let scale = SVScaleBuilder::new(SVScaleKind::Continuous)
        .domain(5.0, 5.0)
        .range(100.0, 200.0)
        .build()
        .expect("a degenerate domain is valid and draws one tick");

    assert_eq!(scale.position(SVScaleValue::Numeric(5.0)), Ok(150.0));
}

#[test]
fn temporal_scale_maps_a_timestamp_onto_a_position() {
    let scale = temporal_scale();

    assert_eq!(scale.kind(), SVScaleKind::Temporal);
    assert_eq!(scale.position(SVScaleValue::Timestamp(0.0)), Ok(0.0));
    assert_eq!(scale.position(SVScaleValue::Timestamp(500.0)), Ok(200.0));
    assert_eq!(scale.position(SVScaleValue::Timestamp(1_000.0)), Ok(400.0));
}

#[test]
fn nominal_scale_places_categories_in_data_order() {
    let scale = nominal_scale(SVCategoryOrder::DataOrder);

    assert_eq!(scale.kind(), SVScaleKind::Nominal);
    assert_eq!(scale.position(SVScaleValue::Category("gamma")), Ok(0.0));
    assert_eq!(scale.position(SVScaleValue::Category("alpha")), Ok(50.0));
    assert_eq!(scale.position(SVScaleValue::Category("beta")), Ok(100.0));
}

#[test]
fn nominal_scale_places_categories_in_alphabetical_order() {
    let scale = nominal_scale(SVCategoryOrder::Alphabetical);

    assert_eq!(scale.position(SVScaleValue::Category("alpha")), Ok(0.0));
    assert_eq!(scale.position(SVScaleValue::Category("beta")), Ok(50.0));
    assert_eq!(scale.position(SVScaleValue::Category("gamma")), Ok(100.0));
}

#[test]
fn nominal_scale_rejects_an_unknown_category() {
    let scale = nominal_scale(SVCategoryOrder::DataOrder);

    let error = scale
        .position(SVScaleValue::Category("delta"))
        .expect_err("a category outside the domain cannot have a position");

    assert!(
        matches!(error, SVError::InvalidConfiguration { .. }),
        "an unknown category is a configuration failure, received {error:?}"
    );
}

#[test]
fn boolean_scale_places_the_two_values_on_two_slots() {
    let scale = boolean_scale();

    assert_eq!(scale.kind(), SVScaleKind::Boolean);
    assert_eq!(scale.position(SVScaleValue::Boolean(false)), Ok(0.0));
    assert_eq!(scale.position(SVScaleValue::Boolean(true)), Ok(10.0));
}

#[test]
fn boolean_scale_uses_the_documented_default_labels() {
    let scale = boolean_scale();

    assert_eq!(scale.domain().lower(), 0.0);
    assert_eq!(scale.domain().upper(), 1.0);
    assert_eq!(
        scale.boolean_labels(),
        Some(("false".to_string(), "true".to_string()))
    );
}

#[test]
fn boolean_scale_accepts_custom_labels() {
    let scale = SVScaleBuilder::new(SVScaleKind::Boolean)
        .boolean_labels("off", "on")
        .build()
        .expect("custom labels are a valid configuration");

    assert_eq!(
        scale.boolean_labels(),
        Some(("off".to_string(), "on".to_string()))
    );
}

#[test]
fn every_value_kind_matches_its_scale_kind() {
    assert_eq!(SVScaleValue::Numeric(1.0).kind(), SVScaleKind::Continuous);
    assert_eq!(SVScaleValue::Category("alpha").kind(), SVScaleKind::Nominal);
    assert_eq!(SVScaleValue::Timestamp(1.0).kind(), SVScaleKind::Temporal);
    assert_eq!(SVScaleValue::Boolean(true).kind(), SVScaleKind::Boolean);
}

#[test]
fn a_value_of_the_wrong_kind_is_rejected() {
    let scale = continuous_scale();

    let error = scale
        .position(SVScaleValue::Category("alpha"))
        .expect_err("a nominal value cannot be placed by a continuous scale");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn explicit_domains_record_their_provenance() {
    let scale = continuous_scale();

    assert_eq!(scale.domain_source(), SVDomainSource::Explicit);
    assert_eq!(
        nominal_scale(SVCategoryOrder::DataOrder).domain_source(),
        SVDomainSource::Intrinsic
    );
    assert_eq!(boolean_scale().domain_source(), SVDomainSource::Intrinsic);
}

#[test]
fn the_domain_and_range_accessors_report_the_configuration() {
    let scale = continuous_scale();

    assert_eq!(scale.domain(), SVDomain::new(0.0, 10.0).expect("valid"));
    assert_eq!(
        scale.range(),
        SVScaleRange::new(100.0, 200.0).expect("valid")
    );
    assert_eq!(scale.tick_count(), None);
}

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
        .categories(&NOMINAL_CATEGORIES)
        .domain(0.0, 3.0)
        .build()
        .expect_err("a nominal domain is its category list, not a numeric interval");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_continuous_scale_requires_an_explicit_domain_before_derivation_exists() {
    let error = SVScaleBuilder::new(SVScaleKind::Continuous)
        .build()
        .expect_err("a continuous scale needs an explicit domain or a data view");

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
