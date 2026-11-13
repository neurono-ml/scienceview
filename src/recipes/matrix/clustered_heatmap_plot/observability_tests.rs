//! The operation span a clustered heatmap emits (patterns P8).

use super::test_support::{sv_build_default, sv_dense_source};
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
            .insert(id.into_u64(), index);
    }

    fn on_record(&self, id: &Id, values: &tracing::span::Record<'_>, _context: Context<'_, S>) {
        let index = self
            .identifiers
            .lock()
            .expect("the identifier mutex is not poisoned")
            .get(&id.into_u64())
            .copied();
        if let Some(index) = index {
            let mut visitor = SVFieldVisitor::default();
            values.record(&mut visitor);
            let mut spans = self
                .spans
                .lock()
                .expect("the recorder mutex is not poisoned");
            if let Some(fields) = spans.get_mut(index) {
                fields.extend(visitor.fields);
            }
        }
    }
}

#[test]
fn the_compose_span_records_the_backend_representation_and_drawn_cells() {
    let _guard = sv_span_test_guard();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let _subscriber_guard = tracing::subscriber::set_default(subscriber);

    let source = sv_dense_source();
    sv_build_default(&source)
        .compose()
        .expect("the clustered heatmap composes");

    let spans = recorder.captured();
    let span = spans
        .iter()
        .find(|span| {
            span.get("operation")
                .is_some_and(|value| value.contains("compose"))
        })
        .expect("the compose span was captured");
    assert_eq!(span.get("backend.kind").map(String::as_str), Some("svg"));
    assert_eq!(
        span.get("representation").map(String::as_str),
        Some("Dense")
    );
    assert_eq!(span.get("points").map(String::as_str), Some("9"));
    assert_eq!(span.get("outcome").map(String::as_str), Some("success"));
}
