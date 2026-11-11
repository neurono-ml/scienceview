//! Verifies the span fields through a real subscriber (P8).
//!
//! A custom layer captures every span created during one test and the fields
//! recorded on it later, so the assertion is about what an exporter would see,
//! not about what the helper intended.

use crate::sv_backend::SVBackendKind;
use crate::sv_backend::test_support::SVCallRecorder;
use crate::sv_backend::test_support::sv_registry_with_recording_backend;
use crate::sv_figure::SVDataBinding;
use crate::sv_figure::SVFigureBuilder;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::Id;
use tracing::Subscriber;
use tracing::field::Field;
use tracing::field::Visit;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::Context;
use tracing_subscriber::layer::SubscriberExt;

type SVSpanFields = BTreeMap<String, String>;

/// Captures span fields as an exporter would read them.
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

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields
            .insert(field.name().to_string(), value.to_string());
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
fn render_emits_one_span_with_the_configured_kind_and_fields() {
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let values = [1.0, 2.0, 3.0];
    let calls = SVCallRecorder::shared();
    let figure = SVFigureBuilder::new()
        .data_source(SVDataBinding::dense(&values))
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
        .data_source(SVDataBinding::dense(&values))
        .backend(SVBackendKind::Svg)
        .backend_registry(Arc::new(sv_registry_with_recording_backend(&calls)))
        .build()
        .expect("the configuration is complete");
    figure.set_backend_kind(SVBackendKind::Html);

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
