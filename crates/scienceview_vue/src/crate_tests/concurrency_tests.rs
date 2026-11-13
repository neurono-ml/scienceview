//! Concurrent renders: one stateless backend serves many threads.

use crate::SVVueBackend;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_figure::SVExecutionMode;
use std::sync::Arc;
use std::thread;

#[test]
fn one_backend_serves_concurrent_renders_with_identical_source() {
    let values: Vec<f64> = (0..64).map(|index| (index as f64 * 0.25).sin()).collect();
    let backend = Arc::new(SVVueBackend::new());
    let mut handles = Vec::new();

    for _ in 0..8 {
        let backend = Arc::clone(&backend);
        let values = values.clone();
        handles.push(thread::spawn(move || {
            backend.render(&SVRenderRequest {
                backend_kind: SVBackendKind::Vue,
                execution_mode: SVExecutionMode::Automatic,
                view: SVColumnView::Dense(&values),
                title: Some("Concurrent"),
            })
        }));
    }

    let outputs: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().expect("the worker does not panic"))
        .collect::<Result<Vec<_>, _>>()
        .expect("every render succeeds");

    let first = &outputs[0];
    for output in &outputs {
        assert_eq!(output, first, "concurrent renders are deterministic");
    }
}
