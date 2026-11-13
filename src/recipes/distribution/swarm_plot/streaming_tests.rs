//! The streaming path: prompt renders and an exportable window snapshot.

use super::SVSwarmPlot;
use super::test_support::SVTestSource;
use crate::sv_backend::SVBackendKind;
use crate::sv_data::{SVColumnRole, SVColumnView, SVRepresentation, SVStreamingView};
use std::sync::mpsc;
use std::time::Duration;

const VALUES: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
const CATEGORY: [f64; 5] = [0.0, 0.0, 1.0, 1.0, 1.0];

#[test]
fn a_streaming_input_returns_promptly_and_shows_the_window() {
    let source = SVTestSource::new(vec![
        (
            "value",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&VALUES, 12, 7)),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&CATEGORY, 12, 7)),
        ),
    ]);
    let plot = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    assert_eq!(plot.representation(), SVRepresentation::Streaming);

    let (sender, receiver) = mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let scene = plot.compose().expect("the streaming window renders");
            let _ = sender.send(scene.number_of_marks());
        });
    });

    let points = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("the render must not block on the stream");
    assert_eq!(points, VALUES.len());
}

#[cfg(feature = "render-svg")]
#[test]
fn a_streaming_scene_can_be_serialized_as_svg() {
    let source = SVTestSource::new(vec![
        (
            "value",
            SVColumnRole::Y,
            SVColumnView::Streaming(SVStreamingView::new(&VALUES, 3, 0)),
        ),
        (
            "category",
            SVColumnRole::Grouping,
            SVColumnView::Streaming(SVStreamingView::new(&CATEGORY, 3, 0)),
        ),
    ]);
    let scene = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the streaming scene renders");

    let svg = scene.to_svg().expect("the SVG path serializes");
    assert!(svg.starts_with("<svg"));
}
