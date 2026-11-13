//! The capability matrix row of the WebAssembly backend.

use crate::SVWasmBackend;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVBackendRequirements;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_backend::sv_mark_supported;
use scienceview_core::sv_backend::sv_verify_backend_requirements;
use scienceview_core::sv_mark::SVMarkKind;

#[test]
fn the_wasm_backend_declares_the_canvas_capabilities() {
    let backend = SVWasmBackend::new();
    let capabilities = backend.capabilities();

    assert_eq!(backend.kind(), SVBackendKind::Wasm);
    assert!(capabilities.events);
    assert!(capabilities.scripting);
    assert!(capabilities.streaming);
    assert!(capabilities.in_place_update);
    assert!(capabilities.presentation);
    assert!(!capabilities.export, "a canvas is not an export format");
    assert_eq!(capabilities, sv_backend_capabilities(SVBackendKind::Wasm));
}

#[test]
fn every_declared_capability_is_reported_through_the_matrix() {
    let capabilities = SVWasmBackend::new().capabilities();

    assert!(capabilities.supports(SVBackendCapability::Events));
    assert!(capabilities.supports(SVBackendCapability::Scripting));
    assert!(capabilities.supports(SVBackendCapability::Streaming));
    assert!(capabilities.supports(SVBackendCapability::InPlaceUpdate));
    assert!(capabilities.supports(SVBackendCapability::Presentation));
    assert!(!capabilities.supports(SVBackendCapability::Export));
}

#[test]
fn a_figure_requiring_animation_hosting_builds_on_wasm() {
    let requirements = SVBackendRequirements::default()
        .requiring_events()
        .requiring_scripting()
        .requiring_streaming()
        .requiring_in_place_update();

    assert!(sv_verify_backend_requirements(SVBackendKind::Wasm, requirements).is_ok());
}

#[test]
fn the_wasm_backend_renders_every_mark_variant() {
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
            sv_mark_supported(mark_kind, SVBackendKind::Wasm),
            "the shared rasterizer draws the {mark_kind} mark"
        );
    }
}

#[test]
fn the_wasm_kind_is_a_runtime_value_without_a_default() {
    assert_eq!(SVBackendKind::from_name("wasm"), Some(SVBackendKind::Wasm));
    assert_eq!(SVBackendKind::from_name("automatic"), None);
}
