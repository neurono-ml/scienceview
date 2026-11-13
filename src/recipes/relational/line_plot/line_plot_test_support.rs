//! Borrowed test sources for the dense, sparse and streaming representations.

use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;

/// A multi-column in-memory source whose columns may use different
/// representations.
#[derive(Clone, Debug)]
pub(super) struct SVTestSource<'values> {
    columns: Vec<(&'values str, SVColumnRole, SVColumnView<'values>)>,
}

impl<'values> SVTestSource<'values> {
    pub(super) fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    #[must_use]
    pub(super) fn with_column(
        mut self,
        name: &'values str,
        role: SVColumnRole,
        view: SVColumnView<'values>,
    ) -> Self {
        self.columns.push((name, role, view));
        self
    }

    #[must_use]
    pub(super) fn with_dense(
        self,
        name: &'values str,
        role: SVColumnRole,
        values: &'values [f64],
    ) -> Self {
        self.with_column(name, role, SVColumnView::Dense(values))
    }

    #[must_use]
    pub(super) fn with_sparse(
        self,
        name: &'values str,
        role: SVColumnRole,
        indices: &'values [usize],
        values: &'values [f64],
        length: usize,
    ) -> Self {
        let sparse = SVSparseView::new(indices, values, length)
            .expect("the test sparse view is structurally valid");
        self.with_column(name, role, SVColumnView::Sparse(sparse))
    }

    #[must_use]
    pub(super) fn with_streaming(
        self,
        name: &'values str,
        role: SVColumnRole,
        values: &'values [f64],
        sequence: u64,
    ) -> Self {
        self.with_column(
            name,
            role,
            SVColumnView::Streaming(SVStreamingView::new(values, sequence, 0)),
        )
    }
}

impl SVDataSource for SVTestSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(
            self.columns
                .iter()
                .map(|(name, role, _)| SVColumnDescriptor::new(*name, *role))
                .collect(),
        )
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.columns
            .iter()
            .find(|(column_name, _, _)| *column_name == name)
            .map(|(_, _, view)| SVColumnSnapshot::Borrowed(*view))
            .ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })
    }
}

/// Builds a dense source with the required `x` and `y` columns.
pub(super) fn sv_dense_source<'values>(
    x: &'values [f64],
    y: &'values [f64],
) -> SVTestSource<'values> {
    SVTestSource::new()
        .with_dense("x", SVColumnRole::X, x)
        .with_dense("y", SVColumnRole::Y, y)
}
