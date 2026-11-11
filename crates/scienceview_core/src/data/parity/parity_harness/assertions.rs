//! The assertions: dense against sparse exactly, streaming within tolerance.

use super::SVRepresentationParityHarness;
use crate::sv_data::{SVColumnView, SVStreamingView};
use crate::sv_window::SVRollingWindow;

impl SVRepresentationParityHarness<'_> {
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
                self.tolerance().is_within_tolerance(*expected, *actual),
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
        if self.window_capacity() >= self.dataset().len() {
            return dense.to_vec();
        }

        let retained = self.retained_sample_count();
        let dataset = self.dataset();
        kernel(SVColumnView::Dense(&dataset[dataset.len() - retained..]))
    }

    fn streaming_output<Kernel>(&self, kernel: &mut Kernel) -> Vec<f64>
    where
        Kernel: FnMut(SVColumnView<'_>) -> Vec<f64>,
    {
        let mut window = SVRollingWindow::with_capacity(self.window_capacity())
            .expect("the harness keeps the window capacity above zero");
        window.extend(self.dataset().iter().copied());

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
