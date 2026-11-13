//! One executable claim per public option that changes the output.

use super::SVRugPlot;
use super::test_support::{
    VALUES, sv_build_default, sv_dense_source, sv_scene_text, sv_scene_to_svg, sv_series_source,
    sv_tick_segments,
};
use crate::sv_backend::SVBackendKind;
use crate::sv_layout::SVLength;
use crate::sv_mark::SVBarOrientation;

#[test]
fn horizontal_orientation_places_the_ticks_in_the_left_margin() {
    let source = sv_dense_source();
    let scene = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .orientation(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");
    let plot = scene.plot_area();
    let expected_length = 0.05 * plot.width;

    assert_eq!(scene.orientation(), SVBarOrientation::Horizontal);
    let segments = sv_tick_segments(&scene);
    assert_eq!(segments.len(), VALUES.len());
    for (start, end) in segments {
        assert!((start.y() - end.y()).abs() < 1e-9);
        assert!((start.x() - 6.0).abs() < 1e-9);
        assert!((end.x() - start.x() - expected_length).abs() < 1e-6);
        assert!(
            end.x() < plot.x,
            "the tick stays left of the plot's data region"
        );
    }
}

#[test]
fn the_height_option_changes_the_tick_length() {
    let source = sv_dense_source();
    let short = sv_build_default(&source)
        .compose()
        .expect("the default rug composes");
    let long = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .height(0.2)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the tall rug composes");

    let short_segment = sv_tick_segments(&short)[0];
    let long_segment = sv_tick_segments(&long)[0];
    let short_length = short_segment.0.y() - short_segment.1.y();
    let long_length = long_segment.0.y() - long_segment.1.y();
    assert!((long_length - 4.0 * short_length).abs() < 1e-6);
}

#[test]
fn an_invalid_height_fails_at_build_time() {
    let source = sv_dense_source();
    for height in [0.0, -0.1, f32::NAN, 1.5] {
        let error = SVRugPlot::builder()
            .data_source(&source)
            .value("value")
            .height(height)
            .backend(SVBackendKind::Svg)
            .build()
            .expect_err("an impossible height is rejected");
        assert!(matches!(
            error,
            crate::sv_error::SVError::InvalidConfiguration { .. }
        ));
    }
}

#[test]
fn the_alpha_option_is_applied_to_every_tick() {
    let source = sv_dense_source();
    let opaque = sv_build_default(&source)
        .compose()
        .expect("the default rug composes");
    let translucent = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .alpha(0.4)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");

    let first =
        |scene: &super::SVRugScene| scene.mark_geometries()[0].primitives()[0].paint().opacity();
    assert!((first(&opaque) - 1.0).abs() < 1e-6);
    assert!((first(&translucent) - 0.4).abs() < 1e-6);
}

#[test]
fn an_invalid_alpha_fails_at_build_time() {
    let source = sv_dense_source();
    let error = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .alpha(1.5)
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("opacity outside 0..=1 is rejected");

    assert!(matches!(
        error,
        crate::sv_error::SVError::InvalidConfiguration { .. }
    ));
}

#[test]
fn the_line_width_option_changes_the_stroke_width() {
    let source = sv_dense_source();
    let default = sv_build_default(&source)
        .compose()
        .expect("the default rug composes");
    let wide = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .line_width(SVLength::absolute(3.0))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");

    let width = |scene: &super::SVRugScene| {
        scene.mark_geometries()[0].primitives()[0]
            .paint()
            .stroke_width()
    };
    assert!((width(&wide) - 3.0).abs() < 1e-9);
    assert!(width(&default) > 0.0, "the token supplies a positive width");
}

#[test]
fn the_series_colours_form_stable_groups() {
    let source = sv_series_source();
    let scene = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rug composes");

    let colours: Vec<crate::sv_scale::SVColour> = scene.mark_geometries()[0]
        .primitives()
        .iter()
        .filter_map(|primitive| primitive.paint().stroke())
        .collect();
    assert_eq!(colours.len(), VALUES.len());
    assert_eq!(colours[0], colours[1]);
    assert_ne!(colours[0], colours[2]);
    assert_ne!(colours[2], colours[4]);
}

#[test]
fn a_rug_without_a_series_draws_no_legend_entries() {
    let source = sv_dense_source();
    let scene = sv_build_default(&source)
        .compose()
        .expect("the rug composes");

    assert_eq!(scene.number_of_series(), 1);
    assert!(scene.legend_labels().is_empty());
}

#[test]
fn the_orientation_option_changes_the_document() {
    let source = sv_dense_source();
    let vertical = sv_build_default(&source)
        .compose()
        .expect("the vertical rug composes");
    let horizontal = SVRugPlot::builder()
        .data_source(&source)
        .value("value")
        .orientation(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the horizontal rug composes");

    assert_ne!(sv_scene_to_svg(&vertical), sv_scene_to_svg(&horizontal));
    assert!(
        sv_scene_text(&vertical).contains("value"),
        "the value column titles the axis it occupies"
    );
}
