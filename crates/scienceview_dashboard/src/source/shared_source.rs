//! One shared source registered under a dashboard identifier.

use crate::sv_data::{SVColumnSnapshot, SVDataSource};

/// A data source shared by every panel that binds to its identifier.
///
/// The dashboard never copies the source: it holds a borrowed reference and
/// hands it to each panel, so two cells read the very same underlying storage.
#[derive(Clone, Copy)]
pub struct SVSharedSource<'a> {
    identifier: &'a str,
    source: &'a dyn SVDataSource,
}

impl<'a> SVSharedSource<'a> {
    /// Registers a source under an identifier.
    pub fn new(identifier: &'a str, source: &'a dyn SVDataSource) -> Self {
        Self {
            identifier,
            source,
        }
    }

    /// Returns the dashboard identifier.
    pub fn identifier(&self) -> &'a str {
        self.identifier
    }

    /// Returns the borrowed source.
    pub fn source(&self) -> &'a dyn SVDataSource {
        self.source
    }

    /// Resolves one column of the shared source.
    ///
    /// # Errors
    ///
    /// Propagates the source's own column errors.
    pub fn column(&self, name: &str) -> Result<SVColumnSnapshot<'a>, crate::sv_error::SVError> {
        self.source.column(name)
    }
}

impl std::fmt::Debug for SVSharedSource<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SVSharedSource")
            .field("identifier", &self.identifier)
            .field("columns", &self.source.schema().number_of_columns())
            .finish_non_exhaustive()
    }
}
