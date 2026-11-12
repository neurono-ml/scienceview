//! Concurrent renders share no state, so every thread sees the same bytes.

use crate::backend::SVSvgBackend;
use scienceview_core::sv_backend::{SVBackendKind, SVRenderBackend, SVRenderRequest};
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use std::sync::Arc;

#[test]
fn concurrent_renders_are_identical_and_thread_safe() {
    let values: Arc<Vec<f64>> = Arc::new((0..2_000).map(|index| (index as f64).sqrt()).collect());
    let expected = {
        let backend = SVSvgBackend::new();
        backend
            .render(&SVRenderRequest {
                backend_kind: SVBackendKind::Svg,
                execution_mode: SVExecutionMode::Automatic,
                view: SVColumnView::Dense(&values),
                title: Some("Concurrent"),
            })
            .expect("render")
    };

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let values = Arc::clone(&values);
            let expected = expected.clone();
            std::thread::spawn(move || {
                let backend = SVSvgBackend::new();
                let output = backend
                    .render(&SVRenderRequest {
                        backend_kind: SVBackendKind::Svg,
                        execution_mode: SVExecutionMode::Automatic,
                        view: SVColumnView::Dense(&values),
                        title: Some("Concurrent"),
                    })
                    .expect("render");
                assert_eq!(output, expected);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("every render thread finishes");
    }
}
