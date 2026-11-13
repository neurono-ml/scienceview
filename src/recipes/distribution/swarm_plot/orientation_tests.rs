//! Orientation: which axis carries the categories, and therefore the swarm.

use super::SVSwarmPlot;
use super::test_support::{circle_centres, dense_source, scene_svg};
use crate::sv_backend::SVBackendKind;
use crate::sv_mark::SVBarOrientation;

fn compose(orientation: SVBarOrientation) -> super::SVSwarmScene {
    let source = dense_source();
    SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .orientation(orientation)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes")
}

#[test]
fn vertical_puts_the_categories_on_the_horizontal_axis() {
    let scene = compose(SVBarOrientation::Vertical);
    let centres = circle_centres(&scene);
    // Two categories, so two distinct x values and one x per category.
    let first = centres[0].0;
    assert!(
        centres[..5]
            .iter()
            .all(|centre| (centre.0 - first).abs() < 1e-9)
    );
    let second = centres[5].0;
    assert!(
        centres[5..]
            .iter()
            .all(|centre| (centre.0 - second).abs() < 1e-9)
    );
    assert!((second - first).abs() > 1.0);
    assert!(
        centres[..5]
            .iter()
            .all(|centre| (centre.0 - second).abs() > 1.0)
    );
}

#[test]
fn horizontal_puts_the_categories_on_the_vertical_axis() {
    let scene = compose(SVBarOrientation::Horizontal);
    let centres = circle_centres(&scene);
    let first = centres[0].1;
    assert!(
        centres[..5]
            .iter()
            .all(|centre| (centre.1 - first).abs() < 1e-9)
    );
    let second = centres[5].1;
    assert!(
        centres[5..]
            .iter()
            .all(|centre| (centre.1 - second).abs() < 1e-9)
    );
    assert!((second - first).abs() > 1.0);
    assert!(
        centres[..5]
            .iter()
            .all(|centre| (centre.1 - second).abs() > 1.0)
    );
}

#[test]
fn the_two_orientations_differ_and_report_their_axes() {
    let vertical = compose(SVBarOrientation::Vertical);
    let horizontal = compose(SVBarOrientation::Horizontal);

    assert_eq!(vertical.orientation(), SVBarOrientation::Vertical);
    assert_eq!(horizontal.orientation(), SVBarOrientation::Horizontal);
    assert_ne!(scene_svg(&vertical), scene_svg(&horizontal));
}
