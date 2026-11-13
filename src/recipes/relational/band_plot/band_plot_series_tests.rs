//! Multi-series bands: one band per series, in first-appearance order.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

use super::SVBandPlotBuilder;

const X: [f64; 8] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
const SERIES: [f64; 8] = [1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0];
const LOWER: [f64; 8] = [0.2, 0.3, 0.4, 0.5, 1.2, 1.3, 1.4, 1.5];
const UPPER: [f64; 8] = [0.7, 0.8, 0.9, 1.0, 1.7, 1.8, 1.9, 2.0];

#[test]
fn explicit_series_draws_one_band_per_series() {
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &X)
        .with_column("lower", SVColumnRole::Y, &LOWER)
        .with_column("upper", SVColumnRole::Y, &UPPER)
        .with_column("series", SVColumnRole::Grouping, &SERIES)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .lower("lower")
        .upper("upper")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bands draw");

    assert_eq!(output.geometries().len(), 2);
    assert_eq!(output.number_of_points(), X.len());
    assert!(!output.decorations().is_empty());
}

#[test]
fn derived_series_aggregates_each_series_separately() {
    let x = [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0];
    let y = [1.0, 3.0, 2.0, 4.0, 11.0, 13.0, 12.0, 14.0];
    let series = [1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .with_column("series", SVColumnRole::Grouping, &series)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the bands draw");

    assert_eq!(output.geometries().len(), 2);
    assert_eq!(output.number_of_points(), 4);
}

#[test]
fn derived_dense_band_is_not_approximate() {
    let x = [0.0, 0.0, 1.0, 1.0];
    let y = [1.0, 3.0, 2.0, 4.0];
    let data = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("y", SVColumnRole::Y, &y)
        .build()
        .expect("the columns share one length");
    let output = SVBandPlotBuilder::new()
        .data_source(&data)
        .x("x")
        .derive()
        .values("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the band draws");

    assert!(!output.is_approximate());
}
