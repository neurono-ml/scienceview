//! Sparse ordering columns and implicit zeros.

use super::super::super::{SVStepDuplicatePolicy, SVStepPlot};
use super::super::test_sources::{SVTestColumn, SVTestSource};
use super::helpers::{build, build_with_series, source};
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_data::{SVColumnRole, SVRepresentation, SVSparseView};

#[test]
fn a_sparse_column_keeps_its_stored_entries() {
    let indices = [0, 99_999];
    let values = [0.0, 1.0];
    let vertical_values = [1.0, 2.0];
    let sparse = SVSparseView::new(&indices, &values, 100_000).expect("the view is valid");

    assert_eq!(sparse.number_of_stored_values(), 2);
    assert_eq!(sparse.length(), 100_000);

    let table = source(
        SVTestColumn::Sparse(sparse),
        SVTestColumn::Sparse(
            SVSparseView::new(&indices, &vertical_values, 100_000).expect("the view is valid"),
        ),
    );
    let composition = build(&table).compose().expect("the sparse plot composes");
    assert_eq!(composition.representation(), SVRepresentation::Sparse);
    assert_eq!(composition.point_count(), 2);
    assert_eq!(composition.series()[0].sample_horizontal().len(), 2);
    assert_eq!(composition.series()[0].horizontal().len(), 3);
}

#[test]
fn a_sparse_first_wins_zero_keeps_the_first_row() {
    let indices = [2];
    let values = [0.0];
    let vertical = [0.0, 10.0, 20.0];
    let sparse_x = SVSparseView::new(&indices, &values, 3).expect("the view is valid");
    let table = source(
        SVTestColumn::Sparse(sparse_x),
        SVTestColumn::Dense(&vertical),
    );
    let composition = SVStepPlot::builder()
        .data_source(&table)
        .x("x")
        .y("y")
        .duplicate_policy(SVStepDuplicatePolicy::FirstWins)
        .backend(SVBackendKind::Svg)
        .build()
        .expect("the configuration builds")
        .compose()
        .expect("the sparse plot composes");
    assert_eq!(composition.series()[0].sample_vertical(), &[0.0]);
}

#[test]
fn a_sparse_last_wins_zero_skips_a_missing_value() {
    let indices = [0, 2];
    let values = [0.0, 5.0];
    let vertical = [1.0, f64::NAN, 3.0];
    let sparse_x = SVSparseView::new(&indices, &values, 3).expect("the view is valid");
    let table = source(
        SVTestColumn::Sparse(sparse_x),
        SVTestColumn::Dense(&vertical),
    );
    let composition = build(&table).compose().expect("the sparse plot composes");
    assert_eq!(composition.series()[0].sample_horizontal(), &[0.0, 5.0]);
    assert_eq!(composition.series()[0].sample_vertical(), &[1.0, 3.0]);
}

#[test]
fn a_sparse_zero_survives_a_non_zero_sample_between_zero_runs() {
    let indices = [1];
    let values = [5.0];
    let first_vertical = [f64::NAN, 10.0, 20.0];
    let first_x = SVSparseView::new(&indices, &values, 3).expect("the view is valid");
    let first_table = source(
        SVTestColumn::Sparse(first_x),
        SVTestColumn::Dense(&first_vertical),
    );
    let first = build(&first_table).compose().expect("the plot composes");
    assert_eq!(first.series()[0].sample_horizontal(), &[0.0, 5.0]);
    assert_eq!(first.series()[0].sample_vertical(), &[20.0, 10.0]);

    let last_vertical = [20.0, 10.0, f64::NAN];
    let last_x = SVSparseView::new(&indices, &values, 3).expect("the view is valid");
    let last_table = source(
        SVTestColumn::Sparse(last_x),
        SVTestColumn::Dense(&last_vertical),
    );
    let last = build(&last_table).compose().expect("the plot composes");
    assert_eq!(last.series()[0].sample_horizontal(), &[0.0, 5.0]);
    assert_eq!(last.series()[0].sample_vertical(), &[20.0, 10.0]);
}

#[test]
fn a_sparse_x_with_series_matches_the_dense_series() {
    let indices = [1];
    let values = [0.0];
    let vertical = [1.0, 2.0, 3.0];
    let series = [0.0, 1.0, 0.0];
    let sparse_x = SVSparseView::new(&indices, &values, 3).expect("the view is valid");
    let sparse_table = SVTestSource::new()
        .with_column("x", SVColumnRole::X, SVTestColumn::Sparse(sparse_x))
        .with_dense("y", SVColumnRole::Y, &vertical)
        .with_dense("series", SVColumnRole::Grouping, &series);
    let dense_table = SVTestSource::new()
        .with_dense("x", SVColumnRole::X, &[0.0, 0.0, 0.0])
        .with_dense("y", SVColumnRole::Y, &vertical)
        .with_dense("series", SVColumnRole::Grouping, &series);

    let sparse = build_with_series(&sparse_table)
        .compose()
        .expect("the sparse plot composes");
    let dense = build_with_series(&dense_table)
        .compose()
        .expect("the dense plot composes");
    assert_eq!(sparse.series(), dense.series());
}
