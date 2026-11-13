//! Successful render spans for dense and streaming data.

use super::super::super::SVStepPlot;
use super::super::test_sources::{SVTestColumn, SVTestSource};
use super::span_recorder::{SVSpanRecorder, sv_span_test_guard};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use tracing_subscriber::{Registry, layer::SubscriberExt};

fn render_operation_spans(
    plot: &SVStepPlot<'_>,
    recorder: &SVSpanRecorder,
) -> Vec<std::collections::BTreeMap<String, String>> {
    let subscriber = Registry::default().with(recorder.clone());
    tracing::subscriber::with_default(subscriber, || {
        let _ = plot.render();
    });
    recorder
        .captured()
        .into_iter()
        .filter(|span| span.get("operation").is_some_and(|name| name == "render"))
        .collect()
}

#[test]
fn render_records_the_configured_kind_representation_and_points() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();

    let horizontal = [1.0, 2.0, 3.0];
    let vertical = [10.0, 20.0, 30.0];
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &horizontal)
        .with_dense("y", SVColumnRole::Y, &vertical);
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds");

    let recorder = SVSpanRecorder::default();
    let operations = render_operation_spans(&plot, &recorder);
    assert_eq!(operations.len(), 1, "{operations:?}");
    assert_eq!(operations[0]["backend.kind"], "svg");
    assert_eq!(operations[0]["representation"], "Dense");
    assert_eq!(operations[0]["points"], "3");
    assert!(
        operations[0]["outcome"] == "success" || operations[0]["outcome"].contains("unavailable"),
        "{:?}",
        operations[0]
    );
    assert!(
        operations[0].contains_key("duration"),
        "{:?}",
        operations[0]
    );
}

#[test]
fn a_streaming_render_records_the_streaming_representation() {
    let _guard = sv_span_test_guard();
    tracing::callsite::rebuild_interest_cache();

    let horizontal = [1.0, 2.0, 3.0];
    let vertical = [10.0, 20.0, 30.0];
    let table = SVTestSource::new()
        .with_column(
            "x",
            SVColumnRole::X,
            SVTestColumn::Streaming(&horizontal, 7),
        )
        .with_column("y", SVColumnRole::Y, SVTestColumn::Streaming(&vertical, 7));
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the streaming configuration builds");

    let recorder = SVSpanRecorder::default();
    let operations = render_operation_spans(&plot, &recorder);
    assert_eq!(operations.len(), 1, "{operations:?}");
    assert_eq!(operations[0]["representation"], "Streaming");
    assert_eq!(operations[0]["points"], "3");
}
