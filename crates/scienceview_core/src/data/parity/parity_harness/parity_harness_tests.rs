//! Companion tests for the parity harness, adopted by a reference kernel.
//!
//! The reference kernel below is the harness's proof of concept: it is written
//! once against [`SVColumnView`] and dispatched exactly once, so the same code
//! serves dense, sparse and streaming input. Real kernels adopt the same
//! three lines — build a harness, pass the kernel, let it assert.

use std::cell::RefCell;

use crate::sv_data::{
    SVColumnView, SVRepresentation, SVRepresentationParityHarness, sv_assert_representation_parity,
};

const DATASET: [f64; 7] = [1.0, 0.0, 3.0, 0.0, 5.0, 0.0, 7.0];

/// A reference kernel: the arithmetic mean of every logical value.
fn sv_mean_of_values(view: SVColumnView<'_>) -> f64 {
    view.dispatch(
        |values| {
            assert!(!values.is_empty(), "the reference kernel needs samples");
            values.iter().sum::<f64>() / values.len() as f64
        },
        |sparse| {
            let total: f64 = sparse.stored_entries().map(|(_, value)| value).sum();
            total / sparse.length() as f64
        },
        |streaming| {
            assert!(!streaming.values().is_empty(), "the window holds samples");
            streaming.values().iter().sum::<f64>() / streaming.length() as f64
        },
    )
}

#[test]
fn the_reference_kernel_agrees_across_the_three_representations() {
    let harness = SVRepresentationParityHarness::new(&DATASET);

    harness.assert_scalar_parity(sv_mean_of_values);
    assert_eq!(sv_mean_of_values(harness.dense_view()), 16.0 / 7.0);
}

#[test]
fn the_harness_visits_dense_sparse_and_streaming_exactly_once_each() {
    let harness = SVRepresentationParityHarness::new(&DATASET);
    let seen = RefCell::new(Vec::new());

    harness.assert_kernel_parity(|view| {
        seen.borrow_mut().push(view.representation());
        vec![view.length() as f64]
    });

    assert_eq!(
        seen.into_inner(),
        vec![
            SVRepresentation::Dense,
            SVRepresentation::Sparse,
            SVRepresentation::Streaming
        ]
    );
}

#[test]
fn the_sparse_representation_stores_only_the_non_zero_entries() {
    let harness = SVRepresentationParityHarness::new(&DATASET);
    let sparse = harness.sparse_view().expect("valid sparse storage");

    assert_eq!(sparse.length(), DATASET.len());
    assert_eq!(sparse.number_of_stored_values(), 4);
    assert_eq!(sparse.indices(), &[0, 2, 4, 6]);
}

#[test]
fn the_free_function_asserts_parity_for_a_vector_kernel() {
    sv_assert_representation_parity(&DATASET, |view| {
        view.dispatch(
            |values| values.iter().map(|value| value * 2.0).collect(),
            |sparse| {
                (0..sparse.length())
                    .map(|index| sparse.value_at(index).unwrap() * 2.0)
                    .collect()
            },
            |streaming| streaming.values().iter().map(|value| value * 2.0).collect(),
        )
    });
}

#[test]
fn a_smaller_window_is_compared_against_the_retained_tail() {
    let harness = SVRepresentationParityHarness::new(&DATASET).with_window_capacity(3);

    assert_eq!(harness.retained_sample_count(), 3);
    harness.assert_scalar_parity(sv_mean_of_values);
}

#[test]
fn a_zero_window_capacity_is_raised_to_one() {
    let harness = SVRepresentationParityHarness::new(&DATASET).with_window_capacity(0);

    assert_eq!(harness.window_capacity(), 1);
}

#[test]
#[should_panic(expected = "the streaming result differs")]
fn the_harness_rejects_a_kernel_that_treats_streaming_differently() {
    let harness = SVRepresentationParityHarness::new(&DATASET);

    harness.assert_scalar_parity(|view| match view.representation() {
        SVRepresentation::Streaming => 1_000.0,
        _ => sv_mean_of_values(view),
    });
}

#[test]
#[should_panic(expected = "dense and sparse disagree exactly")]
fn the_harness_rejects_a_sparse_result_that_differs_at_all() {
    let harness = SVRepresentationParityHarness::new(&DATASET);

    harness.assert_scalar_parity(|view| match view.representation() {
        SVRepresentation::Sparse => sv_mean_of_values(view) + 1e-6,
        _ => sv_mean_of_values(view),
    });
}
