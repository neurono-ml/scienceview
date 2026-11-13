//! Companion tests for the counting kernel.

use crate::sv_data::{SVColumnView, SVSparseView};
use crate::sv_error::SVError;
use crate::sv_stat::{
    SVCountOptions, SVCountOrder, SVCountResult, SVStat, SVStatInput, SVStatResult,
};

fn apply_count(options: SVCountOptions, values: &[f64]) -> SVCountResult {
    let input = SVStatInput::new(SVColumnView::Dense(values));
    match SVStat::count(options)
        .apply(input)
        .expect("the counting configuration is valid")
    {
        SVStatResult::Categories(result) => result,
        other => panic!("the count stat must produce categories, received {other:?}"),
    }
}

#[test]
fn the_data_order_keeps_the_first_appearance_of_each_category() {
    let result = apply_count(SVCountOptions::new(), &[2.0, 1.0, 2.0, 3.0, 1.0]);

    assert_eq!(result.categories(), &[2.0, 1.0, 3.0]);
    assert_eq!(result.counts(), &[2, 2, 1]);
}

#[test]
fn ascending_and_descending_orders_sort_the_categories() {
    let values = [3.0, 1.0, 2.0];

    let ascending = apply_count(
        SVCountOptions::new().order(SVCountOrder::Ascending),
        &values,
    );
    let descending = apply_count(
        SVCountOptions::new().order(SVCountOrder::Descending),
        &values,
    );

    assert_eq!(ascending.categories(), &[1.0, 2.0, 3.0]);
    assert_eq!(descending.categories(), &[3.0, 2.0, 1.0]);
}

#[test]
fn categories_are_matched_by_exact_value() {
    let result = apply_count(SVCountOptions::new(), &[1.0, 1.0, 0.0]);

    assert_eq!(result.categories(), &[1.0, 0.0]);
    assert_eq!(result.counts(), &[2, 1]);
}

#[test]
fn a_sparse_column_counts_its_implicit_zero_category_in_data_order() {
    let indices = [1usize, 3];
    let values = [2.0, 3.0];
    let sparse = SVSparseView::new(&indices, &values, 4).expect("valid sparse storage");
    let input = SVStatInput::new(SVColumnView::Sparse(sparse));

    let result = match SVStat::count(SVCountOptions::new())
        .apply(input)
        .expect("the sparse column is countable")
    {
        SVStatResult::Categories(result) => result,
        other => panic!("the count stat must produce categories, received {other:?}"),
    };

    assert_eq!(result.categories(), &[0.0, 2.0, 3.0]);
    assert_eq!(result.counts(), &[2, 1, 1]);
}

#[test]
fn an_empty_column_is_rejected() {
    let input = SVStatInput::new(SVColumnView::Dense(&[]));

    let error = SVStat::count(SVCountOptions::new())
        .apply(input)
        .expect_err("an empty column has no categories");

    assert_eq!(error, SVError::EmptyInput { operation: "count" });
}

#[test]
fn a_non_finite_category_is_rejected() {
    let values = [1.0, f64::INFINITY];
    let input = SVStatInput::new(SVColumnView::Dense(&values));

    let error = SVStat::count(SVCountOptions::new())
        .apply(input)
        .expect_err("infinity is not a category");

    assert_eq!(
        error,
        SVError::NonFiniteValue {
            operation: "count",
            position: 1,
        }
    );
}

#[test]
fn a_configured_weight_column_is_rejected_instead_of_ignored() {
    let values = [1.0, 2.0, 3.0];
    let weights = [1.0, 1.0, 1.0];
    let error = SVStat::count(SVCountOptions::new())
        .apply(
            SVStatInput::new(SVColumnView::Dense(&values))
                .with_weights(SVColumnView::Dense(&weights)),
        )
        .expect_err("counting does not accept weights");

    assert!(matches!(error, SVError::InvalidConfiguration { .. }));
}
