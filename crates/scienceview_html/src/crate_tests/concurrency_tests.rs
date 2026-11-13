//! Concurrency tests: the stateless backend serves concurrent renders.

use crate::SVHtmlBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use std::sync::Arc;

#[test]
fn concurrent_renders_of_one_figure_agree() {
    let values: Vec<f64> = (0..512).map(|index| (index as f64).sin()).collect();
    let backend = Arc::new(SVHtmlBackend::new());
    let expected = backend
        .render(&SVRenderRequest {
            backend_kind: SVBackendKind::Html,
            execution_mode: SVExecutionMode::Automatic,
            view: SVColumnView::Dense(&values),
            title: Some("Concurrent"),
        })
        .expect("the backend renders")
        .bytes;

    let threads: Vec<_> = (0..8)
        .map(|_| {
            let backend = Arc::clone(&backend);
            let values = values.clone();
            std::thread::spawn(move || {
                backend
                    .render(&SVRenderRequest {
                        backend_kind: SVBackendKind::Html,
                        execution_mode: SVExecutionMode::Automatic,
                        view: SVColumnView::Dense(&values),
                        title: Some("Concurrent"),
                    })
                    .expect("the backend renders")
                    .bytes
            })
        })
        .collect();

    for thread in threads {
        let rendered = thread.join().expect("the render thread does not panic");
        assert_eq!(rendered, expected);
    }
}
