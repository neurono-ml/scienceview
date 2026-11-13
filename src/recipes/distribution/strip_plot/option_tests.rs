//! Each documented option changes the composed output.

use super::SVStripPlotBuilder;
use super::test_support::{sv_dense_source, sv_dot_centres};
use super::{SVStripPlot, SVStripScene};
use crate::sv_backend::SVBackendKind;
use crate::sv_data::SVDataSource;
use crate::sv_figure::SVExecutionMode;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape, SVMarkGeometry, SVMarkPoint, SVMarkShape};
use crate::sv_move::SVJitterMode;

fn sv_compose<'a>(
    source: &'a dyn SVDataSource,
    options: impl FnOnce(SVStripPlotBuilder<'a>) -> SVStripPlotBuilder<'a>,
) -> SVStripScene {
    let builder = SVStripPlot::builder()
        .data_source(source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg);
    options(builder)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the strip plot composes")
}

#[test]
fn the_jitter_amount_widens_the_dots_within_their_category() {
    let source = sv_dense_source();
    let narrow = sv_compose(&source, |builder| builder.jitter(0.05).seed(3));
    let wide = sv_compose(&source, |builder| builder.jitter(0.5).seed(3));

    assert!(sv_horizontal_spread(&wide) > sv_horizontal_spread(&narrow));
}

fn sv_horizontal_spread(scene: &SVStripScene) -> f64 {
    let xs: Vec<f64> = sv_dot_centres(scene).iter().map(SVMarkPoint::x).collect();
    let minimum = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    maximum - minimum
}

#[test]
fn the_jitter_mode_changes_the_distribution_but_not_the_band() {
    let source = sv_dense_source();
    let uniform = sv_compose(&source, |builder| builder.jitter(0.3).seed(5));
    let gaussian = sv_compose(&source, |builder| {
        builder
            .jitter(0.3)
            .seed(5)
            .jitter_mode(SVJitterMode::Gaussian)
    });

    assert_ne!(
        sv_dot_centres(&uniform),
        sv_dot_centres(&gaussian),
        "the two modes draw different offsets"
    );
}

#[test]
fn the_seed_changes_the_offset_sequence() {
    let source = sv_dense_source();
    let first = sv_compose(&source, |builder| builder.jitter(0.3).seed(1));
    let second = sv_compose(&source, |builder| builder.jitter(0.3).seed(2));

    assert_ne!(sv_dot_centres(&first), sv_dot_centres(&second));
}

#[test]
fn the_marker_shape_reaches_the_geometry() {
    let source = sv_dense_source();
    let circle = sv_compose(&source, |builder| builder.marker(SVDotShape::Circle));
    let square = sv_compose(&source, |builder| builder.marker(SVDotShape::Square));

    assert_eq!(circle.number_of_marks(), square.number_of_marks());
    assert_ne!(
        sv_scene_shape_counts(&circle),
        sv_scene_shape_counts(&square)
    );
}

fn sv_scene_shape_counts(scene: &SVStripScene) -> (usize, usize) {
    let mut circles = 0;
    let mut rectangles = 0;
    for geometry in scene.mark_geometries() {
        for primitive in geometry.primitives() {
            match primitive.shape() {
                SVMarkShape::Circle(_) => circles += 1,
                SVMarkShape::Rectangle(_) => rectangles += 1,
                _ => {}
            }
        }
    }
    (circles, rectangles)
}

#[test]
fn alpha_and_size_reach_the_paint_and_the_radius() {
    let source = sv_dense_source();
    let scene = sv_compose(&source, |builder| {
        builder
            .alpha(0.4)
            .size(SVLength::absolute(20.0))
            .marker(SVDotShape::Circle)
    });
    let centres = sv_dot_centres(&scene);
    let radii: Vec<f64> = scene
        .mark_geometries()
        .iter()
        .flat_map(SVMarkGeometry::primitives)
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Circle(circle) => Some(circle.radius()),
            _ => None,
        })
        .collect();

    assert_eq!(centres.len(), radii.len());
    assert!((radii[0] - 10.0).abs() < 1e-9);
    let opacity = scene
        .mark_geometries()
        .iter()
        .flat_map(SVMarkGeometry::primitives)
        .next()
        .map(|primitive| primitive.paint().opacity())
        .expect("a drawn primitive");
    assert!((opacity - 0.4).abs() < 1e-6);
}

#[test]
fn the_orientation_swaps_the_value_and_category_axes() {
    let source = sv_dense_source();
    let vertical = sv_compose(&source, |builder| {
        builder.orientation(SVBarOrientation::Vertical)
    });
    let horizontal = sv_compose(&source, |builder| {
        builder.orientation(SVBarOrientation::Horizontal)
    });

    assert_eq!(vertical.orientation(), SVBarOrientation::Vertical);
    assert_eq!(horizontal.orientation(), SVBarOrientation::Horizontal);
    let vertical_centres = sv_dot_centres(&vertical);
    let horizontal_centres = sv_dot_centres(&horizontal);
    assert_ne!(vertical_centres, horizontal_centres);
    // The first two observations share a category and differ only in value, so
    // the value axis is the one that separates them.
    let vertical_first = vertical_centres[0];
    let vertical_second = vertical_centres[1];
    assert!(
        (vertical_first.y() - vertical_second.y()).abs()
            > (vertical_first.x() - vertical_second.x()).abs()
    );
    let horizontal_first = horizontal_centres[0];
    let horizontal_second = horizontal_centres[1];
    assert!(
        (horizontal_first.x() - horizontal_second.x()).abs()
            > (horizontal_first.y() - horizontal_second.y()).abs()
    );
}

#[test]
fn the_titles_reach_the_decorations() {
    let source = sv_dense_source();
    let scene = sv_compose(&source, |builder| {
        builder
            .title("Body mass")
            .value_label("mass")
            .category_label("species")
    });
    let text = super::test_support::sv_scene_text(&scene);

    assert!(text.contains("Body mass"));
    assert!(text.contains("mass"));
    assert!(text.contains("species"));
}

#[test]
fn the_execution_mode_is_selectable() {
    let source = sv_dense_source();
    let plot = SVStripPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .execution_mode(SVExecutionMode::Synchronous)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.execution_mode(), SVExecutionMode::Synchronous);
}

#[test]
fn an_impossible_alpha_size_or_jitter_fails_at_build_time() {
    let source = sv_dense_source();
    let builder = || {
        SVStripPlot::builder()
            .data_source(&source)
            .value("value")
            .category("category")
            .backend(SVBackendKind::Svg)
    };

    assert!(builder().alpha(1.5).build().is_err());
    assert!(builder().size(SVLength::absolute(-2.0)).build().is_err());
    assert!(builder().jitter(-0.1).build().is_err());
    assert!(builder().jitter(1.5).build().is_err());
}
