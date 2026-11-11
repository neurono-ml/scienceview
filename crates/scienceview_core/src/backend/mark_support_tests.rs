//! Executable specification of which marks each backend can render.

use super::{SVBackendKind, sv_mark_supported, sv_verify_mark_supported};
use crate::sv_error::SVError;
use crate::sv_mark::SVMarkKind;

/// Every mark except the raster image.
const VECTOR_MARKS: [SVMarkKind; 8] = [
    SVMarkKind::Dot,
    SVMarkKind::Line,
    SVMarkKind::Area,
    SVMarkKind::Bar,
    SVMarkKind::Ribbon,
    SVMarkKind::Text,
    SVMarkKind::Wedge,
    SVMarkKind::ErrorBar,
];

#[test]
fn every_backend_renders_the_eight_vector_marks() {
    for backend_kind in SVBackendKind::ALL {
        for mark_kind in VECTOR_MARKS {
            assert!(
                sv_mark_supported(mark_kind, backend_kind),
                "{backend_kind} must render {mark_kind}"
            );
            assert_eq!(sv_verify_mark_supported(mark_kind, backend_kind), Ok(()));
        }
    }
}

#[test]
fn the_terminal_backend_rejects_the_image_mark() {
    let error = sv_verify_mark_supported(SVMarkKind::Image, SVBackendKind::Tui)
        .expect_err("the cell grid cannot embed a raster");

    assert_eq!(
        error,
        SVError::UnsupportedMark {
            backend_kind: SVBackendKind::Tui,
            mark_kind: SVMarkKind::Image,
        }
    );
    assert!(!sv_mark_supported(SVMarkKind::Image, SVBackendKind::Tui));
    assert!(error.to_string().contains("tui"));
    assert!(error.to_string().contains("Image"));
}

#[test]
fn every_backend_that_can_embed_a_raster_supports_the_image_mark() {
    for backend_kind in SVBackendKind::ALL {
        if backend_kind == SVBackendKind::Tui {
            continue;
        }

        assert!(
            sv_mark_supported(SVMarkKind::Image, backend_kind),
            "{backend_kind} can embed a raster"
        );
        assert_eq!(
            sv_verify_mark_supported(SVMarkKind::Image, backend_kind),
            Ok(())
        );
    }
}

#[test]
fn the_kind_lists_every_mark_exactly_once() {
    assert_eq!(SVMarkKind::ALL.len(), 9);
    assert_eq!(SVMarkKind::ALL[0], SVMarkKind::Dot);
    assert_eq!(SVMarkKind::ALL[8], SVMarkKind::ErrorBar);
}
