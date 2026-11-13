//! The documented defaults and each option that changes the output.

use super::super::SVECdfPlot;
use super::test_support::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVLength;
use scienceview_core::sv_mark::SVBarOrientation;
use scienceview_core::sv_scale::SVScaleKind;

const VALUES: [f64; 4] = [3.0, 1.0, 2.0, 3.0];
const LINE_WIDTH: SVLength = SVLength::absolute(3.0);

fn source(values: &[f64]) -> SVTestSource<'_> {
    SVTestSource::new().with_dense("value", SVColumnRole::Y, values)
}

fn build<'a>(source: &'a SVTestSource<'a>) -> SVECdfPlot<'a> {
    SVECdfPlot::builder()
        .data_source(source)
        .value("value")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn the_documented_defaults_are_applied() {
    let table = source(&VALUES);
    let plot = build(&table);

    assert_eq!(plot.backend_kind(), SVBackendKind::Svg);
    assert_eq!(plot.execution_mode(), SVExecutionMode::Automatic);
    assert!(plot.is_normalised());
    assert_eq!(plot.orientation(), SVBarOrientation::Vertical);
    assert!(!plot.is_complementary());
    assert!(plot.line_width().is_none());
    assert_eq!(plot.series_column(), None);
    assert_eq!(plot.viewport().width(), 640.0);
    assert_eq!(plot.viewport().height(), 400.0);
}

#[test]
fn the_curve_reaches_one_at_the_maximum_and_is_a_step() {
    let table = source(&VALUES);
    let composition = build(&table).compose().expect("the curve composes");
    let series = &composition.series()[0];

    assert_eq!(series.sample_horizontal(), &[1.0, 2.0, 3.0]);
    assert_eq!(series.sample_vertical(), &[0.25, 0.5, 1.0]);
    assert_eq!(series.horizontal(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
    assert_eq!(series.vertical(), &[0.0, 0.25, 0.25, 0.5, 0.5, 1.0]);

    // Every segment is horizontal then vertical: no sloped line exists.
    let vertical = series.vertical();
    for index in (0..vertical.len() - 1).step_by(2) {
        assert_eq!(
            series.horizontal()[index],
            series.horizontal()[index + 1],
            "the riser at index {index} is vertical"
        );
    }
    for index in (1..vertical.len() - 1).step_by(2) {
        assert_eq!(
            vertical[index],
            vertical[index + 1],
            "the run at index {index} is horizontal"
        );
    }
}

#[test]
fn one_observation_rises_to_one_at_its_only_value() {
    let values = [7.0];
    let table = source(&values);
    let composition = build(&table).compose().expect("the curve composes");
    let series = &composition.series()[0];

    assert_eq!(series.sample_horizontal(), &[7.0]);
    assert_eq!(series.sample_vertical(), &[1.0]);
    assert_eq!(series.horizontal(), &[7.0, 7.0]);
    assert_eq!(series.vertical(), &[0.0, 1.0]);
}

#[test]
fn all_equal_observations_produce_one_riser() {
    let values = [5.0, 5.0, 5.0];
    let table = source(&values);
    let composition = build(&table).compose().expect("the curve composes");
    let series = &composition.series()[0];

    assert_eq!(series.sample_horizontal(), &[5.0]);
    assert_eq!(series.sample_vertical(), &[1.0]);
    assert_eq!(series.horizontal(), &[5.0, 5.0]);
    assert_eq!(series.vertical(), &[0.0, 1.0]);
    assert_eq!(composition.point_count(), 3);
}

#[test]
fn normalise_off_reaches_the_observation_count() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .normalise(false)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");

    assert_eq!(composition.series()[0].sample_vertical(), &[1.0, 2.0, 4.0]);
    assert_eq!(composition.series()[0].vertical().last(), Some(&4.0));
}

#[test]
fn the_complementary_curve_descends_from_one() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .complementary(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");

    assert_eq!(composition.series()[0].sample_vertical(), &[0.75, 0.5, 0.0]);
    assert_eq!(composition.series()[0].vertical().first(), Some(&1.0));
    assert_eq!(composition.series()[0].vertical().last(), Some(&0.0));
}

#[test]
fn horizontal_orientation_swaps_the_two_channels() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .orient(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");

    assert_eq!(composition.orientation(), SVBarOrientation::Horizontal);
    assert_eq!(
        composition.series()[0].sample_horizontal(),
        &[0.25, 0.5, 1.0]
    );
    assert_eq!(composition.series()[0].sample_vertical(), &[1.0, 2.0, 3.0]);
    assert_eq!(composition.horizontal_scale_kind(), SVScaleKind::Continuous);
}

#[test]
fn horizontal_orientation_expands_the_same_step_vertices() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .orient(SVBarOrientation::Horizontal)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");
    let series = &composition.series()[0];

    // Mirroring the vertical curve swaps the channels, so the risers run along
    // the probability channel at a constant observation and the runs sit at a
    // constant probability. The curve still starts at probability zero on the
    // minimum observation, never at value zero.
    assert_eq!(series.horizontal(), &[0.0, 0.25, 0.25, 0.5, 0.5, 1.0]);
    assert_eq!(series.vertical(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
}

#[test]
fn a_complementary_horizontal_curve_starts_at_one_on_the_minimum() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .orient(SVBarOrientation::Horizontal)
        .complementary(true)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");
    let series = &composition.series()[0];

    assert_eq!(series.horizontal(), &[1.0, 0.75, 0.75, 0.5, 0.5, 0.0]);
    assert_eq!(series.vertical(), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0]);
}

#[test]
fn line_width_reaches_the_composed_curve() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .line_width(LINE_WIDTH)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");

    assert_eq!(plot.line_width(), Some(LINE_WIDTH));
    let composition = plot.compose().expect("the curve composes");
    let geometries = composition.mark_geometries().expect("the line draws");
    assert_eq!(geometries.len(), 1);
}

#[test]
fn a_series_column_splits_the_chart_into_one_curve_per_group() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let series = [1.0, 1.0, 2.0, 2.0];
    let table = SVTestSource::new()
        .with_dense("value", SVColumnRole::Y, &values)
        .with_dense("scenario", SVColumnRole::Grouping, &series);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .series("scenario")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("both curves compose");

    assert_eq!(composition.number_of_series(), 2);
    assert_eq!(composition.series()[0].label(), "series 1");
    assert_eq!(composition.series()[0].sample_horizontal(), &[1.0, 2.0]);
    assert_eq!(composition.series()[0].observation_count(), 2);
    assert_eq!(composition.series()[1].label(), "series 2");
    assert_eq!(composition.series()[1].sample_horizontal(), &[3.0, 4.0]);
}

#[test]
fn a_title_reaches_the_serialized_document() {
    let table = source(&VALUES);
    let plot = SVECdfPlot::builder()
        .data_source(&table)
        .value("value")
        .title("Distribution of speed")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration is complete");
    let composition = plot.compose().expect("the curve composes");

    assert_eq!(composition.title(), Some("Distribution of speed"));
    #[cfg(feature = "render-svg")]
    {
        let document = composition.to_svg().expect("svg");
        assert!(document.contains("<title>Distribution of speed</title>"));
    }
}
