//! Ordering, duplicate positions, missing values and multi-series.

use super::super::super::{SVStepDuplicatePolicy, SVStepPlot};
use super::super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_stat::{SVAggregateOptions, SVStat};

fn build<'a>(source: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

#[test]
fn samples_are_ordered_by_x() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[3.0, 1.0, 2.0])
        .with_dense("y", SVColumnRole::Y, &[30.0, 10.0, 20.0]);
    let composition = build(&table).compose().expect("the plot composes");
    assert_eq!(
        composition.series()[0].horizontal(),
        &[1.0, 2.0, 2.0, 3.0, 3.0]
    );
}

#[test]
fn the_duplicate_policy_chooses_the_winning_observation() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0, 1.0, 2.0])
        .with_dense("y", SVColumnRole::Y, &[10.0, 11.0, 20.0]);

    let last = build(&table).compose().expect("the plot composes");
    assert_eq!(last.series()[0].vertical(), &[11.0, 11.0, 20.0]);

    let first = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .duplicate_policy(SVStepDuplicatePolicy::FirstWins)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");
    assert_eq!(first.series()[0].vertical(), &[10.0, 10.0, 20.0]);
}

#[test]
fn an_aggregate_collapses_duplicate_positions() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0, 1.0, 2.0])
        .with_dense("y", SVColumnRole::Y, &[10.0, 20.0, 30.0]);
    let composition = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .aggregate(SVStat::aggregate(SVAggregateOptions::new()))
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");
    assert_eq!(composition.series()[0].vertical(), &[15.0, 15.0, 30.0]);
}

#[test]
fn missing_values_are_omitted() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0, 2.0, 3.0, 4.0])
        .with_dense("y", SVColumnRole::Y, &[1.0, f64::NAN, 3.0, 4.0]);
    let composition = build(&table).compose().expect("the plot composes");
    assert_eq!(
        composition.series()[0].horizontal(),
        &[1.0, 3.0, 3.0, 4.0, 4.0]
    );
    assert_eq!(composition.point_count(), 3);
}

#[test]
fn multi_series_split_into_one_line_each() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0, 2.0, 1.0, 2.0])
        .with_dense("y", SVColumnRole::Y, &[1.0, 2.0, 3.0, 4.0])
        .with_dense("series", SVColumnRole::Grouping, &[0.0, 0.0, 1.0, 1.0]);
    let composition = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
        .compose()
        .expect("the plot composes");

    assert_eq!(composition.number_of_series(), 2);
    assert_eq!(composition.series()[0].label(), "series 0");
    assert_eq!(composition.series()[1].label(), "series 1");
    assert_ne!(
        composition.series()[0].colour(),
        composition.series()[1].colour()
    );
}
