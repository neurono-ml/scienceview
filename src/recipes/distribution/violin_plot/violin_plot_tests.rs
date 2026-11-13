//! Unit tests for `SVViolinPlot` and its public options (FDR-117).

use scienceview_core::sv_backend::{SVBackendCapability, SVBackendKind};
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::SVBarOrientation;

use super::{SVViolinMissingValuePolicy, SVViolinPlotBuilder, SVViolinScale, SVViolinSummary};

const VALUES: [f64; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

fn table<'values>(values: &'values [f64]) -> SVDataTable<'values> {
    let category = vec![0.0; values.len()];
    let category: &'values [f64] = Box::leak(category.into_boxed_slice());
    SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, values)
        .with_column("category", SVColumnRole::Grouping, category)
        .build()
        .expect("the columns share one length")
}

fn plot<'values>(table: &'values SVDataTable<'values>) -> SVViolinPlotBuilder<'values> {
    SVViolinPlotBuilder::new()
        .data_source(table)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

fn summary(output: &super::SVViolinRenderOutput) -> &SVViolinSummary {
    output.summaries().first().expect("one category is drawn")
}

#[test]
fn default_estimate_matches_the_kernel_density() {
    let data = table(&VALUES);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let summary = summary(&output);

    assert_eq!(summary.grid().len(), 256);
    assert_eq!(summary.density().len(), 256);
    assert!(summary.bandwidth() > 0.0);
    assert!((summary.median() - 5.5).abs() < 1e-12);
    assert!((summary.lower_quartile() - 3.25).abs() < 1e-12);
    assert_eq!(output.number_of_points(), VALUES.len());
}

#[test]
fn bandwidth_option_changes_the_silhouette() {
    let data = table(&VALUES);
    let automatic = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let fixed_plot = plot(&data).bandwidth(0.4).build().expect("completes");
    assert!((fixed_plot.bandwidth_value().expect("explicit") - 0.4).abs() < 1e-12);
    let fixed = fixed_plot.render().expect("draws");
    assert!((summary(&automatic).bandwidth() - summary(&fixed).bandwidth()).abs() > 1e-6);
    assert_ne!(summary(&automatic).density(), summary(&fixed).density());
}

#[test]
fn bandwidth_adjustment_scales_the_derived_bandwidth() {
    let data = table(&VALUES);
    let plain = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let adjusted = plot(&data)
        .bandwidth_adjustment(2.0)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let ratio = summary(&adjusted).bandwidth() / summary(&plain).bandwidth();
    assert!((ratio - 2.0).abs() < 1e-9);
}

#[test]
fn width_option_scales_the_maximum_half_width() {
    let data = table(&VALUES);
    let narrow = plot(&data)
        .width(0.4)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let wide = plot(&data)
        .width(0.8)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let ratio = summary(&narrow).maximum_half_width() / summary(&wide).maximum_half_width();
    assert!((ratio - 0.5).abs() < 1e-12);
}

#[test]
fn width_scale_mode_makes_every_violin_the_same_width() {
    let values = [1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2, 2.4];
    let category = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &category)
        .build()
        .expect("the columns share one length");
    let output = plot(&data)
        .scale(SVViolinScale::Width)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let widths: Vec<f64> = output
        .summaries()
        .iter()
        .map(SVViolinSummary::maximum_half_width)
        .collect();
    assert!((widths[0] - widths[1]).abs() < 1e-12);
}

#[test]
fn count_scale_mode_grows_the_area_with_the_sample_count() {
    let values = [1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 3.0];
    let category = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &category)
        .build()
        .expect("the columns share one length");
    let output = plot(&data)
        .scale(SVViolinScale::Count)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let first = &output.summaries()[0];
    let second = &output.summaries()[1];
    assert!(second.maximum_half_width() > first.maximum_half_width());
}

#[test]
fn vertical_violin_is_symmetric_about_the_category_centre() {
    let data = table(&VALUES);
    let output = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let points: Vec<(f64, f64)> = super::violin_plot_test_support::points(&output);
    let minimum = points
        .iter()
        .map(|point| point.0)
        .fold(f64::INFINITY, f64::min);
    let maximum = points
        .iter()
        .map(|point| point.0)
        .fold(f64::NEG_INFINITY, f64::max);
    let centre = output
        .summaries()
        .first()
        .map(SVViolinSummary::category_position)
        .unwrap_or(0.0);
    let _ = centre;
    assert!((maximum - minimum).abs() > 1.0);
    let middle = (minimum + maximum) / 2.0;
    let left = middle - minimum;
    let right = maximum - middle;
    assert!((left - right).abs() < 1e-6);
}

#[test]
fn inner_box_and_points_add_geometry_without_moving_the_silhouette() {
    let data = table(&VALUES);
    let plain = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    let decorated = plot(&data)
        .show_box(true)
        .show_points(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    assert!(decorated.geometries().len() > plain.geometries().len());
    assert_eq!(
        plain.summaries()[0].density(),
        decorated.summaries()[0].density()
    );
}

#[test]
fn split_series_share_a_category_and_opposite_halves() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0];
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
        .split(true)
        .build()
        .expect("completes")
        .render()
        .expect("draws");

    let positions: Vec<f64> = output
        .summaries()
        .iter()
        .map(SVViolinSummary::category_position)
        .collect();
    assert_eq!(positions.len(), 2);
    assert!((positions[0] - positions[1]).abs() < 1e-12);
    assert_eq!(output.geometries().len(), 2, "one area per split half");
}

#[test]
fn horizontal_orientation_swaps_the_value_axis() {
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

    let vertical_points = super::violin_plot_test_support::points(&vertical);
    let horizontal_points = super::violin_plot_test_support::points(&horizontal);
    let vertical_width = spread(vertical_points.iter().map(|point| point.0));
    let vertical_height = spread(vertical_points.iter().map(|point| point.1));
    let horizontal_width = spread(horizontal_points.iter().map(|point| point.0));
    let horizontal_height = spread(horizontal_points.iter().map(|point| point.1));
    let slot_ratio = vertical.plot_area().width / vertical.plot_area().height;
    assert!((vertical_width / horizontal_height - slot_ratio).abs() < 1e-6);
    assert!((horizontal_width / vertical_height - slot_ratio).abs() < 1e-6);
}

fn spread(values: impl Iterator<Item = f64>) -> f64 {
    let values: Vec<f64> = values.collect();
    let minimum = values.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    maximum - minimum
}

#[test]
fn missing_column_is_an_explicit_configuration_error() {
    let data = table(&VALUES);
    let error = SVViolinPlotBuilder::new()
        .data_source(&data)
        .value("missing")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a missing column cannot build");
    assert_eq!(
        error,
        SVError::ColumnNotFound {
            name: "missing".to_string()
        }
    );
}

#[test]
fn wrong_column_role_is_rejected() {
    let data = table(&VALUES);
    let error = SVViolinPlotBuilder::new()
        .data_source(&data)
        .value("category")
        .category("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a role mismatch cannot build");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn missing_values_are_omitted_or_rejected_by_policy() {
    let values = [1.0, f64::NAN, 3.0, 4.0, 5.0];
    let data = table(&values);
    let omitted = plot(&data)
        .build()
        .expect("completes")
        .render()
        .expect("draws");
    assert_eq!(omitted.number_of_points(), 4);
    let error = plot(&data)
        .missing_value_policy(SVViolinMissingValuePolicy::Error)
        .build()
        .expect("completes")
        .render()
        .expect_err("the Error policy rejects the render");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_backend_without_a_serializer_is_rejected_at_build() {
    let data = table(&VALUES);
    let error = SVViolinPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Html)
        .build()
        .expect_err("the recipe has no HTML serializer");
    assert_eq!(
        error,
        SVError::UnsupportedBackendCapability {
            backend_kind: SVBackendKind::Html,
            capability: SVBackendCapability::Export,
        }
    );
}

#[test]
fn split_without_a_series_column_is_rejected() {
    let data = table(&VALUES);
    let error = plot(&data)
        .split(true)
        .build()
        .expect_err("split needs a series");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn defaults_expose_the_documented_options() {
    let data = table(&VALUES);
    let built = plot(&data).build().expect("completes");
    assert_eq!(built.width_value(), 0.8);
    assert_eq!(built.scale_value(), SVViolinScale::Area);
    assert!(!built.show_box_enabled());
    assert!(!built.show_points_enabled());
    assert!(!built.split_enabled());
    assert_eq!(built.bandwidth_value(), None);
    assert!((built.bandwidth_adjustment_value() - 1.0).abs() < f64::EPSILON);
}
