//! The regular lattice a contour plot draws.

/// A scalar field sampled on a regular lattice.
///
/// `values` is row-major over the vertical axis: the sample at column
/// `x_index` and row `y_index` is at `y_index * x_coordinates().len() +
/// x_index`. The coordinates are the lattice positions in data units, so the
/// first and last coordinate bound the drawn field.
#[derive(Clone, Debug, PartialEq)]
pub struct SVContourField {
    x_coordinates: Vec<f64>,
    y_coordinates: Vec<f64>,
    values: Vec<f64>,
}

impl SVContourField {
    pub(crate) fn new(x_coordinates: Vec<f64>, y_coordinates: Vec<f64>, values: Vec<f64>) -> Self {
        Self {
            x_coordinates,
            y_coordinates,
            values,
        }
    }

    /// Returns the horizontal lattice coordinates, increasing.
    pub fn x_coordinates(&self) -> &[f64] {
        &self.x_coordinates
    }

    /// Returns the vertical lattice coordinates, increasing.
    pub fn y_coordinates(&self) -> &[f64] {
        &self.y_coordinates
    }

    /// Returns every sample, row-major over the vertical axis.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns the number of columns.
    pub fn number_of_columns(&self) -> usize {
        self.x_coordinates.len()
    }

    /// Returns the number of rows.
    pub fn number_of_rows(&self) -> usize {
        self.y_coordinates.len()
    }

    /// Returns the number of samples.
    pub fn number_of_samples(&self) -> usize {
        self.values.len()
    }

    /// Returns one sample, or zero outside the lattice.
    pub fn value_at(&self, x_index: usize, y_index: usize) -> f64 {
        self.values
            .get(y_index * self.number_of_columns() + x_index)
            .copied()
            .unwrap_or(0.0)
    }

    /// Returns the smallest finite sample.
    pub fn lower_value(&self) -> f64 {
        self.sv_finite_values().fold(f64::INFINITY, f64::min)
    }

    /// Returns the largest finite sample.
    pub fn upper_value(&self) -> f64 {
        self.sv_finite_values().fold(f64::NEG_INFINITY, f64::max)
    }

    fn sv_finite_values(&self) -> impl Iterator<Item = f64> + '_ {
        self.values
            .iter()
            .copied()
            .filter(|value| value.is_finite())
    }
}
