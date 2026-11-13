//! The one operation span an image composition emits (patterns P8).

use super::SVImagePlot;
use super::test_support::{SVTestSource, dimensions};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
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
        _id: &Id,
        _context: Context<'_, S>,
    ) {
        let mut visitor = SVFieldVisitor::default();
        attributes.record(&mut visitor);
        self.spans
            .lock()
            .expect("the recorder mutex is not poisoned")
            .push(visitor.fields);
    }
}

fn sv_plot(source: &dyn crate::sv_data::SVDataSource, kind: SVBackendKind) -> SVImagePlot<'_> {
    SVImagePlot::builder()
        .data_source(source)
        .raster(super::SVImageSource::intensity("field", dimensions()))
        .backend(kind)
        .build()
        .expect("the configuration is complete")
}

#[test]
fn a_compose_records_the_backend_representation_and_pixel_count() {
    let _guard = sv_span_test_guard();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let source = SVTestSource::streaming(&super::test_support::FIELD, 4);
    let plot = sv_plot(&source, SVBackendKind::Svg);

    tracing::subscriber::with_default(subscriber, || {
        plot.compose().expect("the image composes");
    });

    let spans = recorder.captured();
    let span = spans
        .iter()
        .find(|span| span.get("operation").map(String::as_str) == Some("compose"))
        .expect("a compose span was recorded");
    assert_eq!(span["backend.kind"], "svg");
    assert_eq!(
        span["representation"],
        SVRepresentation::Streaming.to_string()
    );
    assert_eq!(span["points"], super::test_support::FIELD.len().to_string());
}

#[test]
fn a_native_compose_records_the_native_kind() {
    let _guard = sv_span_test_guard();
    let recorder = SVSpanRecorder::default();
    let subscriber = Registry::default().with(recorder.clone());
    let source = SVTestSource::dense();
    let plot = sv_plot(&source, SVBackendKind::Native);

    tracing::subscriber::with_default(subscriber, || {
        plot.compose().expect("the image composes");
    });

    let spans = recorder.captured();
    let span = spans
        .iter()
        .find(|span| span.get("operation").map(String::as_str) == Some("compose"))
        .expect("a compose span was recorded");
    assert_eq!(span["backend.kind"], "native");
    assert_eq!(span["representation"], SVRepresentation::Dense.to_string());
}
