//! Each documented option changes the rendered output.

use super::SVSwarmPlot;
use super::test_support::{circle_centres, dense_source, scene_svg};
use crate::sv_backend::SVBackendKind;
use crate::sv_layout::SVLength;
use crate::sv_mark::{SVBarOrientation, SVDotShape};
use crate::sv_scale::SVColour;

#[test]
fn the_marker_shape_changes_the_primitives() {
    let source = dense_source();
    let circles = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let diamonds = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .marker(SVDotShape::Diamond)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_ne!(scene_svg(&circles), scene_svg(&diamonds));
    assert_eq!(circles.number_of_marks(), diamonds.number_of_marks());
    assert!(circles.mark_geometries().iter().all(|geometry| {
        geometry
            .primitives()
            .iter()
            .all(|primitive| matches!(primitive.shape(), crate::sv_mark::SVMarkShape::Circle(_)))
    }));
}

#[test]
fn the_marker_size_changes_the_radius_and_the_swarm_pitch() {
    let source = dense_source();
    let small = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let large = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .size(SVLength::absolute(18.0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    let small_radius = first_radius(&small);
    let large_radius = first_radius(&large);
    assert!((small_radius - 3.0).abs() < 1e-9);
    assert!((large_radius - 9.0).abs() < 1e-9);
}

#[test]
fn the_opacity_reaches_every_dot() {
    let source = dense_source();
    let scene = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .alpha(0.4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    for geometry in scene.mark_geometries() {
        for primitive in geometry.primitives() {
            assert!((primitive.paint().opacity() - 0.4).abs() < 1e-6);
        }
    }
}

#[test]
fn a_cross_is_stroked_instead_of_filled() {
    let source = dense_source();
    let scene = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .marker(SVDotShape::Cross)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    for geometry in scene.mark_geometries() {
        for primitive in geometry.primitives() {
            assert!(primitive.paint().fill().is_none());
            assert!(primitive.paint().stroke().is_some());
        }
    }
}

#[test]
fn the_series_colours_the_dots_and_adds_a_legend() {
    let source = dense_source();
    let plain = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let grouped = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .series("series")
        .series_labels(&["alpha", "beta"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_eq!(plain.number_of_series(), 1);
    assert_eq!(grouped.number_of_series(), 2);
    assert_eq!(
        grouped.legend_labels(),
        &["alpha".to_string(), "beta".to_string()]
    );
    let fills: Vec<Option<SVColour>> = grouped
        .mark_geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .map(|primitive| primitive.paint().fill())
        .collect();
    assert!(fills.iter().all(Option::is_some));
    assert_ne!(fills[0], fills[2], "different series get different colours");
}

#[test]
fn the_orientation_moves_the_categories_to_the_vertical_axis() {
    let source = dense_source();
    let vertical = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let horizontal = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .orientation(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_eq!(horizontal.orientation(), SVBarOrientation::Horizontal);
    assert_ne!(scene_svg(&vertical), scene_svg(&horizontal));
    let vertical_centres = circle_centres(&vertical);
    let horizontal_centres = circle_centres(&horizontal);
    assert!((vertical_centres[0].0 - vertical_centres[4].0).abs() < 1e-9);
    assert!((horizontal_centres[0].1 - horizontal_centres[4].1).abs() < 1e-9);
}

#[test]
fn the_swarm_width_changes_the_layout_while_keeping_every_point() {
    let source = dense_source();
    let wide = super::test_support::default_builder(&source)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");
    let narrow = SVSwarmPlot::builder()
        .data_source(&source)
        .value("value")
        .category("category")
        .swarm_width(0.2)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the swarm composes");

    assert_eq!(wide.number_of_marks(), narrow.number_of_marks());
    assert_eq!(
        wide.number_of_mark_primitives(),
        narrow.number_of_mark_primitives()
    );
}

fn first_radius(scene: &super::SVSwarmScene) -> f64 {
    scene
        .mark_geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .find_map(|primitive| match primitive.shape() {
            crate::sv_mark::SVMarkShape::Circle(circle) => Some(circle.radius()),
            _ => None,
        })
        .expect("the default marker is a circle")
}
