//! Concurrent rendering of one image plot from several threads.

use super::test_support::{SVTestSource, sv_build_default, sv_scene_to_svg};

#[test]
fn several_threads_render_the_same_raster_identically() {
    let source = SVTestSource::dense();
    let plot = sv_build_default(&source);
    let expected = sv_scene_to_svg(&plot.compose().expect("the first render succeeds"));

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let plot = &plot;
                scope.spawn(move || {
                    (0..16)
                        .map(|_| {
                            sv_scene_to_svg(&plot.compose().expect("a concurrent render succeeds"))
                        })
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
fn a_cloned_image_plot_renders_the_same_scene() {
    let source = SVTestSource::dense();
    let plot = sv_build_default(&source);
    let clone = plot.clone();

    let first = sv_scene_to_svg(&plot.compose().expect("the original renders"));
    let second = sv_scene_to_svg(&clone.compose().expect("the clone renders"));
    assert_eq!(first, second);
}
