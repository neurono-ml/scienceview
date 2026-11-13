//! The render span carries the configured backend, representation and points.

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};

use super::SVPointPlotBuilder;

const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];

#[path = "observability_support.rs"]
mod support;

#[test]
fn render_span_records_backend_representation_and_points() {
    support::sv_keep_callsite_interest_enabled();
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .build()
        .expect("the columns share one length");
    let plot = SVPointPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = support::SVRecorder::default();
    let captured = recorder.clone();
    let output = support::sv_capture_render(recorder, || plot.render());

    let render = support::sv_render_span(&captured);
    assert_eq!(render.get("backend.kind").map(String::as_str), Some("svg"));
    assert_eq!(
        render.get("representation").map(String::as_str),
        Some("Dense")
    );
    assert_eq!(render.get("points").map(String::as_str), Some("6"));
    assert_eq!(output.number_of_points(), 6);
}

#[test]
fn render_span_counts_only_the_finite_observations() {
    support::sv_keep_callsite_interest_enabled();
    let values = [1.0, f64::NAN, 3.0, 4.0];
    let categories = [0.0; 4];
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the columns share one length");
    let plot = SVPointPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = support::SVRecorder::default();
    let captured = recorder.clone();
    let output = support::sv_capture_render(recorder, || plot.render());

    let render = support::sv_render_span(&captured);
    assert_eq!(render.get("points").map(String::as_str), Some("3"));
    assert_eq!(output.number_of_points(), 3);
}
