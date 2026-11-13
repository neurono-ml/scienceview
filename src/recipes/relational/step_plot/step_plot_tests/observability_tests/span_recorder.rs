//! A subscriber that captures the fields of every span.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use tracing::field::{Field, Visit};
use tracing::{Id, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

/// Serializes the tests that install a subscriber for the shared recorder.
pub(super) static SV_SPAN_TEST_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn sv_span_test_guard() -> MutexGuard<'static, ()> {
    SV_SPAN_TEST_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// Captures span fields as an exporter would read them.
#[derive(Clone, Default)]
pub(super) struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<BTreeMap<String, String>>>>,
    identifiers: Arc<Mutex<BTreeMap<u64, usize>>>,
}

impl SVSpanRecorder {
    pub(super) fn captured(&self) -> Vec<BTreeMap<String, String>> {
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
