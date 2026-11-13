//! A large sample stays one compound path per paint (FDR acceptance).

use super::test_support::{SVTestRugSource, sv_build_default, sv_scene_to_svg, sv_tick_segments};

#[test]
fn a_large_sample_draws_every_tick_in_one_compound_path() {
    let values: Vec<f64> = (0..2_000).map(|index| f64::from(index) * 0.01).collect();
    let source = SVTestRugSource::dense(&values);
    let scene = sv_build_default(&source)
        .compose()
        .expect("the large rug composes");

    assert_eq!(scene.number_of_marks(), values.len());
    assert_eq!(sv_tick_segments(&scene).len(), values.len());
    assert_eq!(
        scene.number_of_mark_primitives(),
        1,
        "the ticks share one paint and are grouped into one path"
    );

    let document = sv_scene_to_svg(&scene);
    assert!(
        document.len() < 200_000,
        "the document stays bounded for 2000 ticks: {} bytes",
        document.len()
    );
}

#[test]
fn a_large_sample_still_places_every_tick_inside_the_value_range() {
    let values: Vec<f64> = (0..1_000).map(f64::from).collect();
    let source = SVTestRugSource::dense(&values);
    let scene = sv_build_default(&source)
        .compose()
        .expect("the large rug composes");
    let plot = scene.plot_area();
    let segments = sv_tick_segments(&scene);

    assert_eq!(segments.len(), values.len());
    assert!(segments.iter().all(|segment| {
        segment.0.x() >= plot.x
            && segment.0.x() <= plot.right()
            && (segment.0.y() - segment.1.y()).abs() > 0.0
    }));
}
