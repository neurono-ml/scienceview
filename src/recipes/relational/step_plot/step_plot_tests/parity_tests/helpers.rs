//! Shared sources and builders for the parity tests.

use super::super::super::SVStepPlot;
use super::super::test_sources::{SVTestColumn, SVTestSource};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;

pub(super) fn source<'a>(
    horizontal: SVTestColumn<'a>,
    vertical: SVTestColumn<'a>,
) -> SVTestSource<'a> {
    SVTestSource::new()
        .with_column("x", SVColumnRole::X, horizontal)
        .with_column("y", SVColumnRole::Y, vertical)
}

pub(super) fn build<'a>(source: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the documented configuration builds")
}

pub(super) fn build_with_series<'a>(source: &'a SVTestSource<'a>) -> SVStepPlot<'a> {
    SVStepPlot::builder()
        .data_source(source)
        .x("x")
        .y("y")
        .series("series")
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration builds")
}
