//! Tests for the operation span contract.
//!
//! The field names are part of the observability contract (P8), so they are
//! asserted here without any subscriber: a span knows its own metadata.

use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVRepresentation;
use crate::sv_observability::SV_OPERATION_SPAN_NAME;
use crate::sv_observability::sv_start_operation_span;
use tracing::subscriber::with_default;
use tracing_subscriber::Registry;

/// Runs `test_body` with a permissive subscriber installed, so the spans are
/// enabled and carry metadata. The subscriber is scoped to this thread, so the
/// tests do not fight over process-wide state.
fn with_enabled_spans<T>(test_body: impl FnOnce() -> T) -> T {
    with_default(Registry::default(), test_body)
}

fn span_field_names(span: &tracing::Span) -> Vec<&'static str> {
    let metadata = span.metadata().expect("the span carries metadata");
    metadata.fields().iter().map(|field| field.name()).collect()
}

#[test]
fn an_operation_span_carries_the_operation_and_the_four_documented_fields() {
    let field_names = with_enabled_spans(|| {
        let span =
            sv_start_operation_span("render", SVBackendKind::Svg, SVRepresentation::Dense, 3);
        span_field_names(&span)
    });

    assert!(field_names.contains(&"operation"), "{field_names:?}");
    assert!(field_names.contains(&"backend.kind"), "{field_names:?}");
    assert!(field_names.contains(&"representation"), "{field_names:?}");
    assert!(field_names.contains(&"points"), "{field_names:?}");
    assert!(field_names.contains(&"duration"), "{field_names:?}");
}

#[test]
fn every_operation_shares_the_documented_span_name() {
    let span_name = with_enabled_spans(|| {
        let span = sv_start_operation_span(
            "update",
            SVBackendKind::Tui,
            SVRepresentation::Streaming,
            12,
        );
        span.metadata().expect("metadata").name()
    });

    assert_eq!(span_name, SV_OPERATION_SPAN_NAME);
}
