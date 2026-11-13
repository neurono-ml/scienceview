//! The unified data source the binding hands to the Rust builders.

use scienceview::sv_data::{SVColumnSnapshot, SVDataSource, SVRepresentation, SVSourceSchema};
use scienceview::sv_error::SVError;
use std::sync::Arc;

use super::data::SVPythonTable;
use super::stream::SVPythonStream;

/// The data a Python caller attached to a figure.
///
/// Exactly one representation reaches the builder, and the builder resolves it
/// once per operation, so the binding adds no second dispatch.
#[derive(Clone, Debug)]
pub enum SVPythonData {
    /// A static dense or sparse table.
    Table(SVPythonTable),
    /// A live bounded stream shared with its producer.
    Stream(Arc<SVPythonStream>),
}

impl SVPythonData {
    /// A dense, single-column table named `value`.
    pub fn dense(values: Vec<f64>) -> Result<Self, SVError> {
        Ok(Self::Table(SVPythonTable::new(vec![
            super::data::SVPythonColumn::dense(
                "value",
                scienceview::sv_data::SVColumnRole::Y,
                values,
            ),
        ])?))
    }

    /// The logical number of values the primary column holds.
    pub fn length(&self) -> usize {
        match self {
            Self::Table(table) => table.length(),
            Self::Stream(stream) => stream.length(),
        }
    }

    /// The representation of the primary column.
    pub fn representation(&self) -> SVRepresentation {
        match self {
            Self::Table(table) => table
                .primary_column()
                .map_or(SVRepresentation::Dense, |column| {
                    column.values().representation()
                }),
            Self::Stream(_) => SVRepresentation::Streaming,
        }
    }

    /// Resolves the primary column: the stream value or the table's `Y` column.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] when a table declares no column
    /// and the column lookup error of the representation in use.
    pub fn primary_column(&self) -> Result<SVColumnSnapshot<'_>, SVError> {
        match self {
            Self::Table(table) => {
                let name = table
                    .primary_column()
                    .map(super::data::SVPythonColumn::name)
                    .ok_or(SVError::MissingDataSource)?;
                table.column(name)
            }
            Self::Stream(stream) => stream.column("value"),
        }
    }

    /// Pushes samples into a live stream.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnsupportedRepresentation`] when the data is static.
    pub fn push(&self, samples: impl IntoIterator<Item = f64>) -> Result<usize, SVError> {
        match self {
            Self::Stream(stream) => Ok(stream.push(samples)),
            Self::Table(_) => Err(SVError::UnsupportedRepresentation {
                operation: "push",
                representation: SVRepresentation::Dense,
                alternative: "build a `StreamingColumn` and push samples into it",
            }),
        }
    }

    /// Whether this data accepts pushed samples.
    pub fn is_streaming(&self) -> bool {
        matches!(self, Self::Stream(_))
    }
}

impl SVDataSource for SVPythonData {
    fn schema(&self) -> SVSourceSchema<'_> {
        match self {
            Self::Table(table) => table.schema(),
            Self::Stream(stream) => stream.schema(),
        }
    }

    fn reactivity(&self) -> scienceview::sv_data::SVReactivity {
        match self {
            Self::Table(table) => table.reactivity(),
            Self::Stream(stream) => stream.reactivity(),
        }
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match self {
            Self::Table(table) => table.column(name),
            Self::Stream(stream) => stream.column(name),
        }
    }
}
