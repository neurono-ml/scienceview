//! Unit tests for `SVPointPlot` and its public options (FDR-123).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape, SVMarkShape};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use super::{SVPointMissingValuePolicy, SVPointOrdering, SVPointPlotBuilder, SVPointRenderOutput};

const VALUES: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const CATEGORIES: [f64; 6] = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
const SERIES: [f64; 6] = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];

fn table<'values>(values: &'values [f64], categories: &'values [f64]) -> SVDataTable<'values> {
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, values)
        .with_column("category", SVColumnRole::Grouping, categories)
        .build()
        .expect("the columns share one length")
}

fn plot<'values>(data: &'values SVDataTable<'values>) -> SVPointPlotBuilder<'values> {
    SVPointPlotBuilder::new()
        .data_source(data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

fn draw(builder: SVPointPlotBuilder<'_>) -> SVPointRenderOutput {
    builder.build().expect("completes").render().expect("draws")
}

#[test]
fn the_default_point_is_the_mean_of_its_category() {
    let values = [1.0, 2.0, 30.0];
    let categories = [0.0, 0.0, 0.0];
    let data = table(&values, &categories);
    let output = draw(plot(&data));

    assert_eq!(output.estimates().len(), 1);
    assert!((output.estimates()[0].value() - 11.0).abs() < 1e-12);
    assert_eq!(output.estimates()[0].number_of_observations(), 3);
    assert_eq!(output.number_of_points(), values.len());
}

#[test]
fn estimator_selects_a_different_summary() {
    let values = [1.0, 2.0, 30.0];
    let categories = [0.0, 0.0, 0.0];
    let data = table(&values, &categories);

    let mean = draw(plot(&data));
    let median = draw(plot(&data).estimator(SVAggregateMethod::Median));
    let total = draw(plot(&data).estimator(SVAggregateMethod::Sum));

    assert!((mean.estimates()[0].value() - 11.0).abs() < 1e-12);
    assert!((median.estimates()[0].value() - 2.0).abs() < 1e-12);
    assert!((total.estimates()[0].value() - 33.0).abs() < 1e-12);
}

#[test]
fn the_default_error_is_one_standard_deviation() {
    let values = [1.0, 2.0, 3.0];
    let categories = [0.0, 0.0, 0.0];
    let data = table(&values, &categories);
    let output = draw(plot(&data));
    let error = output.estimates()[0]
        .error()
        .expect("an interval is reported");
    assert!(error.lower() < 2.0 && error.upper() > 2.0);
    assert!((error.upper() - 2.0 - 1.0).abs() < 1e-12);
}

#[test]
fn a_single_observation_reports_no_interval() {
    let values = [4.0];
    let categories = [0.0];
    let data = table(&values, &categories);
    let output = draw(plot(&data));
    assert!(output.estimates()[0].error().is_none());
}

#[test]
fn confidence_interval_widens_the_percentile_band() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let categories = [0.0; 10];
    let data = table(&values, &categories);

    let narrow = draw(
        plot(&data)
            .error(SVAggregateErrorKind::Percentile(0.95))
            .confidence_interval(0.5),
    );
    let wide = draw(
        plot(&data)
            .error(SVAggregateErrorKind::Percentile(0.95))
            .confidence_interval(0.99),
    );
    let narrow = narrow.estimates()[0].error().expect("an interval");
    let wide = wide.estimates()[0].error().expect("an interval");
    assert!(wide.upper() - wide.lower() > narrow.upper() - narrow.lower());
}

#[test]
fn series_dodges_the_points_inside_the_category_slot() {
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .with_column("series", SVColumnRole::Grouping, &SERIES)
        .build()
        .expect("the columns share one length");
    let output = draw(plot(&data).series("series"));

    assert_eq!(output.estimates().len(), 6);
    assert!((output.estimates()[0].position() - -0.2).abs() < 1e-6);
    assert!((output.estimates()[1].position() - 0.2).abs() < 1e-6);
    assert_eq!(output.series().len(), 2);
}

#[test]
fn ordering_rearranges_the_categories() {
    let values = [5.0, 1.0, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);

    let ascending = draw(plot(&data).ordering(SVPointOrdering::Ascending));
    let descending = draw(plot(&data).ordering(SVPointOrdering::Descending));
    let data_order = draw(plot(&data));

    assert_eq!(ascending.categories(), &[1.0, 2.0, 0.0]);
    assert_eq!(descending.categories(), &[0.0, 2.0, 1.0]);
    assert_eq!(data_order.categories(), &[0.0, 1.0, 2.0]);
}

#[test]
fn join_draws_one_line_per_series() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let categories = [0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
    let series = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .with_column("series", SVColumnRole::Grouping, &series)
        .build()
        .expect("the columns share one length");

    let without = draw(plot(&data).series("series"));
    let with = draw(plot(&data).series("series").join(true));
    assert_eq!(with.geometries().len(), without.geometries().len() + 2);
    assert_eq!(super::test_support::dot_centres(&with).len(), 6);
}

#[test]
fn the_marker_shape_changes_the_geometry() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let circle = draw(plot(&data));
    let square = draw(plot(&data).marker(SVDotShape::Square));

    let has_rectangle = square
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .any(|primitive| matches!(primitive.shape(), SVMarkShape::Rectangle(_)));
    assert!(super::test_support::dot_centres(&circle).len() == 2);
    assert!(has_rectangle);
}

#[test]
fn caps_can_be_switched_off() {
    let values = [1.0, 2.0, 3.0];
    let categories = [0.0, 0.0, 0.0];
    let data = table(&values, &categories);
    let capped = draw(plot(&data));
    let uncapped = draw(plot(&data).caps(false));

    assert!(
        super::test_support::path_command_count(&uncapped)
            < super::test_support::path_command_count(&capped)
    );
}

#[test]
fn orientation_turns_the_points_sideways() {
    let values = [1.0, 2.0, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);
    let vertical = draw(plot(&data));
    let horizontal = draw(plot(&data).orient(SVBarOrientation::Horizontal));

    let vertical = super::test_support::dot_centres(&vertical);
    let horizontal = super::test_support::dot_centres(&horizontal);
    assert!(vertical[0].0 < vertical[1].0);
    assert!(horizontal[0].1 > horizontal[1].1);
}

#[test]
fn an_explicit_colour_replaces_the_palette() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let colour = SVColour::new(10, 20, 30, 255);
    let output = draw(plot(&data).colour(colour));

    let painted = output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .find_map(|primitive| primitive.paint().fill());
    assert_eq!(painted, Some(colour));
}

#[test]
fn a_missing_value_is_omitted_by_default_and_rejected_on_request() {
    let values = [1.0, f64::NAN, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);

    let omitted = draw(plot(&data));
    assert_eq!(omitted.number_of_points(), 2);
    assert_eq!(omitted.estimates().len(), 2);

    let error = plot(&data)
        .missing_value_policy(SVPointMissingValuePolicy::Error)
        .build()
        .expect("the configuration is complete")
        .render()
        .expect_err("a non-finite value is rejected");
    assert!(matches!(error, SVError::NonFiniteValue { .. }));
}

#[test]
fn a_missing_column_is_an_explicit_error() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let error = SVPointPlotBuilder::new()
        .data_source(&data)
        .value("missing")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("the value column does not exist");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "missing".to_string()
        }
    );
}

#[test]
fn a_mistyped_column_is_an_explicit_error() {
    let values = [1.0, 2.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Grouping, &values)
        .with_column("category", SVColumnRole::Grouping, &values)
        .build()
        .expect("the columns share one length");
    let error = SVPointPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a grouping column is not a value");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_builder_requires_a_backend_and_a_data_source() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);

    let missing_backend = SVPointPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .build()
        .expect_err("no backend was selected");
    assert_eq!(missing_backend, SVError::MissingBackendSelection);

    let missing_source = SVPointPlotBuilder::new()
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("no data source was attached");
    assert_eq!(missing_source, SVError::MissingDataSource);
}

#[test]
fn the_default_execution_mode_is_automatic() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let plot = plot(&data).build().expect("completes");
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
}

#[test]
fn the_builder_rejects_invalid_cap_width_and_confidence() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    assert!(matches!(
        plot(&data).cap_width(2.0).build(),
        Err(SVError::InvalidConfiguration { .. })
    ));
    assert!(matches!(
        plot(&data).confidence_interval(1.0).build(),
        Err(SVError::InvalidConfiguration { .. })
    ));
}
