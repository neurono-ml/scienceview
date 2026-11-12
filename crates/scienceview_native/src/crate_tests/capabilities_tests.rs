//! The declared capability matrix and the build-time checks it feeds.

use crate::backend::SVNativeBackend;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVBackendRequirements;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_backend::sv_mark_supported;
use scienceview_core::sv_backend::sv_verify_backend_requirements;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::SVMarkKind;
use std::path::Path;

#[test]
fn the_native_backend_declares_every_interactive_capability() {
    let backend = SVNativeBackend::new();
    let capabilities = backend.capabilities();

    assert_eq!(backend.kind(), SVBackendKind::Native);
    assert!(capabilities.events);
    assert!(capabilities.scripting);
    assert!(capabilities.streaming);
    assert!(capabilities.in_place_update);
    assert!(capabilities.presentation);
    assert!(
        !capabilities.export,
        "a native window is not an export format"
    );
    assert_eq!(capabilities, sv_backend_capabilities(SVBackendKind::Native));
}

#[test]
fn every_declared_capability_is_reported_through_the_matrix() {
    let backend = SVNativeBackend::new();
    let capabilities = backend.capabilities();

    assert!(capabilities.supports(SVBackendCapability::Events));
    assert!(capabilities.supports(SVBackendCapability::Scripting));
    assert!(capabilities.supports(SVBackendCapability::Streaming));
    assert!(capabilities.supports(SVBackendCapability::InPlaceUpdate));
    assert!(capabilities.supports(SVBackendCapability::Presentation));
    assert!(!capabilities.supports(SVBackendCapability::Export));
}

#[test]
fn a_figure_requiring_animation_hosting_builds_on_native() {
    let requirements = SVBackendRequirements::default()
        .requiring_events()
        .requiring_streaming()
        .requiring_in_place_update();

    assert!(sv_verify_backend_requirements(SVBackendKind::Native, requirements).is_ok());
}

#[test]
fn scripting_is_declared_and_accepted_at_build_time() {
    let requirements = SVBackendRequirements::default().requiring_scripting();

    assert!(sv_verify_backend_requirements(SVBackendKind::Native, requirements).is_ok());
}

#[test]
fn saving_reports_the_missing_export_capability() {
    let values = [1.0, 2.0];
    let error = SVNativeBackend::new()
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::Native,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            destination: Path::new("frame.png"),
        })
        .expect_err("a native window declares no export format");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Native,
            capability: SVBackendCapability::Export,
        }
    );
}

#[test]
fn the_native_backend_renders_every_mark_variant() {
    for mark_kind in [
        SVMarkKind::Dot,
        SVMarkKind::Line,
        SVMarkKind::Area,
        SVMarkKind::Bar,
        SVMarkKind::Ribbon,
        SVMarkKind::Text,
        SVMarkKind::Image,
        SVMarkKind::Wedge,
        SVMarkKind::ErrorBar,
    ] {
        assert!(
            sv_mark_supported(mark_kind, SVBackendKind::Native),
            "the native CPU renderer draws the {mark_kind} mark"
        );
    }
}

#[test]
fn the_native_kind_is_a_runtime_value_without_a_default() {
    assert_eq!(
        SVBackendKind::from_name("native"),
        Some(SVBackendKind::Native)
    );
    assert_eq!(SVBackendKind::from_name("automatic"), None);
}

#[cfg(not(feature = "window"))]
#[test]
fn showing_without_the_window_feature_reports_a_backend_failure() {
    use scienceview_core::sv_backend::SVShowRequest;

    let values = [1.0, 2.0];
    let error = SVNativeBackend::new()
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::Native,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect_err("the window layer is not compiled");

    assert!(matches!(
        error,
        SVError::BackendFailure {
            backend_kind: SVBackendKind::Native,
            operation: "show",
            ..
        }
    ));
}
