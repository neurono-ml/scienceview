//! The operation span carries the configured kind, representation and points.

use super::super::SVStepPlot;
use super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};
use tracing::field::{Field, Visit};
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt};

static SV_SPAN_TEST_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Default)]
struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<BTreeMap<String, String>>>>,
    identifiers: Arc<Mutex<BTreeMap<u64, usize>>>,
}

impl SVSpanRecorder {
    fn captured(&self) -> Vec<BTreeMap<String, String>> {
        self.spans
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

#[derive(Default)]
struct SVFieldVisitor {
    fields: BTreeMap<String, String>,
}

impl Visit for SVFieldVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{value:?}"));
    }
}

impl<S: Subscriber> Layer<S> for SVSpanRecorder {
    fn on_new_span(
        &self,
        attributes: &tracing::span::Attributes<'_>,
        id: &Id,
        _context: Context<'_, S>,
    ) {
        let mut visitor = SVFieldVisitor::default();
        attributes.record(&mut visitor);
        let mut spans = self.spans.lock().unwrap_or_else(PoisonError::into_inner);
        spans.push(visitor.fields);
        let index = spans.len() - 1;
        self.identifiers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(id.clone().into_u64(), index);
    }

    fn on_record(&self, id: &Id, values: &tracing::span::Record<'_>, _context: Context<'_, S>) {
        let index = self
            .identifiers
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&id.clone().into_u64())
            .copied();
        let Some(index) = index else {
            return;
        };
        let mut visitor = SVFieldVisitor::default();
        values.record(&mut visitor);
        let mut spans = self.spans.lock().unwrap_or_else(PoisonError::into_inner);
        spans[index].extend(visitor.fields);
    }
}

#[test]
fn render_records_the_configured_kind_representation_and_points() {
    let _guard = SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
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
    let _guard = SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    tracing::callsite::rebuild_interest_cache();

    let horizontal = [1.0, 2.0, 3.0];
    let vertical = [10.0, 20.0, 30.0];
    let table = SVTestSource::new()
        .with_column(
            "x",
            SVColumnRole::X,
            super::test_sources::SVTestColumn::Streaming(&horizontal, 7),
        )
        .with_column(
            "y",
            SVColumnRole::Y,
            super::test_sources::SVTestColumn::Streaming(&vertical, 7),
        );
    let plot = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the streaming configuration builds");

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
    assert_eq!(operations.len(), 1, "{spans:?}");
    assert_eq!(operations[0]["representation"], "Streaming");
    assert_eq!(operations[0]["points"], "3");
}

#[test]
fn a_failing_composition_still_emits_the_operation_span() {
    let _guard = SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
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
