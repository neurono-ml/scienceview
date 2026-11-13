//! The render span carries the configured backend, representation and points.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use tracing::field::{Field, Visit};
use tracing::{Event, Id, Metadata, Subscriber, span};

use super::SVBoxPlotBuilder;

const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const CATEGORY: [f64; 10] = [0.0; 10];

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
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORY)
        .build()
        .expect("the columns share one length");
    let plot = SVBoxPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        plot.render().expect("the box plot draws");
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
    assert_eq!(render.get("points").map(String::as_str), Some("10"));
}

#[test]
fn render_span_counts_only_the_finite_observations() {
    let values = [1.0, f64::NAN, 3.0, 4.0, 5.0];
    let category = [0.0; 5];
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &category)
        .build()
        .expect("the columns share one length");
    let plot = SVBoxPlotBuilder::new()
        .data_source(&table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        let output = plot.render().expect("the box plot draws");
        assert_eq!(output.number_of_points(), 4);
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
    assert_eq!(render.get("points").map(String::as_str), Some("4"));
}
