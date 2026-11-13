//! The documented defaults and each option of the heatmap.

use super::test_support::{sv_build_default, sv_dense_source, sv_scene_text};
use super::{SVHeatmapMatrix, SVHeatmapOrientation, SVHeatmapPlot, SVMissingValue};
use crate::sv_backend::SVBackendKind;
use crate::sv_error::SVError;
use crate::sv_mark::SVFormatSpec;
use crate::sv_scale::{SVColour, SVColourMap};

fn sv_uniform_colour_map() -> SVColourMap {
    SVColourMap::from_stops(
        "greys",
        vec![
            SVColour::new(0, 0, 0, 255),
            SVColour::new(255, 255, 255, 255),
        ],
    )
    .expect("two stops form a map")
}

#[test]
fn the_default_configuration_builds_and_draws_every_cell() {
    let source = sv_dense_source();
    let plot = sv_build_default(&source);
    let scene = plot.compose().expect("the default heatmap composes");

    assert_eq!(scene.number_of_marks(), 9);
    assert_eq!(scene.cells().len(), 9);
    assert_eq!(scene.row_labels(), ["0", "1", "2"]);
    assert_eq!(scene.column_labels(), ["0", "1", "2"]);
    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert!(plot.has_colour_bar());
    assert_eq!(plot.missing_value(), SVMissingValue::Transparent);
    assert_eq!(plot.orientation(), SVHeatmapOrientation::RowsAlongVertical);
}

#[test]
fn every_cell_takes_the_colour_map_colour_of_its_value() {
    let source = sv_dense_source();
    let scene = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .colour_map(sv_uniform_colour_map())
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the heatmap composes");

    for cell in scene.cells() {
        let value = cell.value().expect("a finite cell");
        assert_eq!(
            cell.colour(),
            Some(
                scene
                    .colour_scale()
                    .colour(value)
                    .expect("the value has a colour")
            ),
            "cell ({}, {}) matches the shared scale",
            cell.row(),
            cell.column()
        );
    }
    assert_eq!(scene.colour_scale().domain().lower(), 1.0);
    assert_eq!(scene.colour_scale().domain().upper(), 9.0);
}

#[test]
fn annotations_use_the_requested_format() {
    let source = sv_dense_source();
    let scene = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .annot(true)
        .annot_format(SVFormatSpec::Fixed(1))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the annotated heatmap composes");

    let text = sv_scene_text(&scene);
    assert!(text.contains("1.0"), "the annotation is formatted: {text}");
    assert!(text.contains("9.0"));
}

#[test]
fn the_colour_bar_can_be_switched_off() {
    let source = sv_dense_source();
    let with_bar = sv_build_default(&source)
        .compose()
        .expect("the default composes");
    let without_source = sv_dense_source();
    let without_bar = SVHeatmapPlot::builder()
        .data_source(&without_source)
        .row("row")
        .column("column")
        .value("value")
        .colour_bar(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the heatmap composes");

    assert!(
        without_bar.decoration_primitives_after_marks().len()
            < with_bar.decoration_primitives_after_marks().len()
    );
}

#[test]
fn a_centre_symmetrises_the_colour_domain() {
    let source = sv_dense_source();
    let scene = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("value")
        .centre(0.0)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the heatmap composes");

    assert_eq!(scene.colour_scale().domain().lower(), -9.0);
    assert_eq!(scene.colour_scale().domain().upper(), 9.0);
}

#[test]
fn a_direct_matrix_uses_the_supplied_labels() {
    let values = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
    let matrix = SVHeatmapMatrix::new(&values, 2, 3).expect("the shape matches");
    let scene = SVHeatmapPlot::builder()
        .matrix(matrix)
        .row_labels(&["north", "south"])
        .column_labels(&["one", "two", "three"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the direct matrix composes");

    assert_eq!(scene.row_labels(), ["north", "south"]);
    assert_eq!(scene.column_labels(), ["one", "two", "three"]);
    assert_eq!(scene.number_of_marks(), 6);
}

#[test]
fn square_cells_are_as_tall_as_wide() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let matrix = SVHeatmapMatrix::new(&values, 2, 3).expect("the shape matches");
    let scene = SVHeatmapPlot::builder()
        .matrix(matrix)
        .square(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the square heatmap composes");

    for primitive in scene.mark_geometries()[0].primitives() {
        if let crate::sv_mark::SVMarkShape::Rectangle(rectangle) = primitive.shape() {
            assert_eq!(rectangle.rectangle().width, rectangle.rectangle().height);
        }
    }
}

#[test]
fn the_orientation_swaps_the_two_axes() {
    let source = sv_dense_source();
    let vertical = sv_build_default(&source)
        .compose()
        .expect("the default composes");
    let horizontal_source = sv_dense_source();
    let horizontal = SVHeatmapPlot::builder()
        .data_source(&horizontal_source)
        .row("row")
        .column("column")
        .value("value")
        .orientation(SVHeatmapOrientation::RowsAlongHorizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete")
        .compose()
        .expect("the rotated heatmap composes");

    let second =
        |scene: &super::SVHeatmapScene| scene.mark_geometries()[0].primitives()[1].shape().clone();
    assert_ne!(second(&vertical), second(&horizontal));
}

#[test]
fn a_missing_backend_is_an_explicit_error() {
    let values = [1.0];
    let matrix = SVHeatmapMatrix::new(&values, 1, 1).expect("the shape matches");
    let error = SVHeatmapPlot::builder()
        .matrix(matrix)
        .build()
        .expect_err("the backend is required");
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn an_incomplete_input_is_an_explicit_error() {
    let error = SVHeatmapPlot::builder()
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("an input is required");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_mismatched_matrix_shape_is_rejected() {
    let values = [1.0, 2.0];
    let error = SVHeatmapMatrix::new(&values, 3, 3).expect_err("the shape does not match");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn a_missing_table_column_is_an_explicit_error() {
    let source = sv_dense_source();
    let error = SVHeatmapPlot::builder()
        .data_source(&source)
        .row("row")
        .column("column")
        .value("absent")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column is missing");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "absent".to_string()
        }
    );
}

#[test]
fn a_backend_without_cells_is_rejected() {
    let values = [1.0];
    let matrix = SVHeatmapMatrix::new(&values, 1, 1).expect("the shape matches");
    let error = SVHeatmapPlot::builder()
        .matrix(matrix)
        .backend(SVBackendKind::Tui)
        .build()
        .expect_err("the terminal cannot draw image cells");
    assert!(matches!(error, SVError::UnsupportedMark { .. }));
}

#[test]
fn mismatched_labels_are_rejected() {
    let values = [1.0, 2.0];
    let matrix = SVHeatmapMatrix::new(&values, 1, 2).expect("the shape matches");
    let error = SVHeatmapPlot::builder()
        .matrix(matrix)
        .row_labels(&["a", "b"])
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("one row cannot carry two labels");
    assert!(matches!(error, SVError::MismatchedLengths { .. }));
}

#[test]
fn the_missing_cell_rule_decides_what_is_drawn() {
    let values = [1.0, f64::NAN, 3.0, 4.0];
    let build = |missing| {
        let matrix = SVHeatmapMatrix::new(&values, 2, 2).expect("the shape matches");
        SVHeatmapPlot::builder()
            .matrix(matrix)
            .missing(missing)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("the heatmap composes")
    };

    let omitted = build(SVMissingValue::Omit);
    let transparent = build(SVMissingValue::Transparent);
    let neutral = build(SVMissingValue::Neutral);

    assert_eq!(omitted.number_of_marks(), 3);
    assert_eq!(transparent.number_of_marks(), 4);
    assert_eq!(neutral.number_of_marks(), 4);
    let missing = transparent
        .cells()
        .iter()
        .find(|cell| cell.value().is_none())
        .expect("the missing cell is recorded");
    assert_eq!(missing.colour(), None);
    let neutral_missing = neutral
        .cells()
        .iter()
        .find(|cell| cell.value().is_none())
        .expect("the neutral cell is recorded");
    assert!(neutral_missing.colour().is_some());
}

#[test]
fn a_large_matrix_composes_every_cell_deterministically() {
    let values: Vec<f64> = (0..1_600).map(|index| (index % 37) as f64).collect();
    let matrix = SVHeatmapMatrix::new(&values, 40, 40).expect("the shape matches");
    let plot = SVHeatmapPlot::builder()
        .matrix(matrix)
        .colour_bar(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    let first = plot.compose().expect("the large matrix composes");
    let second = plot.compose().expect("the large matrix composes again");
    assert_eq!(first.number_of_marks(), 1_600);
    assert_eq!(first, second);
}

#[test]
fn robust_derives_the_domain_from_the_central_percentiles() {
    let values = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 100.0];
    let matrix = SVHeatmapMatrix::new(&values, 2, 5).expect("the shape matches");
    let build = |robust| {
        SVHeatmapPlot::builder()
            .matrix(matrix)
            .robust(robust)
            .colour_bar(false)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("the heatmap composes")
    };

    let plain = build(false);
    let robust = build(true);

    assert_eq!(
        plain.colour_scale().domain().upper(),
        100.0,
        "the plain domain reaches the outlier"
    );
    assert!(
        robust.colour_scale().domain().upper() < 100.0,
        "the robust domain stops before the outlier"
    );
}

#[test]
fn a_positive_line_width_strokes_every_cell() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let matrix = SVHeatmapMatrix::new(&values, 2, 2).expect("the shape matches");
    let build = |line_width| {
        SVHeatmapPlot::builder()
            .matrix(matrix)
            .line_width(line_width)
            .colour_bar(false)
            .backend(SVBackendKind::Svg)
            .build()
            .expect("the configuration is complete")
            .compose()
            .expect("the heatmap composes")
    };

    let plain = build(0.0);
    let stroked = build(2.0);

    assert!(
        plain
            .mark_geometries()
            .iter()
            .flat_map(|geometry| geometry.primitives())
            .all(|primitive| primitive.paint().stroke().is_none()),
        "the default cells have no border"
    );
    assert!(
        stroked
            .mark_geometries()
            .iter()
            .flat_map(|geometry| geometry.primitives())
            .all(|primitive| primitive.paint().stroke().is_some()),
        "every cell takes the requested border"
    );
}
