//! Tests for the operation span contract.
//!
//! The field names are part of the observability contract (P8), so they are
//! asserted here without any subscriber: a span knows its own metadata.

use crate::sv_backend::SVBackendKind;
use crate::sv_figure::SVDataRepresentation;
use crate::sv_observability::sv_start_operation_span;

fn span_field_names(span: &tracing::Span) -> Vec<&'static str> {
    let metadata = span.metadata().expect("the span carries metadata");
    metadata
        .fields()
        .iter()
        .map(|field| field.name())
        .collect()
}

#[test]
fn an_operation_span_carries_the_four_documented_fields() {
    let span = sv_start_operation_span(
        "render",
        SVBackendKind::Svg,
        SVDataRepresentation::Dense,
        3,
    );

    let field_names = span_field_names(&span);

    assert!(field_names.contains(&"backend.kind"), "{field_names:?}");
    assert!(field_names.contains(&"representation"), "{field_names:?}");
    assert!(field_names.contains(&"points"), "{field_names:?}");
    assert!(field_names.contains(&"duration"), "{field_names:?}");
}

#[test]
fn the_span_name_is_the_operation_name() {
    let span = sv_start_operation_span(
        "update",
        SVBackendKind::Tui,
        SVDataRepresentation::Streaming,
        12,
    );

    assert_eq!(span.metadata().expect("metadata").name(), "update");
}
