//! Companion tests for the builder, the configuration and the composition.

use super::configuration::SVExtrapolationPolicy;
use super::{SVRegressionPlot, SVRegressionPlotBuilder};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVDotShape;
use scienceview_core::sv_stat::SVFitModel;

const X: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const Y: [f64; 5] = [1.0, 3.0, 5.0, 7.0, 9.0];

fn table<'a>(x: &'a [f64], y: &'a [f64]) -> SVDataTable<'a> {
    SVDataTable::builder()
        .with_column("x", SVColumnRole::X, x)
        .with_column("y", SVColumnRole::Y, y)
        .build()
        .expect("the test table is well formed")
}

fn plot<'a>(table: &'a SVDataTable<'a>) -> SVRegressionPlot<'a> {
    SVRegressionPlotBuilder::new()
        .data_source(table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the default configuration is valid")
}

fn build_error(
    builder: SVRegressionPlotBuilder<'_, super::SVWithDataSource, super::SVWithBackend>,
) -> SVError {
    match builder.build() {
        Err(error) => error,
        Ok(_) => panic!("the configuration must fail"),
    }
}

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
fn the_linear_fit_recovers_the_generating_line() {
    let data = table(&X, &Y);
    let composition = plot(&data).compose().expect("the fit runs");

    assert_eq!(composition.series().len(), 1);
    let series = &composition.series()[0];
    assert_eq!(series.label(), "all");
    assert!((series.coefficients()[0] - 1.0).abs() < 1.0e-9);
    assert!((series.coefficients()[1] - 2.0).abs() < 1.0e-9);
    assert_eq!(composition.point_count(), 5);
    assert_eq!(composition.dropped_pairs(), 0);
}

#[test]
fn a_polynomial_order_is_fitted_not_interpolated() {
    let x = [-2.0, -1.0, 0.0, 1.0, 2.0];
    let y: Vec<f64> = x.iter().map(|value| value * value).collect();
    let data = table(&x, &y);
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .model(SVFitModel::Polynomial)
        .order(2)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let composition = chart.compose().expect("the fit runs");
    let coefficients = composition.series()[0].coefficients();
    assert_eq!(coefficients.len(), 3);
    assert!((coefficients[0] - 0.0).abs() < 1.0e-9);
    assert!((coefficients[1] - 0.0).abs() < 1.0e-9);
    assert!((coefficients[2] - 1.0).abs() < 1.0e-9);
    assert_eq!(composition.series()[0].grid().len(), 128);
}

#[test]
fn the_confidence_band_brackets_the_curve() {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 3.2, 4.8, 7.1, 9.2];
    let data = table(&x, &y);
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .confidence(0.95)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let composition = chart.compose().expect("the fit runs");
    let series = &composition.series()[0];
    let lower = series.lower().expect("the band is configured");
    let upper = series.upper().expect("the band is configured");
    assert_eq!(lower.len(), series.predicted().len());
    for ((low, predicted), high) in lower.iter().zip(series.predicted()).zip(upper) {
        assert!(low <= predicted && predicted <= high);
    }
}

#[test]
fn each_series_receives_its_own_fit_and_colour() {
    let series = [0.0, 0.0, 0.0, 1.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("y", SVColumnRole::Y, &Y)
        .with_column("series", SVColumnRole::Grouping, &series)
        .build()
        .expect("the test table is well formed");
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let composition = chart.compose().expect("the fit runs");
    assert_eq!(composition.series().len(), 2);
    assert_eq!(composition.series()[0].label(), "0");
    assert_eq!(composition.series()[1].label(), "1");
    assert_ne!(
        composition.series()[0].colour(),
        composition.series()[1].colour()
    );
}

#[test]
fn clipping_keeps_only_the_visible_grid_samples() {
    let data = table(&X, &Y);
    let chart = SVRegressionPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .y("y")
        .extrapolation(SVExtrapolationPolicy::ClipToVisibleDomain)
        .visible_x_domain(1.0, 2.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is valid");

    let composition = chart.compose().expect("the fit runs");
    let series = &composition.series()[0];
    assert!(series.grid().len() < 128);
    assert!(
        series
            .grid()
            .iter()
            .all(|point| (1.0..=2.0).contains(point))
    );
}

#[test]
fn a_non_finite_pair_is_dropped_before_the_fit() {
    let x = [0.0, 1.0, 2.0, f64::NAN, 4.0];
    let y = [1.0, 3.0, f64::INFINITY, 7.0, 9.0];
    let data = table(&x, &y);
    let composition = plot(&data).compose().expect("the fit runs");

    assert_eq!(composition.point_count(), 3);
    assert_eq!(composition.dropped_pairs(), 2);
    assert!((composition.series()[0].coefficients()[1] - 2.0).abs() < 1.0e-9);
}

#[test]
fn an_all_non_finite_input_reports_an_empty_fit() {
    let x = [f64::NAN, f64::NAN];
    let y = [f64::NAN, f64::NAN];
    let data = table(&x, &y);

    assert_eq!(
        plot(&data).compose(),
        Err(SVError::EmptyInput {
            operation: "regression plot"
        })
    );
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
