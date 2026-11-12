//! The kernel-density options.

/// The kernel shape a density estimate convolves its observations with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVKernelShape {
    /// The Gaussian kernel, unbounded and smooth.
    #[default]
    Gaussian,
    /// The Epanechnikov kernel, compactly supported.
    Epanechnikov,
    /// The uniform kernel, compactly supported.
    Uniform,
}

impl SVKernelShape {
    /// Returns the kernel name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVKernelShape::Gaussian => "Gaussian",
            SVKernelShape::Epanechnikov => "Epanechnikov",
            SVKernelShape::Uniform => "Uniform",
        }
    }

    /// Evaluates the kernel at a standardised distance.
    pub(crate) fn evaluate(&self, standardised: f64) -> f64 {
        match self {
            SVKernelShape::Gaussian => {
                (-0.5 * standardised * standardised).exp() / (2.0 * std::f64::consts::PI).sqrt()
            }
            SVKernelShape::Epanechnikov => {
                if standardised.abs() <= 1.0 {
                    0.75 * (1.0 - standardised * standardised)
                } else {
                    0.0
                }
            }
            SVKernelShape::Uniform => {
                if standardised.abs() <= 1.0 {
                    0.5
                } else {
                    0.0
                }
            }
        }
    }
}

/// The configuration of [`SVStat::KernelDensity`](super::SVStat::KernelDensity).
///
/// A density over an unbounded stream is meaningless: when the input is
/// streaming, the estimate describes the current window and says so.
#[derive(Clone, Debug, PartialEq)]
pub struct SVKernelDensityOptions {
    bandwidth: Option<f64>,
    kernel: SVKernelShape,
    grid_count: usize,
    cut: f64,
}

impl Default for SVKernelDensityOptions {
    fn default() -> Self {
        Self {
            bandwidth: None,
            kernel: SVKernelShape::Gaussian,
            grid_count: 256,
            cut: 3.0,
        }
    }
}

impl SVKernelDensityOptions {
    /// Starts a Gaussian density with a derived bandwidth.
    pub fn new() -> Self {
        Self::default()
    }

    /// Fixes the bandwidth instead of deriving it by the rule of thumb.
    #[must_use]
    pub fn bandwidth(mut self, bandwidth: f64) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// Chooses the kernel shape.
    #[must_use]
    pub fn kernel(mut self, kernel: SVKernelShape) -> Self {
        self.kernel = kernel;
        self
    }

    /// Chooses how many samples the returned grid carries.
    #[must_use]
    pub fn grid_count(mut self, grid_count: usize) -> Self {
        self.grid_count = grid_count;
        self
    }

    /// Chooses how many bandwidths the grid extends beyond the data.
    #[must_use]
    pub fn cut(mut self, cut: f64) -> Self {
        self.cut = cut;
        self
    }

    /// Returns the explicit bandwidth, when one was fixed.
    pub fn explicit_bandwidth(&self) -> Option<f64> {
        self.bandwidth
    }

    /// Returns the kernel shape.
    pub fn kernel_shape(&self) -> SVKernelShape {
        self.kernel
    }

    /// Returns the number of grid samples.
    pub fn requested_grid_count(&self) -> usize {
        self.grid_count
    }

    /// Returns the cut factor.
    pub fn cut_factor(&self) -> f64 {
        self.cut
    }
}
