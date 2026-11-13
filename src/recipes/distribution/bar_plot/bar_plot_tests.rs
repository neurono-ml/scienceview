//! Unit tests for `SVBarPlot` and its public options (FDR-121).

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVDataTable};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_mark::{SVBarOrientation, SVFormatSpec, SVMarkShape};
use scienceview_core::sv_scale::SVColour;
use scienceview_core::sv_stat::{SVAggregateErrorKind, SVAggregateMethod};

use super::{
    SVBarArrangement, SVBarMissingValuePolicy, SVBarOrdering, SVBarPlotBuilder, SVBarRenderOutput,
};

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

fn plot<'values>(data: &'values SVDataTable<'values>) -> SVBarPlotBuilder<'values> {
    SVBarPlotBuilder::new()
        .data_source(data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
}

fn draw(builder: SVBarPlotBuilder<'_>) -> SVBarRenderOutput {
    builder.build().expect("completes").render().expect("draws")
}

#[test]
fn default_bar_is_the_mean_of_its_category() {
    let values = [1.0, 3.0, 5.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);
    let output = draw(plot(&data));

    assert_eq!(output.bins().len(), 3);
    for (bin, expected) in output.bins().iter().zip(values) {
        assert!((bin.value() - expected).abs() < 1e-12);
        assert!((bin.top() - expected).abs() < 1e-12);
        assert_eq!(bin.baseline(), 0.0);
        assert!((bin.width() - 0.8).abs() < 1e-6);
        assert_eq!(bin.number_of_observations(), 1);
    }
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

    assert!((mean.bins()[0].value() - 11.0).abs() < 1e-12);
    assert!((median.bins()[0].value() - 2.0).abs() < 1e-12);
    assert!((total.bins()[0].value() - 33.0).abs() < 1e-12);
}

#[test]
fn error_adds_an_interval_and_a_drawn_segment() {
    let values = [1.0, 2.0, 3.0];
    let categories = [0.0, 0.0, 0.0];
    let data = table(&values, &categories);
    let without = draw(plot(&data));
    let with = draw(plot(&data).error(SVAggregateErrorKind::StandardError));

    assert!(without.bins()[0].error().is_none());
    let error = with.bins()[0].error().expect("an interval is reported");
    assert!(error.lower() < 2.0 && error.upper() > 2.0);
    assert!(with.geometries().len() > without.geometries().len());
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
    let narrow = narrow.bins()[0].error().expect("an interval");
    let wide = wide.bins()[0].error().expect("an interval");
    let narrow_width = narrow.upper() - narrow.lower();
    let wide_width = wide.upper() - wide.lower();
    assert!(
        wide_width > narrow_width,
        "{wide_width} is not wider than {narrow_width}"
    );
}

#[test]
fn series_dodges_the_bars_inside_the_category_slot() {
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .with_column("series", SVColumnRole::Grouping, &SERIES)
        .build()
        .expect("the columns share one length");
    let output = draw(plot(&data).series("series"));

    assert_eq!(output.bins().len(), 6);
    assert!((output.bins()[0].position() - -0.2).abs() < 1e-6);
    assert!((output.bins()[1].position() - 0.2).abs() < 1e-6);
    assert!((output.bins()[0].width() - 0.4).abs() < 1e-6);
    assert_eq!(output.series().len(), 2);
}

#[test]
fn stack_grows_each_series_from_the_previous_top() {
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &VALUES)
        .with_column("category", SVColumnRole::Grouping, &CATEGORIES)
        .with_column("series", SVColumnRole::Grouping, &SERIES)
        .build()
        .expect("the columns share one length");
    let output = draw(
        plot(&data)
            .series("series")
            .arrangement(SVBarArrangement::Stack),
    );

    assert_eq!(output.bins().len(), 6);
    let first = &output.bins()[0];
    let second = &output.bins()[1];
    assert_eq!(second.baseline(), first.top());
    assert!((second.top() - (first.value() + second.value())).abs() < 1e-12);
    assert!((second.width() - 0.8).abs() < 1e-6);
}

#[test]
fn ordering_rearranges_the_categories() {
    let values = [5.0, 1.0, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);

    let ascending = draw(plot(&data).ordering(SVBarOrdering::Ascending));
    let descending = draw(plot(&data).ordering(SVBarOrdering::Descending));
    let data_order = draw(plot(&data));

    assert_eq!(ascending.categories(), &[1.0, 2.0, 0.0]);
    assert_eq!(descending.categories(), &[0.0, 2.0, 1.0]);
    assert_eq!(data_order.categories(), &[0.0, 1.0, 2.0]);
}

#[test]
fn width_changes_the_drawn_rectangle() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let narrow = draw(plot(&data).width(0.2));
    let wide = draw(plot(&data).width(0.9));
    let narrow = super::test_support::rectangles(&narrow)[0].2;
    let wide = super::test_support::rectangles(&wide)[0].2;
    assert!(wide > narrow);
}

#[test]
fn orientation_turns_the_bars_sideways() {
    let values = [1.0, 2.0, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);
    let vertical = draw(plot(&data));
    let horizontal = draw(plot(&data).orient(SVBarOrientation::Horizontal));

    let vertical = super::test_support::rectangles(&vertical);
    let horizontal = super::test_support::rectangles(&horizontal);
    let vertical = vertical.last().expect("a bar is drawn");
    let horizontal = horizontal.last().expect("a bar is drawn");
    assert!(vertical.3 > vertical.2);
    assert!(horizontal.2 > horizontal.3);
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
fn labels_draw_a_text_primitive() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    let output = draw(plot(&data).label(true).label_format(SVFormatSpec::Fixed(1)));

    let has_text = output
        .geometries()
        .iter()
        .flat_map(|geometry| geometry.primitives())
        .any(|primitive| matches!(primitive.shape(), SVMarkShape::Text(_)));
    assert!(has_text);
}

#[test]
fn a_missing_value_is_omitted_by_default_and_rejected_on_request() {
    let values = [1.0, f64::NAN, 3.0];
    let categories = [0.0, 1.0, 2.0];
    let data = table(&values, &categories);

    let omitted = draw(plot(&data));
    assert_eq!(omitted.number_of_points(), 2);
    assert_eq!(omitted.bins().len(), 2);

    let error = plot(&data)
        .missing_value_policy(SVBarMissingValuePolicy::Error)
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
    let error = SVBarPlotBuilder::new()
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
    let error = SVBarPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .backend(SVBackendKind::Svg)
        .build()
        .expect_err("a grouping column is not a value");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn a_weight_scales_the_mean() {
    let values = [1.0, 3.0];
    let categories = [0.0, 0.0];
    let weights = [3.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .with_column("weight", SVColumnRole::Weight, &weights)
        .build()
        .expect("the columns share one length");
    let output = draw(plot(&data).weight("weight"));
    assert!((output.bins()[0].value() - 1.5).abs() < 1e-12);
}

#[test]
fn weights_are_rejected_for_an_estimator_that_cannot_use_them() {
    let values = [1.0, 3.0];
    let categories = [0.0, 0.0];
    let weights = [3.0, 1.0];
    let data = SVDataTable::builder()
        .with_column("value", SVColumnRole::Y, &values)
        .with_column("category", SVColumnRole::Grouping, &categories)
        .with_column("weight", SVColumnRole::Weight, &weights)
        .build()
        .expect("the columns share one length");
    let error = plot(&data)
        .weight("weight")
        .estimator(SVAggregateMethod::Median)
        .build()
        .expect_err("a median cannot be weighted");
    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}

#[test]
fn the_builder_requires_a_backend_and_a_data_source() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);

    let missing_backend = SVBarPlotBuilder::new()
        .data_source(&data)
        .value("value")
        .category("category")
        .build()
        .expect_err("no backend was selected");
    assert_eq!(missing_backend, SVError::MissingBackendSelection);

    let missing_source = SVBarPlotBuilder::new()
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
fn the_builder_rejects_invalid_width_and_confidence() {
    let values = [1.0, 2.0];
    let categories = [0.0, 1.0];
    let data = table(&values, &categories);
    assert!(matches!(
        plot(&data).width(0.0).build(),
        Err(SVError::InvalidConfiguration { .. })
    ));
    assert!(matches!(
        plot(&data).confidence_interval(1.0).build(),
        Err(SVError::InvalidConfiguration { .. })
    ));
}
