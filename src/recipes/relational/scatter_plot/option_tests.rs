//! One executable claim per public option that changes the output.

use super::test_support::{
    HORIZONTAL, sv_build_default, sv_dense_source, sv_full_source, sv_scene_text,
};
use super::{SVScatterLegendPlacement, SVScatterPlot, SVScatterSizeScale};
use crate::sv_backend::SVBackendKind;
use crate::sv_mark::SVDotShape;

#[test]
fn the_marker_option_changes_the_drawn_shape() {
    let source = sv_dense_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .marker(SVDotShape::Triangle)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let scene = plot.render().expect("the scatter renders");
    let first = &scene.mark_geometries()[0].primitives()[0];
    assert!(matches!(
        first.shape(),
        crate::sv_mark::SVMarkShape::Path(_)
    ));
}

#[test]
fn the_alpha_option_is_applied_to_every_mark() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .render()
        .expect("the default scatter renders");
    let opaque = scene.mark_geometries()[0].primitives()[0].paint().opacity();

    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .alpha(0.4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");
    let translucent = scene.mark_geometries()[0].primitives()[0].paint().opacity();

    assert!((opaque - 1.0).abs() < 1e-6);
    assert!((translucent - 0.4).abs() < 1e-6);
}

#[test]
fn the_size_column_sizes_points_individually() {
    let source = sv_full_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .size("size")
        .size_scale(SVScatterSizeScale::new(4.0, 12.0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");
    let radii: Vec<f64> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive.shape() {
            crate::sv_mark::SVMarkShape::Circle(circle) => Some(circle.radius()),
            _ => None,
        })
        .collect();

    assert_eq!(radii.len(), HORIZONTAL.len());
    assert!(radii[0] < radii[radii.len() - 1]);
}

#[test]
fn the_series_column_assigns_one_stable_colour_and_legend_entry_per_value() {
    let source = sv_full_source();
    let plot = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .series_labels(&["alpha", "beta", "gamma"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let scene = plot.render().expect("the scatter renders");

    assert_eq!(scene.number_of_series(), 3);
    let first = scene.mark_geometries()[0].primitives();
    let colours: Vec<crate::sv_scale::SVColour> = first
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();
    assert_eq!(colours[0], colours[1]);
    assert_ne!(colours[0], colours[2]);
    assert_ne!(colours[2], colours[4]);
    assert!(scene.legend_labels().contains(&"alpha".to_string()));
    assert!(scene.legend_labels().contains(&"gamma".to_string()));
}

#[test]
fn the_colour_column_overrides_the_series_colour_per_point() {
    let source = sv_full_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .colour("colour")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    let colours: Vec<crate::sv_scale::SVColour> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();
    assert_ne!(colours[0], colours[4]);
}

#[test]
fn the_edge_options_paint_a_stroke() {
    let source = sv_dense_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .edge_colour(crate::sv_scale::SVColour::new(0, 0, 0, 255))
        .edge_width(crate::sv_layout::SVLength::absolute(1.5))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    let paint = scene.mark_geometries()[0].primitives()[0].paint();
    assert_eq!(
        paint.stroke(),
        Some(crate::sv_scale::SVColour::new(0, 0, 0, 255))
    );
    assert!((paint.stroke_width() - 1.5).abs() < 1e-9);
}

#[test]
fn the_legend_placement_option_is_carried_into_the_scene() {
    let source = sv_full_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .legend(SVScatterLegendPlacement::Inside)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    assert!(matches!(
        scene.legend_placement(),
        SVScatterLegendPlacement::Inside
    ));
}

#[test]
fn the_title_and_axis_labels_become_decorations() {
    let source = sv_dense_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .title("Measurements")
        .x_label("width")
        .y_label("length")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect("the scatter renders");

    assert_eq!(scene.title(), Some("Measurements"));
    let text = sv_scene_text(&scene);
    assert!(text.contains("Measurements"));
    assert!(text.contains("width"));
    assert!(text.contains("length"));
}
