//! The configuration surface: defaults, option reach and build-time errors.

use super::super::{SVExtrapolationPolicy, SVRegressionPlotBuilder};
use super::{X, Y, build_error, plot, table};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDotShape;
use scienceview_core::sv_stat::SVFitModel;

#[test]
fn the_default_configuration_matches_the_fdr() {
    let data = table(&X, &Y);
    let chart = plot(&data);
    let configuration = chart.configuration();

    assert_eq!(configuration.model(), SVFitModel::Linear);
    assert_eq!(configuration.order(), 1);
    assert_eq!(configuration.confidence(), None);
    assert!(configuration.show_points());
    assert!(configuration.show_fit());
    assert_eq!(configuration.marker(), SVDotShape::Circle);
    assert_eq!(configuration.line_width(), SVLength::absolute(2.0));
    assert_eq!(
        configuration.extrapolation(),
        SVExtrapolationPolicy::FitRange
    );
    assert_eq!(configuration.series(), None);
}

#[test]
fn a_missing_column_fails_at_build_time() {
    let data = table(&X, &Y);
    let error = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("missing")
            .backend(SVBackendKind::Svg),
    );

    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "missing".to_string()
        }
    );
}

#[test]
fn incomplete_and_inconsistent_configurations_fail_at_build_time() {
    let data = table(&X, &Y);
    let missing_x = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .y("y")
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(missing_x, SVError::InvalidConfiguration { .. }));

    let same_column = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("x")
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(same_column, SVError::InvalidConfiguration { .. }));

    let zero_order = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("y")
            .model(SVFitModel::Polynomial)
            .order(0)
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(zero_order, SVError::InvalidConfiguration { .. }));

    let invalid_confidence = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("y")
            .confidence(1.5)
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(
        invalid_confidence,
        SVError::InvalidConfiguration { .. }
    ));

    let hidden_both = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("y")
            .show_points(false)
            .show_fit(false)
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(hidden_both, SVError::InvalidConfiguration { .. }));

    let inverted_domain = build_error(
        SVRegressionPlotBuilder::new()
            .data_source(&data)
            .x("x")
            .y("y")
            .visible_x_domain(4.0, 1.0)
            .backend(SVBackendKind::Svg),
    );
    assert!(matches!(
        inverted_domain,
        SVError::InvalidConfiguration { .. }
    ));
}

#[test]
fn option_setters_reach_the_configuration() {
    let data = table(&X, &Y);
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .title("A fitted line")
        .model(SVFitModel::Exponential)
        .order(3)
        .show_points(false)
        .show_fit(true)
        .marker(SVDotShape::Diamond)
        .line_width(SVLength::absolute(3.0))
        .extrapolation(SVExtrapolationPolicy::ClipToVisibleDomain)
        .visible_x_domain(0.5, 3.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let configuration = chart.configuration();
    assert_eq!(chart.title(), Some("A fitted line"));
    assert_eq!(configuration.model(), SVFitModel::Exponential);
    assert_eq!(configuration.order(), 3);
    assert!(!configuration.show_points());
    assert_eq!(configuration.marker(), SVDotShape::Diamond);
    assert_eq!(configuration.line_width(), SVLength::absolute(3.0));
    assert_eq!(configuration.visible_x_domain(), Some((0.5, 3.5)));
    assert_eq!(chart.backend_kind(), SVBackendKind::Svg);
}
