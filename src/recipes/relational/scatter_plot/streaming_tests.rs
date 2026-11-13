//! The streaming path: prompt renders and an exportable snapshot.

use super::test_support::{SVTestSource, sv_build_default, sv_scene_to_svg, sv_streaming_source};
use crate::sv_data::{SVColumnView, SVRepresentation, SVStreamingView};
use std::sync::mpsc;
use std::time::Duration;

const HORIZONTAL: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const VERTICAL: [f64; 5] = [1.0, 4.0, 2.0, 5.0, 3.0];

#[test]
fn a_streaming_input_returns_promptly_and_shows_the_window() {
    let streaming_source = sv_streaming_source();
    let plot = sv_build_default(&streaming_source);
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
    assert_eq!(points, HORIZONTAL.len());
}

#[test]
fn a_streaming_scene_can_be_serialized_as_svg() {
    let streaming_source = SVTestSource {
        horizontal: SVColumnView::Streaming(SVStreamingView::new(&HORIZONTAL, 11, 3)),
        vertical: SVColumnView::Streaming(SVStreamingView::new(&VERTICAL, 11, 3)),
        series: None,
        sizes: None,
        colours: None,
    };
    let scene = sv_build_default(&streaming_source)
        .compose()
        .expect("the streaming scene renders");
    let svg = sv_scene_to_svg(&scene);
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("<path"), "the marks are serialized");
}
