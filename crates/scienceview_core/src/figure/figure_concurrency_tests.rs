//! Concurrency acceptance for a built figure (PRD §9).
//!
//! One built figure is shared by several threads and rendered concurrently.
//! The figure is immutable during a render and holds its backend through an
//! `Arc`, so every thread resolves the same kind and receives its own output.

use super::SVFigureBuilder;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::test_support::SVCallRecorder;
use crate::sv_backend::test_support::sv_registry_with_recording_backend;
use crate::sv_data::SVColumnView;
use std::sync::Arc;
use std::thread;

const THREAD_COUNT: usize = 8;

#[test]
fn one_built_figure_renders_safely_from_several_threads() {
    let values: Vec<f64> = (0..1024).map(f64::from).collect();
    let recorder = SVCallRecorder::shared();
    let figure = SVFigureBuilder::new()
        .data_source(SVColumnView::Dense(&values))
        .backend(SVBackendKind::Svg)
        .backend_registry(Arc::new(sv_registry_with_recording_backend(&recorder)))
        .build()
        .expect("the configuration is complete");

    let figure = Arc::new(figure);
    let outputs = thread::scope(|scope| {
        let handles: Vec<_> = (0..THREAD_COUNT)
            .map(|_| {
                let figure = Arc::clone(&figure);
                scope.spawn(move || figure.render().expect("each thread renders"))
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("the thread does not panic"))
            .collect::<Vec<_>>()
    });

    assert_eq!(outputs.len(), THREAD_COUNT);
    assert!(
        outputs
            .iter()
            .all(|output| output.backend_kind == SVBackendKind::Svg)
    );
    assert_eq!(recorder.render_calls(), THREAD_COUNT);
    assert_eq!(recorder.provider_lookups(), THREAD_COUNT);
}

#[test]
fn concurrent_operations_on_distinct_backends_do_not_interfere() {
    let values = [1.0, 2.0, 3.0];
    let recorder = SVCallRecorder::shared();
    let registry = Arc::new(sv_registry_with_recording_backend(&recorder));

    let builds: Vec<_> = [SVBackendKind::Svg, SVBackendKind::Html, SVBackendKind::Tui]
        .into_iter()
        .map(|backend_kind| {
            SVFigureBuilder::new()
                .data_source(SVColumnView::Dense(&values))
                .backend(backend_kind)
                .backend_registry(Arc::clone(&registry))
                .build()
                .expect("the configuration is complete")
        })
        .collect();

    let outcomes = thread::scope(|scope| {
        let handles: Vec<_> = builds
            .iter()
            .map(|figure| scope.spawn(move || figure.render().map(|_output| ())))
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("the thread does not panic"))
            .collect::<Vec<_>>()
    });

    // Only the recording backend serves `Svg`; the other kinds are unregistered.
    assert!(outcomes[0].is_ok());
    assert!(outcomes[1].is_err());
    assert!(outcomes[2].is_err());
}
