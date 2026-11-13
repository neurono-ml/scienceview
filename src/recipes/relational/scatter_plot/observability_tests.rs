//! The one operation span a scatter render emits (patterns P8).

use super::SVScatterPlot;
use super::test_support::{sv_dense_source, sv_streaming_source};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use tracing::field::{Field, Visit};
use tracing::{Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt};

type SVSpanFields = BTreeMap<String, String>;

static SV_SPAN_TEST_LOCK: Mutex<()> = Mutex::new(());

fn sv_span_test_guard() -> std::sync::MutexGuard<'static, ()> {
    SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[derive(Clone, Default)]
struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<SVSpanFields>>>,
    identifiers: Arc<Mutex<BTreeMap<u64, usize>>>,
}

impl SVSpanRecorder {
    fn captured(&self) -> Vec<SVSpanFields> {
        self.spans
            .lock()
            .expect("the recorder mutex is not poisoned")
            .clone()
    }
}

#[derive(Default)]
struct SVFieldVisitor {
    fields: SVSpanFields,
}

impl Visit for SVFieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{value:?}"));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }
}

impl<S: Subscriber + for<'lookup> LookupSpan<'lookup>> Layer<S> for SVSpanRecorder {
    fn on_new_span(
        &self,
        attributes: &tracing::span::Attributes<'_>,
        id: &Id,
        _context: Context<'_, S>,
    ) {
        let mut visitor = SVFieldVisitor::default();
        attributes.record(&mut visitor);

        let index = {
            let mut spans = self
                .spans
                .lock()
                .expect("the recorder mutex is not poisoned");
            spans.push(visitor.fields);
            spans.len() - 1
        };

        self.identifiers
            .lock()
            .expect("the identifier mutex is not poisoned")
            .insert(id.clone().into_u64(), index);
    }

    fn on_record(&self, id: &Id, values: &tracing::span::Record<'_>, _context: Context<'_, S>) {
        let index = self
            .identifiers
            .lock()
            .expect("the identifier mutex is not poisoned")
            .get(&id.clone().into_u64())
            .copied();
        let Some(index) = index else {
            return;
        };

        let mut visitor = SVFieldVisitor::default();
        values.record(&mut visitor);
        let mut spans = self
            .spans
            .lock()
            .expect("the recorder mutex is not poisoned");
        spans[index].extend(visitor.fields);
    }
}

#[test]
fn a_render_records_the_backend_representation_and_point_count() {
    let _guard = sv_span_test_guard();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let source = sv_streaming_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let scene = tracing::subscriber::with_default(subscriber, || {
        plot.render().expect("the scatter renders")
    });

    assert_eq!(scene.number_of_marks(), 5);
    let spans = recorder.captured();
    let render_spans: Vec<_> = spans
        .iter()
        .filter(|span| span.get("operation").map(String::as_str) == Some("render"))
        .collect();
    assert_eq!(render_spans.len(), 1, "one render span per operation");
    assert_eq!(render_spans[0]["backend.kind"], "native");
    assert_eq!(render_spans[0]["representation"], "Streaming");
    assert_eq!(render_spans[0]["points"], "5");
    assert_eq!(render_spans[0]["outcome"], "success");
}

#[test]
fn the_span_reports_the_dense_representation_and_the_configured_kind() {
    let _guard = sv_span_test_guard();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let source = sv_dense_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    tracing::subscriber::with_default(subscriber, || {
        plot.render().expect("the scatter renders");
    });

    let spans = recorder.captured();
    let span = spans
        .iter()
        .find(|span| span.get("operation").map(String::as_str) == Some("render"))
        .expect("a render span was recorded");
    assert_eq!(span["backend.kind"], "svg");
    assert_eq!(span["representation"], SVRepresentation::Dense.to_string());
}
