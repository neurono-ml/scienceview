//! Size sanity: a large figure stays a single, reasonable document.

use crate::SVHtmlBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;

const MAXIMUM_DOCUMENT_BYTES: usize = 1_048_576;

#[test]
fn ten_thousand_points_fit_in_one_megabyte() {
    let values: Vec<f64> = (0..10_000)
        .map(|index| (index as f64 / 100.0).sin())
        .collect();
    let document = SVHtmlBackend::new()
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            title: Some("Large"),
        })
        .expect("the backend renders a large figure");

    assert_eq!(document.backend_kind, SVBackendKind::Html);
    assert!(
        document.bytes.len() < MAXIMUM_DOCUMENT_BYTES,
        "a 10k-point document must stay under 1 MiB, got {} bytes",
        document.bytes.len()
    );
    let html = String::from_utf8(document.bytes).expect("the document is UTF-8");
    assert!(
        html.contains("\"points\":10000"),
        "the payload carries every point"
    );
}
