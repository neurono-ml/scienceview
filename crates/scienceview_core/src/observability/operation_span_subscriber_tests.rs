//! Verifies the span fields through a real subscriber (P8).
//!
//! A custom layer captures every span created during one test and the fields
//! recorded on it later, so the assertion is about what an exporter would see,
//! not about what the helper intended.

use crate::sv_backend::SVBackendKind;
use crate::sv_backend::test_support::SVCallRecorder;
use crate::sv_backend::test_support::sv_registry_with_recording_backend;
use crate::sv_data::SVColumnView;
use crate::sv_figure::SVFigureBuilder;
use crate::sv_observability::span_test_support::SVSpanRecorder;
use std::sync::Arc;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

#[test]
fn render_emits_one_span_with_the_configured_kind_and_fields() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let values = [1.0, 2.0, 3.0];
    let calls = SVCallRecorder::shared();
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Svg)
        .backend_registry(Arc::new(sv_registry_with_recording_backend(&calls)))
        .build()
        .expect("the configuration is complete");

    tracing::subscriber::with_default(subscriber, || {
        figure.render().expect("the recording backend renders");
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "one span per operation: {spans:?}");
    assert_eq!(spans[0]["backend.kind"], "svg");
    assert_eq!(spans[0]["representation"], "Dense");
    assert_eq!(spans[0]["points"], "3");
    assert_eq!(spans[0]["operation"], "render");
    assert_eq!(spans[0]["outcome"], "success");
    assert!(spans[0].contains_key("duration"), "{:?}", spans[0]);
}

#[test]
fn a_failing_operation_records_the_configured_kind_and_the_failure() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let values = [1.0, 2.0];
    let calls = SVCallRecorder::shared();
    let mut figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Svg)
        .backend_registry(Arc::new(sv_registry_with_recording_backend(&calls)))
        .build()
        .expect("the configuration is complete");
    figure
        .set_backend_kind(SVBackendKind::Html)
        .expect("html can host dense data");

    tracing::subscriber::with_default(subscriber, || {
        figure.render().expect_err("no html backend is registered");
    });

    let spans = recorder.captured();
    assert_eq!(spans.len(), 1, "{spans:?}");
    assert_eq!(spans[0]["backend.kind"], "html");
    assert!(
        spans[0]["outcome"].contains("unavailable"),
        "the failure is recorded: {:?}",
        spans[0]
    );
}
