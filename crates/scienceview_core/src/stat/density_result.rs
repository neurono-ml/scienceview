//! The density a kernel-density kernel returns.

use crate::sv_stat::SVKernelShape;

/// The derived view of
/// [`SVStat::KernelDensity`](super::SVStat::KernelDensity).
#[derive(Clone, Debug, PartialEq)]
pub struct SVKernelDensityResult {
    grid: Vec<f64>,
    density: Vec<f64>,
    bandwidth: f64,
    kernel: SVKernelShape,
    windowed: bool,
}

impl SVKernelDensityResult {
    /// Assembles a density result.
    pub(crate) fn new(
        grid: Vec<f64>,
        density: Vec<f64>,
        bandwidth: f64,
        kernel: SVKernelShape,
        windowed: bool,
    ) -> Self {
        Self {
            grid,
            density,
            bandwidth,
            kernel,
            windowed,
        }
    }

    /// Returns the evaluation grid, in ascending order.
    pub fn grid(&self) -> &[f64] {
        &self.grid
    }

    /// Returns one density estimate per grid sample.
    pub fn density(&self) -> &[f64] {
        &self.density
    }

    /// Returns the bandwidth the estimate used.
    pub fn bandwidth(&self) -> f64 {
        self.bandwidth
    }

    /// Returns the kernel shape the estimate used.
    pub fn kernel(&self) -> SVKernelShape {
        self.kernel
    }

    /// Returns `true` when the estimate describes only the current window.
    pub fn is_windowed(&self) -> bool {
        self.windowed
    }
}
