//! Unit tests for `SVBoxenPlot` and its public options (FDR-118).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape, SVMarkShape, SVPathCommand};

use super::{SVBoxenPlotBuilder, SVBoxenTailRule};

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

fn table_of_length(length: usize) -> (Vec<f64>, Vec<f64>) {
    let values: Vec<f64> = (1..=length).map(|index| index as f64).collect();
    let categories = vec![0.0; length];
    (values, categories)
}

fn plot<'values>(table: &'values SVDataTable<'values>) -> SVBoxenPlotBuilder<'values> {
    SVBoxenPlotBuilder::new()
        .data_source(table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

#[test]
fn default_summary_uses_the_percentile_kernel() {
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

    assert_eq!(summary.depth(), 1);
    assert_eq!(summary.boxes().len(), 1);
    assert!((summary.boxes()[0].lower() - 3.25).abs() < 1e-12);
    assert!((summary.boxes()[0].upper() - 7.75).abs() < 1e-12);
    assert!((summary.median() - 5.5).abs() < 1e-12);
    assert_eq!(summary.outliers(), &[1.0, 2.0, 3.0, 8.0, 9.0, 10.0]);
    assert_eq!(output.number_of_points(), VALUES.len());
}

#[test]
fn depth_option_adds_nested_levels() {
    let data = table(&VALUES);
    let output = plot(&data)
        .depth(2)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert_eq!(summary.depth(), 2);
    assert_eq!(summary.boxes().len(), 3);
    let bounds: Vec<f64> = summary
        .boxes()
        .iter()
        .flat_map(|level| [level.lower(), level.upper()])
        .collect();
    assert!((bounds[0] - 2.125).abs() < 1e-12);
    assert!((bounds[1] - 3.25).abs() < 1e-12);
    assert!((bounds[2] - 3.25).abs() < 1e-12);
    assert!((bounds[3] - 7.75).abs() < 1e-12);
    assert!((bounds[4] - 7.75).abs() < 1e-12);
    assert!((bounds[5] - 8.875).abs() < 1e-12);
    assert_eq!(
        summary
            .boxes()
            .iter()
            .map(|level| level.level())
            .collect::<Vec<_>>(),
        vec![0, 1, 0]
    );
    assert_eq!(
        summary
            .boxes()
            .iter()
            .map(|level| level.relative_width())
            .collect::<Vec<_>>(),
        vec![0.5, 1.0, 0.5]
    );
}

#[test]
fn derived_depth_follows_the_henderson_rule() {
    let (values, categories) = table_of_length(64);
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the columns share one length");
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    // floor(log2(64)) - 3 = 3 levels, therefore 2 * 3 - 1 = 5 boxes.
    assert_eq!(summary.depth(), 3);
    assert_eq!(summary.boxes().len(), 5);
}

#[test]
fn the_proportion_rule_changes_the_derived_depth() {
    let (values, categories) = table_of_length(1024);
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .build()
        .expect("the columns share one length");

    let tukey = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let proportion = plot(&data)
        .tail_rule(SVBoxenTailRule::proportion(0.01))
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert_eq!(tukey.summaries()[0].depth(), 7);
    assert_eq!(proportion.summaries()[0].depth(), 8);
    assert!(proportion.summaries()[0].boxes().len() > tukey.summaries()[0].boxes().len());
}

#[test]
fn the_full_rule_extends_to_the_observed_extremes() {
    let data = table(&VALUES);
    let output = plot(&data)
        .tail_rule(SVBoxenTailRule::Full)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert_eq!(summary.depth(), 4);
    assert_eq!(summary.boxes().len(), 7);
    assert!((summary.boxes()[0].lower() - 1.0).abs() < 1e-12);
    assert!((summary.boxes()[6].upper() - 10.0).abs() < 1e-12);
    assert!(summary.outliers().is_empty());
}

#[test]
fn show_median_controls_the_median_line() {
    let data = table(&VALUES);
    let without = plot(&data)
        .show_median(false)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let with = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    assert!(with.geometries().len() > without.geometries().len());
}

#[test]
fn outliers_are_hidden_by_default_and_can_be_shown() {
    let data = table(&VALUES);
    let hidden = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    assert_eq!(hidden.number_of_outliers(), 0);
    assert_eq!(
        hidden.summaries()[0].outliers(),
        &[1.0, 2.0, 3.0, 8.0, 9.0, 10.0],
        "the summary keeps the outliers even when they are not drawn"
    );

    let shown = plot(&data)
        .show_outliers(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    assert_eq!(shown.number_of_outliers(), 6);
    assert!(shown.geometries().len() > hidden.geometries().len());
}

#[test]
fn series_draws_one_nest_per_group() {
    let values = [1.0, 2.0, 3.0, 4.0, 8.0, 9.0, 10.0, 11.0];
    let category = [0.0; 8];
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
        "dodge separates the two nests"
    );
}

#[test]
fn horizontal_orientation_places_the_nest_along_x() {
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
    assert!((vertical_width - 0.8 * plot.width).abs() < 1e-3);
    assert!((vertical_height - 0.5 * plot.height).abs() < 1e-3);
    assert!((horizontal_width - 0.5 * plot.width).abs() < 1e-3);
    assert!((horizontal_height - 0.8 * plot.height).abs() < 1e-3);
}

#[test]
fn a_constant_distribution_renders_a_zero_height_central_box() {
    let values = [5.0; 6];
    let data = table(&values);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = output.summaries().first().expect("one summary");

    assert_eq!(summary.boxes()[0].lower(), summary.boxes()[0].upper());
    assert_eq!(summary.median(), 5.0);
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
    let error = SVBoxenPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a category is required");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));

    let error = SVBoxenPlotBuilder::new()
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
        plot(&data).depth(0).boxed(),
        plot(&data)
            .tail_rule(SVBoxenTailRule::proportion(1.5))
            .boxed(),
        plot(&data)
            .tail_rule(SVBoxenTailRule::proportion(0.0))
            .boxed(),
        plot(&data).width(1.5).boxed(),
        plot(&data).width(0.0).boxed(),
    ] {
        assert!(
            matches!(request, Err(SVError::InvalidConfiguration { .. })),
            "an impossible option fails at build"
        );
    }
    assert!(matches!(
        SVBoxenPlotBuilder::new().build(),
        Err(SVError::MissingDataSource)
    ));
    assert!(matches!(
        SVBoxenPlotBuilder::new()
            .data_source(&data)
            .value("value")
            .category("category")
            .build()
            .map(|_| ()),
        Err(SVError::MissingBackendSelection)
    ));
}

#[test]
fn outlier_marker_changes_the_marker_shape() {
    let data = table(&VALUES);
    let output = plot(&data)
        .show_outliers(true)
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
fn width_option_shrinks_every_box() {
    let data = table(&VALUES);
    let wide = plot(&data)
        .width(0.8)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let narrow = plot(&data)
        .width(0.4)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let wide_width = filled_box_boxes(&wide)[0].2 - filled_box_boxes(&wide)[0].0;
    let narrow_width = filled_box_boxes(&narrow)[0].2 - filled_box_boxes(&narrow)[0].0;
    assert!((wide_width / narrow_width - 2.0).abs() < 1e-9);
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
trait SVBoxenBuildResult {
    fn boxed(self) -> Result<(), SVError>;
}

impl SVBoxenBuildResult for SVBoxenPlotBuilder<'_> {
    fn boxed(self) -> Result<(), SVError> {
        self.build().map(|_| ())
    }
}

/// The bounding boxes of the filled paths, in draw order.
fn filled_box_boxes(output: &super::SVBoxenRenderOutput) -> Vec<(f64, f64, f64, f64)> {
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

fn filled_box_extent(output: &super::SVBoxenRenderOutput) -> (f64, f64) {
    let (minimum_x, minimum_y, maximum_x, maximum_y) = filled_box_boxes(output)
        .first()
        .copied()
        .expect("the box body is a filled path");
    (maximum_x - minimum_x, maximum_y - minimum_y)
}
