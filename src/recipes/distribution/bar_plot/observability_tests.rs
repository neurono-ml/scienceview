//! The render span carries the configured backend, representation and points.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Once, PoisonError};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use tracing::field::{Field, Visit};
use tracing::{Event, Id, Metadata, Subscriber, span};

use super::SVBarPlotBuilder;

const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];

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

/// Keeps the global callsite interest enabled for the whole test binary.
struct SVInterestSubscriber;

impl Subscriber for SVInterestSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _attributes: &span::Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _identifier: &Id, _values: &span::Record<'_>) {}

    fn record_follows_from(&self, _identifier: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, _identifier: &Id) {}

    fn exit(&self, _identifier: &Id) {}
}

fn sv_keep_callsite_interest_enabled() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = tracing::subscriber::set_global_default(SVInterestSubscriber);
    });
}

#[test]
fn render_span_records_backend_representation_and_points() {
    sv_keep_callsite_interest_enabled();
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .build()
        .expect("the columns share one length");
    let plot = SVBarPlotBuilder::new()
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
        plot.render().expect("the bar plot draws");
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
    assert_eq!(render.get("points").map(String::as_str), Some("6"));
}

#[test]
fn render_span_counts_only_the_finite_observations() {
    sv_keep_callsite_interest_enabled();
    let values = [1.0, f64::NAN, 3.0, 4.0];
    let categories = [0.0; 4];
    let table = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the columns share one length");
    let plot = SVBarPlotBuilder::new()
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
        let output = plot.render().expect("the bar plot draws");
        assert_eq!(output.number_of_points(), 3);
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
    assert_eq!(render.get("points").map(String::as_str), Some("3"));
}
