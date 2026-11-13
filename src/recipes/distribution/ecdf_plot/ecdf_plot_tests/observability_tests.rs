//! The operation span carries the configured backend, representation and points.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use tracing::field::{Field, Visit};
use tracing::{Event, Id, Metadata, Subscriber, span};

use super::super::SVECdfPlot;
use super::test_support::SVTestSource;

/// Serializes the tests that install a subscriber for the shared callsite.
static SV_SPAN_TEST_LOCK: Mutex<()> = Mutex::new(());

fn sv_span_test_guard() -> MutexGuard<'static, ()> {
    SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

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
    let _guard = sv_span_test_guard();
    let values = [1.0, 2.0, 3.0, 4.0];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        plot.render().expect("the curve draws");
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
    assert_eq!(render.get("points").map(String::as_str), Some("4"));
}

#[test]
fn render_span_counts_only_finite_observations() {
    let _guard = sv_span_test_guard();
    let values = [1.0, f64::NAN, 3.0];
    let table = SVTestSource::new().with_dense("value", SVColumnRole::Y, &values);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .backend(SVBackendKind::Native)
        .build()
        .expect("the configuration is complete");

    let recorder = SVRecorder::default();
    let captured = recorder.clone();
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        plot.render().expect("the curve draws");
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
        render.get("backend.kind").map(String::as_str),
        Some("native")
    );
    assert_eq!(render.get("points").map(String::as_str), Some("2"));
}
