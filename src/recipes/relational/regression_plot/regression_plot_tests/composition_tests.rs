//! The composition: fits, grouping, clipping and the data-boundary errors.

use super::super::{SVExtrapolationPolicy, SVRegressionPlotBuilder};
use super::{X, Y, plot, table};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVColumnView, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVFitModel, SVFitOptions, SVStat, SVStatInput, SVStatResult};

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
fn the_confidence_band_is_the_kernel_interval() {
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

    let input = SVStatInput::new(SVColumnView::Dense(&y)).with_secondary(SVColumnView::Dense(&x));
    let kernel = match SVStat::fit(SVFitOptions::new().confidence(0.95))
        .apply(input)
        .expect("the kernel fits")
    {
        SVStatResult::Fit(result) => result,
        other => panic!("the fit stat produced an unexpected result: {other:?}"),
    };
    assert_eq!(series.grid(), kernel.grid());
    assert_eq!(series.predicted(), kernel.predicted());
    assert_eq!(lower, kernel.lower().expect("the kernel carries a band"));
    assert_eq!(upper, kernel.upper().expect("the kernel carries a band"));
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
