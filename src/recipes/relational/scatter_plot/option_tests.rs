//! One executable claim per public option that changes the output.

use super::test_support::{
    HORIZONTAL, sv_build_default, sv_dense_source, sv_full_source, sv_scene_text, sv_scene_to_svg,
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

    let scene = plot.compose().expect("the scatter renders");
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
        .compose()
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
    let scene = plot.compose().expect("the scatter renders");
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
    let scene = plot.compose().expect("the scatter renders");
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
    let scene = plot.compose().expect("the scatter renders");

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
        .compose()
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
        .compose()
        .expect("the scatter renders");

    let paint = scene.mark_geometries()[0].primitives()[0].paint();
    assert_eq!(
        paint.stroke(),
        Some(crate::sv_scale::SVColour::new(0, 0, 0, 255))
    );
    assert!((paint.stroke_width() - 1.5).abs() < 1e-9);
}

fn sv_legend_scene(
    placement: SVScatterLegendPlacement,
    source: &dyn crate::sv_data::SVDataSource,
) -> super::SVScatterScene {
    SVScatterPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .series("series")
        .series_labels(&["alpha", "beta", "gamma"])
        .legend(placement)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the scatter composes")
}

#[test]
fn the_legend_carries_every_series_label_in_placement_order() {
    let source = sv_full_source();
    let scene = sv_legend_scene(SVScatterLegendPlacement::Inside, &source);

    assert_eq!(scene.number_of_series(), 3);
    assert_eq!(scene.legend_labels(), &["alpha", "beta", "gamma"]);
    let text = sv_scene_text(&scene);
    for label in ["alpha", "beta", "gamma"] {
        assert!(text.contains(label), "the legend draws the `{label}` entry");
    }
}

#[test]
fn the_legend_placement_changes_where_the_entries_are_drawn() {
    let source = sv_full_source();
    let inside = sv_legend_scene(SVScatterLegendPlacement::Inside, &source);
    let right = sv_legend_scene(SVScatterLegendPlacement::Right, &source);

    assert!(matches!(
        inside.legend_placement(),
        SVScatterLegendPlacement::Inside
    ));
    assert!(matches!(
        right.legend_placement(),
        SVScatterLegendPlacement::Right
    ));
    assert_ne!(
        sv_scene_to_svg(&inside),
        sv_scene_to_svg(&right),
        "a different placement draws the legend at a different position"
    );
}

#[test]
fn a_scatter_without_a_series_draws_no_legend_entries() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the scatter composes");

    assert_eq!(scene.number_of_series(), 1);
    assert!(
        scene.legend_labels().is_empty(),
        "no series channel means no legend entries"
    );
}

#[test]
fn the_axis_labels_default_to_the_column_names() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the scatter composes");
    let text = sv_scene_text(&scene);
    let words: Vec<&str> = text.split_whitespace().collect();

    assert!(
        words.contains(&"x"),
        "the horizontal axis falls back to `x`"
    );
    assert!(words.contains(&"y"), "the vertical axis falls back to `y`");
}

#[test]
fn the_colour_column_wins_over_the_series_palette_within_one_group() {
    let source = sv_full_source();
    let scene = SVScatterPlot::builder()
        .data_source(&source)
        .x("x")
        .y("y")
        .series("series")
        .colour("colour")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the scatter composes");

    let colours: Vec<crate::sv_scale::SVColour> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| primitive.paint().fill())
        .collect();

    // Points 0 and 1 share a series value but carry different `colour` values,
    // so the continuous override — not the series palette — decides the paint.
    assert_ne!(colours[0], colours[1]);
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
        .compose()
        .expect("the scatter renders");

    assert_eq!(scene.title(), Some("Measurements"));
    let text = sv_scene_text(&scene);
    assert!(text.contains("Measurements"));
    assert!(text.contains("width"));
    assert!(text.contains("length"));
}
