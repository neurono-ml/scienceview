//! The declared capability and mark matrices for a terminal backend.

use crate::backend::SVTuiBackend;
use crate::cell::SVAnsiColourDepth;
use scienceview_core::sv_backend::SVBackendCapability;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::sv_backend_capabilities;
use scienceview_core::sv_backend::sv_mark_supported;
use scienceview_core::sv_mark::SVMarkKind;

#[test]
fn the_terminal_backend_declares_its_matrix_row() {
    let backend = SVTuiBackend::new();
    let capabilities = backend.capabilities();

    assert_eq!(backend.kind(), SVBackendKind::Tui);
    assert!(capabilities.events);
    assert!(!capabilities.scripting);
    assert!(capabilities.streaming);
    assert!(capabilities.in_place_update);
    assert!(!capabilities.export, "a terminal is not an export format");
    assert!(capabilities.presentation);
    assert_eq!(capabilities, sv_backend_capabilities(SVBackendKind::Tui));
}

#[test]
fn every_declared_capability_is_reported_through_the_matrix() {
    let capabilities = SVTuiBackend::new().capabilities();

    assert!(capabilities.supports(SVBackendCapability::Events));
    assert!(capabilities.supports(SVBackendCapability::Streaming));
    assert!(capabilities.supports(SVBackendCapability::InPlaceUpdate));
    assert!(capabilities.supports(SVBackendCapability::Presentation));
    assert!(!capabilities.supports(SVBackendCapability::Scripting));
    assert!(!capabilities.supports(SVBackendCapability::Export));
}

#[test]
fn the_backend_renders_every_mark_except_the_borrowed_raster() {
    for mark_kind in [
        SVMarkKind::Dot,
        SVMarkKind::Line,
        SVMarkKind::Area,
        SVMarkKind::Bar,
        SVMarkKind::Ribbon,
        SVMarkKind::Text,
        SVMarkKind::Wedge,
        SVMarkKind::ErrorBar,
    ] {
        assert!(
            sv_mark_supported(mark_kind, SVBackendKind::Tui),
            "the cell grid approximates the {mark_kind} mark"
        );
    }

    assert!(
        !sv_mark_supported(SVMarkKind::Image, SVBackendKind::Tui),
        "a borrowed bitmap cannot be shown honestly on a cell grid"
    );
}

#[test]
fn the_terminal_kind_is_a_runtime_value_without_a_default() {
    assert_eq!(SVBackendKind::from_name("tui"), Some(SVBackendKind::Tui));
    assert_eq!(SVBackendKind::from_name("automatic"), None);
}

#[test]
fn every_colour_depth_produces_escape_sequences() {
    let colour = scienceview_core::sv_scale::SVColour::new(9, 105, 218, 255);

    for depth in [
        SVAnsiColourDepth::TrueColor,
        SVAnsiColourDepth::Ansi256,
        SVAnsiColourDepth::Ansi16,
    ] {
        assert!(crate::cell::sv_ansi_colour_code(colour, depth).is_some());
    }
}
