//! One operation resolves each source column exactly once.

use super::super::SVStepPlot;
use super::test_sources::{SVCountingSource, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;

#[test]
fn render_resolves_each_column_once() {
    let inner = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0, 2.0, 3.0])
        .with_dense("y", SVColumnRole::Y, &[10.0, 20.0, 30.0])
        .with_dense("series", SVColumnRole::Grouping, &[0.0, 0.0, 0.0]);
    let source = SVCountingSource::new(inner);
    let plot = SVStepPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration builds");

    let before = source.number_of_calls();
    let _ = plot.render();
    assert_eq!(
        source.number_of_calls() - before,
        3,
        "x, y and series are resolved once each"
    );
}
