//! Span recorder shared by the point-plot observability tests.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Once, PoisonError};

use scienceview_core::sv_error::SVError;
use tracing::field::{Field, Visit};
use tracing::{Event, Id, Metadata, Subscriber, span};

use super::super::SVPointRenderOutput;

#[derive(Clone, Default)]
pub(crate) struct SVRecorder {
    pub(crate) spans: Arc<Mutex<Vec<BTreeMap<String, String>>>>,
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

pub(crate) fn sv_keep_callsite_interest_enabled() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = tracing::subscriber::set_global_default(SVInterestSubscriber);
    });
}

/// Captures one render and returns its output.
pub(crate) fn sv_capture_render(
    recorder: SVRecorder,
    render: impl FnOnce() -> Result<SVPointRenderOutput, SVError>,
) -> SVPointRenderOutput {
    tracing::callsite::rebuild_interest_cache();
    tracing::subscriber::with_default(recorder, || {
        tracing::callsite::rebuild_interest_cache();
        render().expect("the point plot draws")
    })
}

/// Returns the recorded render span.
pub(crate) fn sv_render_span(recorder: &SVRecorder) -> BTreeMap<String, String> {
    let spans = recorder
        .spans
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    spans
        .into_iter()
        .find(|span| span.get("operation") == Some(&"render".to_string()))
        .expect("the render operation emits a span")
}
