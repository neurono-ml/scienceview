//! Explicit configuration failures.

use super::super::super::SVStepPlot;
use super::super::test_sources::SVTestSource;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::SVColumnRole;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_stat::{SVBinOptions, SVStat};

fn expect_build_error(result: Result<SVStepPlot<'_>, SVError>) -> SVError {
    match result {
        Ok(_) => panic!("the configuration must be rejected"),
        Err(error) => error,
    }
}

#[test]
fn a_missing_column_is_reported() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0])
        .with_dense("y", SVColumnRole::Y, &[1.0]);
    let error = expect_build_error(
        SVStepPlot::builder()
            .data_source(&table)
            .x("x")
            .y("absent")
            .backend(SVBackendKind::Svg)
            .build(),
    );
    assert!(matches!(error, SVError::ColumnNotFound { .. }), "{error:?}");
}

#[test]
fn a_mistyped_label_column_is_rejected() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::Label, &[1.0])
        .with_dense("y", SVColumnRole::Y, &[1.0]);
    let error = expect_build_error(
        SVStepPlot::builder()
            .data_source(&table)
            .x("x")
            .y("y")
            .backend(SVBackendKind::Svg)
            .build(),
    );
    assert!(
        matches!(error, SVError::InvalidConfiguration { .. }),
        "{error:?}"
    );
}

#[test]
fn a_non_aggregate_stat_is_rejected() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0])
        .with_dense("y", SVColumnRole::Y, &[1.0]);
    let error = expect_build_error(
        SVStepPlot::builder()
            .data_source(&table)
            .x("x")
            .y("y")
            .aggregate(SVStat::bin(SVBinOptions::new()))
            .backend(SVBackendKind::Svg)
            .build(),
    );
    assert!(
        matches!(error, SVError::InvalidConfiguration { .. }),
        "{error:?}"
    );
}

#[test]
fn the_backend_selection_is_required() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0])
        .with_dense("y", SVColumnRole::Y, &[1.0]);
    let error = expect_build_error(
        SVStepPlot::builder()
            .data_source(&table)
            .x("x")
            .y("y")
            .build(),
    );
    assert_eq!(error, SVError::MissingBackendSelection);
}

#[test]
fn a_backend_without_a_recipe_renderer_is_rejected_at_build() {
    let table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[1.0])
        .with_dense("y", SVColumnRole::Y, &[1.0]);
    let error = expect_build_error(
        SVStepPlot::builder()
            .data_source(&table)
            .x("x")
            .y("y")
            .backend(SVBackendKind::Native)
            .build(),
    );
    assert!(
        matches!(error, SVError::InvalidConfiguration { .. }),
        "{error:?}"
    );
}
