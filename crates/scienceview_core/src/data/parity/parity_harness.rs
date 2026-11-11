//! The parity harness every kernel adopts.

use crate::sv_data::{SVColumnView, SVRepresentationTolerance, SVSparseView, SVStreamingView};
use crate::sv_error::SVError;
use crate::sv_window::SVRollingWindow;

/// Proves that a kernel produces the same output for the three representations.
///
/// The harness takes one dataset, derives the sparse storage from its non-zero
/// entries and replays the dataset through a bounded rolling window. A kernel
/// is then run against all three views and compared:
///
/// - dense against sparse **exactly**, because a sparse view holds the same
///   values at the same positions;
/// - dense against streaming within the harness tolerance, because a streaming
///   view is a bounded window of the same values.
///
/// When the window capacity is smaller than the dataset, the streaming result
/// is compared against the kernel applied to the retained tail — which is what
/// the window actually holds — instead of the full dataset.
#[derive(Clone, Debug)]
pub struct SVRepresentationParityHarness<'a> {
    dataset: &'a [f64],
    sparse_indices: Vec<usize>,
    sparse_values: Vec<f64>,
    tolerance: SVRepresentationTolerance,
    window_capacity: usize,
}

impl<'a> SVRepresentationParityHarness<'a> {
    /// Builds a harness over a borrowed dataset.
    ///
    /// The default window capacity equals the dataset length, so the streaming
    /// representation carries every sample and the three results must agree.
    pub fn new(dataset: &'a [f64]) -> Self {
        let (sparse_indices, sparse_values) = dataset
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != 0.0)
            .map(|(index, value)| (index, *value))
            .unzip();

        Self {
            dataset,
            sparse_indices,
            sparse_values,
            tolerance: SVRepresentationTolerance::DEFAULT,
            window_capacity: dataset.len().max(1),
        }
    }

    /// Replaces the tolerance used for the streaming comparison.
    #[must_use]
    pub fn with_tolerance(mut self, tolerance: SVRepresentationTolerance) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Replaces the capacity of the rolling window used for streaming.
    ///
    /// A capacity of zero is raised to one, because a window always holds at
    /// least one sample.
    #[must_use]
    pub fn with_window_capacity(mut self, capacity: usize) -> Self {
        self.window_capacity = capacity.max(1);
        self
    }

    /// Returns the tolerance in use.
    pub fn tolerance(&self) -> SVRepresentationTolerance {
        self.tolerance
    }

    /// Returns the capacity of the streaming window.
    pub fn window_capacity(&self) -> usize {
        self.window_capacity
    }

    /// Returns the borrowed dataset.
    pub fn dataset(&self) -> &'a [f64] {
        self.dataset
    }

    /// Returns the dataset as a dense view.
    pub fn dense_view(&self) -> SVColumnView<'a> {
        SVColumnView::Dense(self.dataset)
    }

    /// Returns the dataset as a sparse view over its non-zero entries.
    ///
    /// # Errors
    ///
    /// Returns the validation failure of [`SVSparseView::new`], which cannot
    /// happen for storage derived from a dense slice.
    pub fn sparse_view(&self) -> Result<SVSparseView<'_>, SVError> {
        SVSparseView::new(
            &self.sparse_indices,
            &self.sparse_values,
            self.dataset.len(),
        )
    }

    /// Returns the number of samples the streaming window retains.
    pub fn retained_sample_count(&self) -> usize {
        self.dataset.len().min(self.window_capacity)
    }

    /// Asserts that a vector-returning kernel agrees across representations.
    ///
    /// # Panics
    ///
    /// Panics with a message naming the disagreeing representation and
    /// position when dense and sparse differ, or when streaming exceeds the
    /// tolerance.
    pub fn assert_kernel_parity<Kernel>(&self, mut kernel: Kernel)
    where
        Kernel: FnMut(SVColumnView<'_>) -> Vec<f64>,
    {
        let dense = kernel(self.dense_view());

        let sparse_view = self
            .sparse_view()
            .expect("the harness derives valid sparse storage");
        let sparse = kernel(SVColumnView::Sparse(sparse_view));

        assert_eq!(
            dense.len(),
            sparse.len(),
            "the dense and sparse kernels returned different result lengths"
        );
        for (position, (dense_value, sparse_value)) in dense.iter().zip(sparse.iter()).enumerate() {
            assert_eq!(
                dense_value, sparse_value,
                "dense and sparse disagree exactly at position {position}: {dense_value} != {sparse_value}"
            );
        }

        let expected_streaming = self.expected_streaming_output(&mut kernel, &dense);
        let streaming = self.streaming_output(&mut kernel);

        assert_eq!(
            streaming.len(),
            expected_streaming.len(),
            "the streaming kernel returned a different number of results"
        );
        for (position, (expected, actual)) in
            expected_streaming.iter().zip(streaming.iter()).enumerate()
        {
            assert!(
                self.tolerance.is_within_tolerance(*expected, *actual),
                "the streaming result differs at position {position}: {actual} is outside the tolerance around {expected}"
            );
        }
    }

    /// Asserts that a scalar-returning kernel agrees across representations.
    ///
    /// # Panics
    ///
    /// Panics when the three representations disagree, exactly as
    /// [`SVRepresentationParityHarness::assert_kernel_parity`] does.
    pub fn assert_scalar_parity<Kernel>(&self, mut kernel: Kernel)
    where
        Kernel: FnMut(SVColumnView<'_>) -> f64,
    {
        self.assert_kernel_parity(move |view| vec![kernel(view)]);
    }

    fn expected_streaming_output<Kernel>(&self, kernel: &mut Kernel, dense: &[f64]) -> Vec<f64>
    where
        Kernel: FnMut(SVColumnView<'_>) -> Vec<f64>,
    {
        if self.window_capacity >= self.dataset.len() {
            return dense.to_vec();
        }

        let retained = self.retained_sample_count();
        kernel(SVColumnView::Dense(
            &self.dataset[self.dataset.len() - retained..],
        ))
    }

    fn streaming_output<Kernel>(&self, kernel: &mut Kernel) -> Vec<f64>
    where
        Kernel: FnMut(SVColumnView<'_>) -> Vec<f64>,
    {
        let mut window = SVRollingWindow::with_capacity(self.window_capacity)
            .expect("the harness keeps the window capacity above zero");
        window.extend(self.dataset.iter().copied());

        let sequence = window.sequence();
        let dropped_samples = window.dropped_samples();
        let view = SVColumnView::Streaming(SVStreamingView::new(
            window.snapshot(),
            sequence,
            dropped_samples,
        ));

        kernel(view)
    }
}

/// Asserts representation parity for a kernel over a borrowed dataset.
///
/// # Panics
///
/// Panics when the three representations disagree; see
/// [`SVRepresentationParityHarness`].
pub fn sv_assert_representation_parity<Kernel>(dataset: &[f64], kernel: Kernel)
where
    Kernel: FnMut(SVColumnView<'_>) -> Vec<f64>,
{
    SVRepresentationParityHarness::new(dataset).assert_kernel_parity(kernel);
}

#[cfg(test)]
#[path = "parity_harness_tests.rs"]
mod parity_harness_tests;
