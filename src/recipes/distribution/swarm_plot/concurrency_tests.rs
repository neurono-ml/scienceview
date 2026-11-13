//! Concurrent rendering of one swarm from several threads.

use super::SVSwarmPlot;
use super::test_support::{dense_source, scene_svg};
use crate::sv_backend::SVBackendKind;

#[test]
fn several_threads_render_the_same_swarm_identically() {
    let source = dense_source();
    let plot = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let expected = scene_svg(&plot.compose().expect("the first render succeeds"));

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let plot = &plot;
                scope.spawn(move || {
                    (0..8)
                        .map(|_| scene_svg(&plot.compose().expect("a concurrent render succeeds")))
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        for handle in handles {
            let rendered = handle.join().expect("the render thread does not panic");
            for svg in rendered {
                assert_eq!(svg, expected, "concurrent renders agree byte for byte");
            }
        }
    });
}

#[test]
fn a_cloned_swarm_renders_the_same_scene() {
    let source = dense_source();
    let plot = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let clone = plot.clone();

    assert_eq!(
        scene_svg(&plot.compose().expect("the original renders")),
        scene_svg(&clone.compose().expect("the clone renders"))
    );
}
