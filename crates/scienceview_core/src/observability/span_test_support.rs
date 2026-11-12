//! A span recorder shared by the observability tests (P8).
//!
//! A custom layer captures every span created during one test and the fields
//! recorded on it later, so an assertion is about what an exporter would see,
//! not about what the helper intended.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::Id;
use tracing::Subscriber;
use tracing::field::Field;
use tracing::field::Visit;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

/// One captured span, field name to rendered value.
pub(crate) type SVSpanFields = BTreeMap<String, String>;

/// Captures span fields as an exporter would read them.
#[derive(Clone, Default)]
pub(crate) struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<SVSpanFields>>>,
    identifiers: Arc<Mutex<BTreeMap<u64, usize>>>,
}

impl SVSpanRecorder {
    /// Returns every captured span in creation order.
    pub(crate) fn captured(&self) -> Vec<SVSpanFields> {
        self.spans
            .lock()
            .expect("the recorder mutex is not poisoned")
            .clone()
    }
}

#[derive(Default)]
pub(crate) struct SVFieldVisitor {
    pub(crate) fields: SVSpanFields,
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
