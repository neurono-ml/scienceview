//! Rendering the same figure from several threads is safe.

use super::test_support::{SVJointTestSource, sv_build_default};

#[test]
fn the_same_figure_renders_from_several_threads() {
    let source = SVJointTestSource::dense();
    let plot = sv_build_default(&source);
    let expected = plot.compose().expect("the figure composes");

    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let plot = &plot;
                scope.spawn(move || plot.compose().expect("the figure composes concurrently"))
            })
            .collect();
        for handle in handles {
            let scene = handle.join().expect("the render thread did not panic");
            assert_eq!(scene, expected);
        }
    });
}
