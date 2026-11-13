//! Unit tests for `SVBoxPlot` and its public options (FDR-116).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape, SVMarkShape, SVPathCommand};

use super::{SVBoxPlotBuilder, SVWhiskerRule};

const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

fn table<'values>(values: &'values [f64]) -> SVDataTable<'values> {
    let category = vec![0.0; values.len()];
    // The caller owns both slices for the duration of the test.
    let category: &'values [f64] = Box::leak(category.into_boxed_slice());
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, values)
        .with_column("category", SVColumnRole::Grouping, category)
        .build()
        .expect("the columns share one length")
}

fn plot<'values>(table: &'values SVDataTable<'values>) -> SVBoxPlotBuilder<'values> {
    SVBoxPlotBuilder::new()
        .data_source(table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

#[test]
fn default_summary_matches_the_percentile_estimator() {
    let data = table(&VALUES);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output
        .summaries()
        .first()
        .expect("one category is summarised");

    assert!((summary.lower_quartile() - 3.25).abs() < 1e-12);
    assert!((summary.median() - 5.5).abs() < 1e-12);
    assert!((summary.upper_quartile() - 7.75).abs() < 1e-12);
    assert!((summary.lower_whisker() - 1.0).abs() < 1e-12);
    assert!((summary.upper_whisker() - 10.0).abs() < 1e-12);
    assert!(summary.outliers().is_empty());
    assert_eq!(output.number_of_points(), VALUES.len());
}

#[test]
fn default_draws_a_filled_box_and_stroked_segments() {
    let data = table(&VALUES);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let primitives: Vec<_> = output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .collect();
    assert!(
        primitives
            .iter()
            .any(|primitive| primitive.paint().fill().is_some()),
        "the box body is filled"
    );
    assert!(
        primitives
            .iter()
            .any(|primitive| primitive.paint().stroke().is_some()),
        "the median and whiskers are stroked"
    );
}

#[test]
fn tukey_rule_marks_points_outside_the_fences_as_outliers() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0];
    let data = table(&values);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert!((summary.upper_whisker() - 9.0).abs() < 1e-12);
    assert_eq!(summary.outliers(), &[100.0]);
    assert_eq!(output.number_of_outliers(), 1);
}

#[test]
fn min_max_rule_draws_the_extremes_and_no_outliers() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0];
    let data = table(&values);
    let output = plot(&data)
        .whisker(SVWhiskerRule::MinMax)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert!((summary.lower_whisker() - 1.0).abs() < 1e-12);
    assert!((summary.upper_whisker() - 100.0).abs() < 1e-12);
    assert!(summary.outliers().is_empty());
}

#[test]
fn percentile_rule_uses_the_requested_percentiles() {
    let data = table(&VALUES);
    let output = plot(&data)
        .whisker(SVWhiskerRule::Percentile(0.1))
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert!((summary.lower_whisker() - 1.9).abs() < 1e-12);
    assert!((summary.upper_whisker() - 9.1).abs() < 1e-12);
    assert_eq!(summary.outliers(), &[1.0, 10.0]);
}

#[test]
fn show_outliers_can_be_turned_off() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0];
    let data = table(&values);
    let output = plot(&data)
        .show_outliers(false)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(output.number_of_outliers(), 0);
    assert_eq!(
        output.summaries().first().expect("one summary").outliers(),
        &[100.0],
        "hiding outliers does not change the summary"
    );
}

#[test]
fn quartiles_option_moves_the_box_edges() {
    let data = table(&VALUES);
    let output = plot(&data)
        .quartiles(vec![0.1, 0.5, 0.9])
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert!((summary.lower_quartile() - 1.9).abs() < 1e-12);
    assert!((summary.upper_quartile() - 9.1).abs() < 1e-12);
}

#[test]
fn notch_option_draws_three_stacked_segments() {
    let data = table(&VALUES);
    let plain = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let notched = plot(&data)
        .notch(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = notched.summaries().first().expect("one summary");

    assert!(summary.notch_half_width() > 0.0);
    assert!(
        summary.notch_half_width() < (summary.upper_quartile() - summary.lower_quartile()) / 2.0
    );
    assert_eq!(filled_box_widths(&plain).len(), 1);
    let widths = filled_box_widths(&notched);
    assert_eq!(widths.len(), 3, "the notch stacks three ribbon segments");
    assert!(
        widths[2] < widths[0] * 0.75 && widths[2] < widths[1] * 0.75,
        "the middle segment around the median is the narrowed one: {widths:?}"
    );
}

#[test]
fn show_mean_adds_a_marker() {
    let data = table(&VALUES);
    let without = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let with = plot(&data)
        .show_mean(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert!(with.geometries().len() > without.geometries().len());
    assert!((with.summaries()[0].mean() - 5.5).abs() < 1e-12);
}

#[test]
fn series_draws_one_box_per_group() {
    let values = [1.0, 2.0, 3.0, 4.0, 8.0, 9.0, 10.0, 11.0];
    let category = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let series = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &category)
        .with_column("series", SVColumnRole::Grouping, &series)
        .build()
        .expect("the columns share one length");

    let output = plot(&data)
        .series("series")
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(output.summaries().len(), 2);
    assert_eq!(output.series().len(), 2);
    let offsets: Vec<f64> = output
        .summaries()
        .iter()
        .map(|summary| summary.category_position())
        .collect();
    assert!(
        (offsets[0] - offsets[1]).abs() > 1e-9,
        "dodge separates the two boxes"
    );
}

#[test]
fn a_constant_distribution_renders_a_flat_box_without_a_notch() {
    let values = [5.0; 6];
    let data = table(&values);
    let output = plot(&data)
        .notch(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert_eq!(summary.notch_half_width(), 0.0);
    assert_eq!(
        filled_box_boxes(&output).len(),
        1,
        "a zero IQR draws no notch"
    );
    assert!(summary.lower_quartile() == summary.upper_quartile());
}

#[test]
fn a_single_category_box_keeps_its_slot_width() {
    let data = table(&VALUES);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let (width, height) = filled_box_extent(&output);
    assert!(
        (width - 0.8 * output.plot_area().width).abs() < 1e-3,
        "one category still owns a full slot: {width}"
    );
    assert!(height > 0.0);
}

#[test]
fn horizontal_orientation_places_the_summary_along_x() {
    let data = table(&VALUES);
    let vertical = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let horizontal = plot(&data)
        .orient(SVBarOrientation::Horizontal)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let (vertical_width, vertical_height) = filled_box_extent(&vertical);
    let (horizontal_width, horizontal_height) = filled_box_extent(&horizontal);
    let plot = vertical.plot_area();
    // The category slot is one unit wide, so the box occupies `width` of it.
    // The quartile span (`q3 - q1` = 0.5 of the value domain) runs along the
    // value axis: y for vertical, x for horizontal.
    assert!((vertical_width - 0.8 * plot.width).abs() < 1e-3);
    assert!((vertical_height - 0.5 * plot.height).abs() < 1e-3);
    assert!((horizontal_width - 0.5 * plot.width).abs() < 1e-3);
    assert!((horizontal_height - 0.8 * plot.height).abs() < 1e-3);
}

#[test]
fn a_missing_value_is_skipped_without_failing_the_render() {
    let values = [1.0, f64::NAN, 3.0, 4.0, 5.0];
    let data = table(&values);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(output.number_of_points(), 4);
    assert_eq!(output.summaries().len(), 1);
}

#[test]
fn an_all_missing_input_fails_with_empty_input() {
    let values = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    let data = table(&values);
    let error = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect_err("no finite observation remains");
    assert!(matches!(error, SVError::EmptyInput { .. }));
}

#[test]
fn missing_columns_are_rejected_at_build_time() {
    let data = table(&VALUES);
    let error = SVBoxPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a category is required");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));

    let error = SVBoxPlotBuilder::new()
        .data_source(&data)
        .value("absent")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the column must exist");
    assert!(matches!(error, SVError::ColumnNotFound { .. }));
}

#[test]
fn invalid_options_are_rejected_at_build_time() {
    let data = table(&VALUES);
    for request in [
        plot(&data).quartiles(vec![0.5]).boxed(),
        plot(&data).quartiles(vec![0.7, 0.6, 0.5]).boxed(),
        plot(&data).whisker(SVWhiskerRule::Percentile(0.7)).boxed(),
        plot(&data).width(1.5).boxed(),
    ] {
        assert!(
            matches!(request, Err(SVError::InvalidConfiguration { .. })),
            "an impossible option fails at build"
        );
    }
}

#[test]
fn outlier_marker_changes_the_marker_shape() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0];
    let data = table(&values);
    let output = plot(&data)
        .outlier_marker(SVDotShape::Square)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert!(
        output
            .geometries()
            .iter()
            .flat_map(|geometry| geometry.primitives())
            .any(|primitive| matches!(primitive.shape(), SVMarkShape::Rectangle(_))),
        "the square outlier is a rectangle primitive"
    );
}

#[test]
fn default_backend_kind_is_carried_and_reported() {
    let data = table(&VALUES);
    let built = plot(&data).build().expect("completes");
    assert_eq!(built.backend_kind(), SVBackendKind::Svg);
    assert_eq!(
        built.execution_mode(),
        scienceview_core::sv_figure::SVExecutionMode::Automatic
    );
}

/// Small helper so the invalid-option cases share one closure type.
trait SVBoxBuildResult {
    fn boxed(self) -> Result<(), SVError>;
}

impl SVBoxBuildResult for SVBoxPlotBuilder<'_> {
    fn boxed(self) -> Result<(), SVError> {
        self.build().map(|_| ())
    }
}

/// The bounding boxes of the filled paths, in draw order.
fn filled_box_boxes(output: &super::SVBoxRenderOutput) -> Vec<(f64, f64, f64, f64)> {
    output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .filter(|primitive| primitive.paint().fill().is_some())
        .filter_map(|primitive| match primitive.shape() {
            SVMarkShape::Path(path) => Some(path.commands()),
            _ => None,
        })
        .map(|commands| {
            let mut minimum_x = f64::INFINITY;
            let mut minimum_y = f64::INFINITY;
            let mut maximum_x = f64::NEG_INFINITY;
            let mut maximum_y = f64::NEG_INFINITY;
            for command in commands {
                if let SVPathCommand::MoveTo(point) | SVPathCommand::LineTo(point) = command {
                    minimum_x = minimum_x.min(point.x());
                    minimum_y = minimum_y.min(point.y());
                    maximum_x = maximum_x.max(point.x());
                    maximum_y = maximum_y.max(point.y());
                }
            }
            (minimum_x, minimum_y, maximum_x, maximum_y)
        })
        .collect()
}

/// The width and height of the first filled box body.
fn filled_box_extent(output: &super::SVBoxRenderOutput) -> (f64, f64) {
    let (minimum_x, minimum_y, maximum_x, maximum_y) = filled_box_boxes(output)
        .first()
        .copied()
        .expect("the box body is a filled path");
    (maximum_x - minimum_x, maximum_y - minimum_y)
}

/// The horizontal extent of every filled box segment.
fn filled_box_widths(output: &super::SVBoxRenderOutput) -> Vec<f64> {
    filled_box_boxes(output)
        .into_iter()
        .map(|(minimum_x, _, maximum_x, _)| maximum_x - minimum_x)
        .collect()
}
