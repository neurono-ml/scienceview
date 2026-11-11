//! The harness value: one dataset, three views, one tolerance policy.

use crate::sv_data::{SVColumnView, SVRepresentationTolerance, SVSparseView};
use crate::sv_error::SVError;

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
