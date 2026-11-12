//! Proves the operation span records the configured backend kind.
//!
//! The native backend is the validation target of the library, so its
//! operations must carry `backend.kind=native` into an exporter even when the
//! frame is produced headlessly.

use crate::backend::sv_register_native_backend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVFigureBuilder;
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
use tracing_subscriber::registry::LookupSpan;

type SVSpanFields = BTreeMap<String, String>;

#[derive(Clone, Default)]
struct SVSpanRecorder {
    spans: Arc<Mutex<Vec<SVSpanFields>>>,
    identifiers: Arc<Mutex<BTreeMap<u64, usize>>>,
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
fn rendering_a_figure_records_the_native_kind_in_its_span() {
    sv_register_native_backend();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let values = [1.0, 2.0, 3.0];
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Native)
        .title("Observed")
        .build()
        .expect("the configuration is complete");

    tracing::subscriber::with_default(subscriber, || {
        figure.render().expect("the native backend renders");
    });

    let spans = recorder
        .spans
        .lock()
        .expect("the recorder mutex is not poisoned")
        .clone();
    let render_spans: Vec<_> = spans
        .iter()
        .filter(|span| span.get("operation").map(String::as_str) == Some("render"))
        .collect();
    assert_eq!(
        render_spans.len(),
        1,
        "one render span per operation: {spans:?}"
    );
    assert_eq!(render_spans[0]["backend.kind"], "native");
    assert_eq!(render_spans[0]["outcome"], "success");
    assert_eq!(render_spans[0]["points"], "3");
}
