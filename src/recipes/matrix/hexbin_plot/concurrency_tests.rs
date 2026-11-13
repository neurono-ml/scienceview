//! Concurrent rendering of one hexbin from several threads.

use super::test_support::{sv_build_default, sv_dense_source, sv_render_svg};

#[test]
fn several_threads_render_the_same_hexbin_identically() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let expected = sv_render_svg(&plot.compose().expect("the first render succeeds"));

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let plot = &plot;
                scope.spawn(move || {
                    (0..16)
                        .map(|_| {
                            sv_render_svg(&plot.compose().expect("a concurrent render succeeds"))
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
fn a_cloned_hexbin_renders_the_same_lattice() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let clone = plot.clone();

    assert_eq!(
        sv_render_svg(&plot.compose().expect("the original renders")),
        sv_render_svg(&clone.compose().expect("the clone renders"))
    );
}
