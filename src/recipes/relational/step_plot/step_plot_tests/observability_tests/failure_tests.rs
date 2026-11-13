//! A failed composition still emits its operation span.

use super::super::super::SVStepPlot;
use super::super::test_sources::SVTestSource;
use super::span_recorder::{SVSpanRecorder, sv_span_test_guard};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use tracing_subscriber::{Registry, layer::SubscriberExt};

#[test]
fn a_failing_composition_still_emits_the_operation_span() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();

    let empty: [f64; 0] = [];
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &empty)
        .with_dense("y", SVColumnRole::Y, &empty);
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the empty configuration builds");

    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    tracing::subscriber::with_default(subscriber, || {
        let _ = plot.render();
    });

    let spans = recorder.captured();
    let operations: Vec<_> = spans
        .iter()
        .filter(|span| span.get("operation").is_some_and(|name| name == "render"))
        .collect();
    assert_eq!(operations.len(), 1, "one operation span: {spans:?}");
    assert_eq!(operations[0]["points"], "0");
    assert!(
        operations[0]["outcome"].contains("needs at least one observation"),
        "{:?}",
        operations[0]
    );
}
