//! The render span carries the configured backend, representation and points.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use tracing::field::{Field, Visit};
use tracing::{Event, Id, Metadata, Subscriber, span};

use super::SVBandPlotBuilder;

#[derive(Clone, Default)]
struct SVRecorder {
    spans: Arc<Mutex<Vec<BTreeMap<String, String>>>>,
}

struct SVVisitor<'fields> {
    fields: &'fields mut BTreeMap<String, String>,
}

impl Visit for SVVisitor<'_> {
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

impl Subscriber for SVRecorder {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attributes: &span::Attributes<'_>) -> Id {
        let mut fields = BTreeMap::new();
        attributes.record(&mut SVVisitor {
            fields: &mut fields,
        });
        let mut spans = self.spans.lock().unwrap_or_else(PoisonError::into_inner);
        spans.push(fields);
        Id::from_u64(spans.len() as u64)
    }

    fn record(&self, identifier: &Id, values: &span::Record<'_>) {
        let mut spans = self.spans.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(fields) = spans.get_mut(identifier.into_u64() as usize - 1) {
            values.record(&mut SVVisitor { fields });
        }
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

#[test]
fn render_span_records_backend_representation_and_points() {
    let x = [0.0, 1.0, 2.0];
    let lower = [0.5, 0.8, 1.2];
    let upper = [1.5, 2.2, 3.1];
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let plot = SVBandPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        plot.render().expect("the band draws");
    });

    let spans = captured
        .spans
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let render = spans
        .iter()
        .find(|span| span.get("operation") == Some(&"render".to_string()))
        .expect("the render operation emits a span");
    assert_eq!(render.get("backend.kind").map(String::as_str), Some("svg"));
    assert_eq!(
        render.get("representation").map(String::as_str),
        Some("Dense")
    );
    assert_eq!(render.get("points").map(String::as_str), Some("3"));
}

#[test]
fn render_span_counts_only_drawn_points() {
    let x = [0.0, 1.0, 2.0];
    let lower = [0.5, f64::NAN, 1.2];
    let upper = [1.5, 2.2, 3.1];
    let table = SVDataTable::builder()
        .with_column("x", SVColumnRole::X, &x)
        .with_column("lower", SVColumnRole::Y, &lower)
        .with_column("upper", SVColumnRole::Y, &upper)
        .build()
        .expect("the columns share one length");
    let plot = SVBandPlotBuilder::new()
        .data_source(&table)
        .x("x")
        .lower("lower")
        .upper("upper")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        let output = plot.render().expect("the band draws");
        assert_eq!(output.number_of_points(), 2);
    });

    let spans = captured
        .spans
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    let render = spans
        .iter()
        .find(|span| span.get("operation") == Some(&"render".to_string()))
        .expect("the render operation emits a span");
    assert_eq!(
        render.get("points").map(String::as_str),
        Some("2"),
        "the span must count drawn vertices, not the omitted sample"
    );
}
