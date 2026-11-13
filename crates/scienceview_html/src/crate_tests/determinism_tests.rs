//! Determinism tests: the same figure always produces the same bytes.

use crate::SVHtmlBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

const VALUES: [f64; 6] = [0.0, 1.0, 4.0, 9.0, 16.0, 25.0];

fn render() -> Vec<u8> {
    SVHtmlBackend::new()
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&VALUES),
            title: Some("Deterministic"),
        })
        .expect("the backend renders")
        .bytes
}

#[test]
fn rendering_the_same_figure_twice_is_byte_identical() {
    assert_eq!(render(), render());
}

#[test]
fn an_update_payload_is_byte_identical_across_calls() {
    let backend = SVHtmlBackend::new();
    let update = || {
        backend
            .update(&SVUpdateRequest {
                backend_kind: SVBackendKind::Html,
                execution_mode: SVExecutionMode::Automatic,
                view: SVColumnView::Dense(&VALUES),
                appended_points: 3,
            })
            .expect("the backend updates")
            .bytes
    };

    assert_eq!(update(), update());
}

#[test]
fn store_custom_properties_follow_a_fixed_order() {
    let html = String::from_utf8(render()).expect("the document is UTF-8");
    let background = html.find("--sv-colour-background").expect("background");
    let accent = html.find("--sv-colour-accent").expect("accent");
    let font = html.find("--sv-font-family").expect("font");
    let spacing = html.find("--sv-spacing-medium").expect("spacing");

    assert!(background < accent && accent < font && font < spacing);
}
