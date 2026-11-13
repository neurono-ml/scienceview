//! Tests for the capability matrix and the build-time rejection rule.

use crate::sv_backend::SVBackendCapability;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::SVBackendRequirements;
use crate::sv_backend::sv_backend_capabilities;
use crate::sv_backend::sv_verify_backend_requirements;
use crate::sv_error::SVError;

#[test]
fn every_kind_has_a_matrix_row() {
    for backend_kind in SVBackendKind::ALL {
        let capabilities = sv_backend_capabilities(backend_kind);
        assert!(
            capabilities.export || capabilities.presentation,
            "`{backend_kind}` must at least export or present the figure"
        );
    }
}

#[test]
fn export_only_backends_are_export_only() {
    let capabilities = sv_backend_capabilities(SVBackendKind::Svg);

    assert!(capabilities.export, "`Svg` exports");
    assert!(!capabilities.events, "`Svg` has no events");
    assert!(!capabilities.scripting, "`Svg` has no scripting");
    assert!(!capabilities.streaming, "`Svg` has no streaming");
    assert!(
        !capabilities.in_place_update,
        "`Svg` cannot update in place"
    );
}

#[test]
fn the_html_row_declares_a_live_document() {
    let capabilities = sv_backend_capabilities(SVBackendKind::Html);

    assert!(capabilities.export, "`Html` exports a page");
    assert!(capabilities.events, "`Html` delivers gesture actions");
    assert!(capabilities.streaming, "`Html` accepts a snapshot");
    assert!(capabilities.in_place_update, "`Html` updates in place");
    assert!(!capabilities.scripting, "`Html` hosts no script engine");
    assert!(!capabilities.presentation, "`Html` presents no window");
}

#[test]
fn every_interactive_backend_supports_in_place_update() {
    for backend_kind in [
        SVBackendKind::Tui,
        SVBackendKind::Native,
        SVBackendKind::Wasm,
        SVBackendKind::React,
        SVBackendKind::Vue,
        SVBackendKind::Angular,
        SVBackendKind::Tauri,
        SVBackendKind::Dioxus,
    ] {
        assert!(
            sv_backend_capabilities(backend_kind).in_place_update,
            "`{backend_kind}` must update in place (PRD §4.2)"
        );
    }
}

#[test]
fn a_streaming_figure_is_rejected_by_an_export_only_backend() {
    let requirements = SVBackendRequirements::default().requiring_streaming();

    let error = sv_verify_backend_requirements(SVBackendKind::Svg, requirements)
        .expect_err("an export-only backend cannot stream");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Streaming,
        }
    );
    assert!(error.to_string().contains("streaming"));
}

#[test]
fn scripting_is_rejected_by_a_document_backend_without_scripting() {
    let requirements = SVBackendRequirements::default().requiring_scripting();

    let error = sv_verify_backend_requirements(SVBackendKind::Html, requirements)
        .expect_err("a document backend cannot host a script engine");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Scripting,
        }
    );
}

#[test]
fn a_supported_combination_is_accepted() {
    let requirements = SVBackendRequirements::default()
        .requiring_events()
        .requiring_scripting()
        .requiring_streaming()
        .requiring_in_place_update();

    assert_eq!(
        sv_verify_backend_requirements(SVBackendKind::Native, requirements),
        Ok(())
    );
}

#[test]
fn nothing_required_is_accepted_everywhere() {
    for backend_kind in SVBackendKind::ALL {
        assert_eq!(
            sv_verify_backend_requirements(backend_kind, SVBackendRequirements::default()),
            Ok(())
        );
    }
}

#[test]
fn the_first_unmet_capability_in_declaration_order_is_reported() {
    let requirements = SVBackendRequirements::default()
        .requiring_scripting()
        .requiring_streaming();

    let error = sv_verify_backend_requirements(SVBackendKind::Svg, requirements)
        .expect_err("neither capability is supported");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Svg,
            capability: SVBackendCapability::Scripting,
        }
    );
}
