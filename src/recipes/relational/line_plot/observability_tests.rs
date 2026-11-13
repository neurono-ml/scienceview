//! The operation span carries the configured kind, representation and points.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnView;
use tracing::Subscriber;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use super::SVLinePlotBuilder;

#[derive(Clone, Default)]
struct SVCapturedSpan {
    name: String,
    fields: BTreeMap<String, String>,
}

#[derive(Clone, Default)]
struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<SVCapturedSpan>>>,
}

#[derive(Default)]
struct SVFieldVisitor {
    fields: BTreeMap<String, String>,
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
}

impl<S: Subscriber> Layer<S> for SVSpanRecorder {
    fn on_new_span(&self, attributes: &Attributes<'_>, _id: &Id, _context: Context<'_, S>) {
        let mut visitor = SVFieldVisitor::default();
        attributes.record(&mut visitor);
        self.spans
            .lock()
            .expect("the recorder mutex is not poisoned")
            .push(SVCapturedSpan {
                name: attributes.metadata().name().to_string(),
                fields: visitor.fields,
            });
    }
}

#[test]
fn composing_records_the_backend_representation_and_point_count() {
    let recorder = SVSpanRecorder::default();
    let subscriber = registry().with(recorder.clone());
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let plot = SVLinePlotBuilder::new()
        .x(SVColumnView::Dense(&x))
        .y(SVColumnView::Dense(&y))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    tracing::subscriber::with_default(subscriber, || {
        plot.compose().expect("the chart composes");
    });

    let spans = recorder
        .spans
        .lock()
        .expect("the recorder mutex is not poisoned")
        .clone();
    let span = spans
        .iter()
        .find(|span| span.name == "scienceview.operation")
        .expect("the operation span is emitted");

    assert_eq!(
        span.fields.get("operation").map(String::as_str),
        Some("compose")
    );
    assert_eq!(
        span.fields.get("backend.kind").map(String::as_str),
        Some("svg")
    );
    assert_eq!(
        span.fields.get("representation").map(String::as_str),
        Some("Dense")
    );
    assert_eq!(span.fields.get("points").map(String::as_str), Some("3"));
}
