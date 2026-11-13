//! Concurrent composition and serialization of one contour chart.

use super::test_support::{sv_build_default, sv_matrix_source, sv_scene_to_svg};

#[test]
fn several_threads_serialize_the_same_chart_identically() {
    let source = sv_matrix_source();
    let plot = sv_build_default(&source);
    let expected = sv_scene_to_svg(&plot.compose().expect("the first composition succeeds"));

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let plot = &plot;
                scope.spawn(move || {
                    (0..8)
                        .map(|_| {
                            sv_scene_to_svg(
                                &plot.compose().expect("a concurrent composition succeeds"),
                            )
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        for handle in handles {
            let rendered = handle.join().expect("the worker does not panic");
            for svg in rendered {
                assert_eq!(svg, expected, "concurrent renders agree byte for byte");
            }
        }
    });
}

#[test]
fn a_cloned_chart_serializes_the_same_scene() {
    let source = sv_matrix_source();
    let plot = sv_build_default(&source);
    let clone = plot.clone();

    assert_eq!(
        sv_scene_to_svg(&plot.compose().expect("the original composes")),
        sv_scene_to_svg(&clone.compose().expect("the clone composes"))
    );
}
