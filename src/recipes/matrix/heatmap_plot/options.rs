//! The public option values of the heatmap.

use crate::sv_error::SVError;

/// How a missing (non-finite) cell is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVMissingValue {
    /// The cell is not drawn at all.
    Omit,
    /// The cell is drawn fully transparent, so the background shows through.
    #[default]
    Transparent,
    /// The cell is drawn in the theme surface colour.
    Neutral,
}

impl SVMissingValue {
    /// Returns the rule name as written in tests and observability.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVMissingValue::Omit => "Omit",
            SVMissingValue::Transparent => "Transparent",
            SVMissingValue::Neutral => "Neutral",
        }
    }
}

/// Which matrix axis the rows run along.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVHeatmapOrientation {
    /// Rows run down the vertical axis, columns across the horizontal axis.
    #[default]
    RowsAlongVertical,
    /// Rows run across the horizontal axis, columns down the vertical axis.
    RowsAlongHorizontal,
}

impl SVHeatmapOrientation {
    /// Returns the orientation name as written in tests and observability.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVHeatmapOrientation::RowsAlongVertical => "RowsAlongVertical",
            SVHeatmapOrientation::RowsAlongHorizontal => "RowsAlongHorizontal",
        }
    }
}

/// A borrowed, row-major matrix of values.
///
/// The matrix carries its own shape, so a direct matrix needs no long table.
/// A non-finite value is a missing cell and is drawn according to the
/// configured [`SVMissingValue`] rule.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVHeatmapMatrix<'a> {
    values: &'a [f64],
    number_of_rows: usize,
    number_of_columns: usize,
}

impl<'a> SVHeatmapMatrix<'a> {
    /// Builds a matrix from its row-major values and its shape.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MismatchedLengths`] when the value count is not
    /// `rows * columns`, and [`SVError::InvalidConfiguration`] when a side is
    /// zero while the other is not.
    pub fn new(
        values: &'a [f64],
        number_of_rows: usize,
        number_of_columns: usize,
    ) -> Result<Self, SVError> {
        if number_of_rows == 0 || number_of_columns == 0 {
            if !values.is_empty() {
                return Err(SVError::InvalidConfiguration {
                    message: "a heatmap matrix with an empty side cannot carry values".to_string(),
                });
            }
            return Ok(Self {
                values,
                number_of_rows,
                number_of_columns,
            });
        }
        let expected = number_of_rows * number_of_columns;
        if values.len() != expected {
            return Err(SVError::MismatchedLengths {
                name: "matrix values".to_string(),
                expected,
                actual: values.len(),
            });
        }
        Ok(Self {
            values,
            number_of_rows,
            number_of_columns,
        })
    }

    /// Returns the row-major values.
    pub fn values(&self) -> &'a [f64] {
        self.values
    }

    /// Returns the number of rows.
    pub fn number_of_rows(&self) -> usize {
        self.number_of_rows
    }

    /// Returns the number of columns.
    pub fn number_of_columns(&self) -> usize {
        self.number_of_columns
    }

    /// Returns the value at a position, when the position is inside the shape.
    pub fn value_at(&self, row: usize, column: usize) -> Option<f64> {
        if row >= self.number_of_rows || column >= self.number_of_columns {
            return None;
        }
        self.values
            .get(row * self.number_of_columns + column)
            .copied()
    }
}
