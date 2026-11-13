//! The operation span carries the configured kind, representation and drawn
//! marks.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_stat::{SVAggregateMethod, SVAggregateOptions};
use tracing::Subscriber;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;

use super::SVLinePlotBuilder;
use super::line_plot_test_support::{SVTestSource, sv_dense_source};

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

fn captured_points(recorder: &SVSpanRecorder) -> Vec<String> {
    recorder
        .spans
        .lock()
        .expect("the recorder mutex is not poisoned")
        .iter()
        .filter(|span| span.name == "scienceview.operation")
        .filter_map(|span| span.fields.get("points").cloned())
        .collect()
}

#[test]
fn composing_records_the_backend_representation_and_drawn_marks() {
    let recorder = SVSpanRecorder::default();
    let subscriber = registry().with(recorder.clone());
    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
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

#[test]
fn aggregation_reports_drawn_marks_not_the_logical_length() {
    let recorder = SVSpanRecorder::default();
    let subscriber = registry().with(recorder.clone());
    let x = [1.0, 1.0, 2.0, 2.0];
    let y = [1.0, 3.0, 2.0, 4.0];
    let source = sv_dense_source(&x, &y);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .aggregate(SVAggregateOptions::new().method(SVAggregateMethod::Mean))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the chart builds");

    tracing::subscriber::with_default(subscriber, || {
        plot.compose().expect("the chart composes");
    });

    assert_eq!(plot.point_count(), 4, "the logical input length");
    assert_eq!(captured_points(&recorder), vec!["2".to_string()]);
}

#[test]
fn a_streaming_window_reports_a_growing_drawn_mark_count() {
    let recorder = SVSpanRecorder::default();
    let subscriber = registry().with(recorder.clone());
    let short_x = [0.0, 1.0];
    let long_x = [0.0, 1.0, 2.0, 3.0];
    let short_y = [10.0, 20.0];
    let long_y = [10.0, 20.0, 30.0, 40.0];
    let short_source =
        sv_dense_source(&short_x, &short_x).with_streaming("value", SVColumnRole::Y, &short_y, 2);
    let long_source =
        sv_dense_source(&long_x, &long_x).with_streaming("value", SVColumnRole::Y, &long_y, 4);
    let short = SVLinePlotBuilder::new()
        .data_source(&short_source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");
    let long = SVLinePlotBuilder::new()
        .data_source(&long_source)
        .x("x")
        .y("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the chart builds");

    tracing::subscriber::with_default(subscriber, || {
        short.compose().expect("the short window composes");
        long.compose().expect("the grown window composes");
    });

    assert_eq!(
        captured_points(&recorder),
        vec!["2".to_string(), "4".to_string()]
    );
}

#[test]
fn a_streaming_span_reports_the_streaming_representation() {
    let recorder = SVSpanRecorder::default();
    let subscriber = registry().with(recorder.clone());
    let x = [0.0, 1.0];
    let y = [10.0, 20.0];
    let source = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &x)
        .with_streaming("y", SVColumnRole::Y, &y, 2);
    let plot = SVLinePlotBuilder::new()
        .data_source(&source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Native)
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
        span.fields.get("representation").map(String::as_str),
        Some("Streaming")
    );
    assert_eq!(span.fields.get("points").map(String::as_str), Some("2"));
}
