//! Companion tests for the sorted observation view.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::sorted_observations::SVSortedObservations;

#[test]
fn a_dense_column_is_sorted_without_an_implicit_zero() {
    let values = [3.0, -1.0, 2.0];
    let observations = SVObservationValues::from_view(SVColumnView::Dense(&values));
    let sorted = SVSortedObservations::from_observations(&observations);

    assert_eq!(sorted.value_at_rank(0), Some(-1.0));
    assert_eq!(sorted.value_at_rank(2), Some(3.0));
    assert_eq!(sorted.value_at_rank(3), None);
}

#[test]
fn the_implicit_zeros_are_a_multiplicity_not_an_expansion() {
    let indices = [1usize, 4];
    let values = [-2.0, 5.0];
    let sparse = SVSparseView::new(&indices, &values, 7).expect("valid sparse storage");
    let observations = SVObservationValues::from_view(SVColumnView::Sparse(sparse));
    let sorted = SVSortedObservations::from_observations(&observations);

    assert_eq!(sorted.value_at_rank(0), Some(-2.0));
    assert_eq!(sorted.value_at_rank(1), Some(0.0));
    assert_eq!(sorted.value_at_rank(5), Some(0.0));
    assert_eq!(sorted.value_at_rank(6), Some(5.0));
    assert_eq!(sorted.quantile(0.5), Some(0.0));
}

#[test]
fn quantiles_interpolate_between_neighbouring_ranks() {
    let values = [1.0, 2.0, 3.0, 4.0];
    let observations = SVObservationValues::from_view(SVColumnView::Dense(&values));
    let sorted = SVSortedObservations::from_observations(&observations);

    assert_eq!(sorted.quantile(0.25), Some(1.75));
    assert_eq!(sorted.quantile(0.5), Some(2.5));
    assert_eq!(sorted.quantile(0.75), Some(3.25));
    assert_eq!(sorted.quantile(1.0), Some(4.0));
}
