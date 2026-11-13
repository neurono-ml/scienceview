//! Capability, registry and failure-path tests.

use crate::SVReactBackend;
use crate::SVReactBackendProvider;
use crate::sv_register_react_backend;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVBackendRegistry;
use scienceview_core::sv_backend::SVBackendRequirements;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderBackendProvider;
use scienceview_core::sv_backend::SVSaveRequest;
use scienceview_core::sv_backend::SVShowRequest;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_backend::sv_verify_backend_requirements;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use std::path::Path;

#[test]
fn the_backend_reports_the_react_kind_and_the_component_row() {
    let backend = SVReactBackend::new();
    let capabilities = backend.capabilities();

    assert_eq!(backend.kind(), SVBackendKind::React);
    assert_eq!(capabilities, sv_backend_capabilities(SVBackendKind::React));
    assert!(capabilities.supports(SVBackendCapability::Events));
    assert!(capabilities.supports(SVBackendCapability::Scripting));
    assert!(capabilities.supports(SVBackendCapability::Streaming));
    assert!(capabilities.supports(SVBackendCapability::InPlaceUpdate));
    assert!(capabilities.supports(SVBackendCapability::Presentation));
    assert!(!capabilities.supports(SVBackendCapability::Export));
}

#[test]
fn a_figure_requiring_component_capabilities_builds_on_react() {
    let requirements = SVBackendRequirements::default()
        .requiring_events()
        .requiring_scripting()
        .requiring_streaming()
        .requiring_in_place_update();

    assert!(sv_verify_backend_requirements(SVBackendKind::React, requirements).is_ok());
}

#[test]
fn the_provider_serves_only_the_react_kind() {
    let provider = SVReactBackendProvider::new();

    assert_eq!(
        provider
            .provide(SVBackendKind::React)
            .map(|backend| backend.kind()),
        Some(SVBackendKind::React)
    );
    assert!(provider.provide(SVBackendKind::Vue).is_none());
    assert!(provider.provide(SVBackendKind::Angular).is_none());
    assert!(provider.provide(SVBackendKind::Svg).is_none());
}

#[test]
fn registration_makes_the_backend_resolvable() {
    let registry = SVBackendRegistry::new();
    registry.register(std::sync::Arc::new(SVReactBackendProvider::new()));
    let backend = registry
        .resolve(SVBackendKind::React)
        .expect("the provider serves React");

    assert_eq!(backend.kind(), SVBackendKind::React);
    sv_register_react_backend();
    sv_register_react_backend();
}

#[test]
fn saving_reports_the_missing_export_capability() {
    let values = [1.0, 2.0];
    let error = SVReactBackend::new()
        .save(&SVSaveRequest {
            backend_kind: SVBackendKind::React,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            destination: Path::new("figure.jsx"),
        })
        .expect_err("a component is not a file export");

    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::React,
            capability: SVBackendCapability::Export,
        }
    );
}

#[test]
fn showing_reports_that_the_host_must_mount_the_component() {
    let values = [1.0, 2.0];
    let error = SVReactBackend::new()
        .show(&SVShowRequest {
            backend_kind: SVBackendKind::React,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
        })
        .expect_err("a component cannot mount itself");

    match error {
        SVError::BackendFailure {
            backend_kind,
            operation,
            message,
        } => {
            assert_eq!(backend_kind, SVBackendKind::React);
            assert_eq!(operation, "show");
            assert!(message.contains("host application"));
        }
        other => panic!("expected a backend failure, got {other:?}"),
    }
}
