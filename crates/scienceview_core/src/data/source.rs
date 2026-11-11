//! The open ingestion trait and the metadata a source declares.

use std::borrow::Cow;

use crate::data::{SVColumnView, SVDataError, SVRepresentation, SVStreamingView};
use crate::window::SVWindowReadGuard;

/// What a column means to an artist.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVColumnRole {
    /// The independent position along the horizontal axis.
    X,
    /// The dependent position along the vertical axis.
    Y,
    /// A column that groups other columns.
    Grouping,
    /// A column that carries labels.
    Label,
    /// A column that weighs other columns.
    Weight,
}

/// How changes reach a source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SVReactivity {
    /// The data never changes once the source is built.
    #[default]
    Static,
    /// Changes are pushed by the owner of the source.
    Observed,
    /// Changes arrive from an event stream.
    EventDriven,
    /// The data changes only when the owner explicitly pushes an update.
    Manual,
}

/// The name and the role of one column of a source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVColumnDescriptor<'a> {
    name: Cow<'a, str>,
    role: SVColumnRole,
}

impl<'a> SVColumnDescriptor<'a> {
    /// Builds a descriptor from a name and a role.
    pub fn new(name: impl Into<Cow<'a, str>>, role: SVColumnRole) -> Self {
        Self {
            name: name.into(),
            role,
        }
    }

    /// Returns the column name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the role of the column.
    pub fn role(&self) -> SVColumnRole {
        self.role
    }
}

/// The columns a source exposes, with their names and roles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SVSourceSchema<'a> {
    columns: Cow<'a, [SVColumnDescriptor<'a>]>,
}

impl<'a> SVSourceSchema<'a> {
    /// Borrows the descriptors a source already owns.
    pub fn from_columns(columns: &'a [SVColumnDescriptor<'a>]) -> Self {
        Self {
            columns: Cow::Borrowed(columns),
        }
    }

    /// Takes ownership of freshly built descriptors.
    pub fn owned(columns: Vec<SVColumnDescriptor<'a>>) -> Self {
        Self {
            columns: Cow::Owned(columns),
        }
    }

    /// Returns the descriptors of every column.
    pub fn columns(&self) -> &[SVColumnDescriptor<'a>] {
        &self.columns
    }

    /// Returns the number of columns.
    pub fn number_of_columns(&self) -> usize {
        self.columns.len()
    }

    /// Returns `true` when the source declares no column.
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Returns the descriptor of a named column.
    pub fn column(&self, name: &str) -> Option<&SVColumnDescriptor<'a>> {
        self.columns
            .iter()
            .find(|descriptor| descriptor.name() == name)
    }
}

/// A resolved read over one column of a source.
///
/// Static representations borrow directly from the source. A live source keeps
/// its window behind a mutex, so its read owns the exclusive guard and hands
/// out a borrowed snapshot while it is alive. Either way the read is
/// non-blocking and the data is borrowed, never copied into an owned array.
#[derive(Debug)]
pub enum SVColumnSnapshot<'a> {
    /// A dense or sparse view borrowed from the source.
    Borrowed(SVColumnView<'a>),
    /// An exclusive, non-blocking read of a shared rolling window.
    Window(SVWindowReadGuard<'a>),
}

impl SVColumnSnapshot<'_> {
    /// Resolves the representation once, for dispatch and observability.
    pub fn representation(&mut self) -> SVRepresentation {
        match self {
            SVColumnSnapshot::Borrowed(view) => view.representation(),
            SVColumnSnapshot::Window(_) => SVRepresentation::Streaming,
        }
    }

    /// Borrows the column as one of the three representations.
    pub fn column_view(&mut self) -> SVColumnView<'_> {
        match self {
            SVColumnSnapshot::Borrowed(view) => *view,
            SVColumnSnapshot::Window(window) => {
                let sequence = window.sequence();
                let dropped_samples = window.dropped_samples();
                let values = window.values();
                SVColumnView::Streaming(SVStreamingView::new(values, sequence, dropped_samples))
            }
        }
    }

    /// Dispatches on the representation exactly once, guard included.
    pub fn dispatch<Result>(
        &mut self,
        dense: impl FnOnce(&[f64]) -> Result,
        sparse: impl FnOnce(&crate::data::SVSparseView<'_>) -> Result,
        streaming: impl FnOnce(&SVStreamingView<'_>) -> Result,
    ) -> Result {
        match self {
            SVColumnSnapshot::Borrowed(view) => view.dispatch(dense, sparse, streaming),
            SVColumnSnapshot::Window(window) => {
                let sequence = window.sequence();
                let dropped_samples = window.dropped_samples();
                let values = window.values();
                let streaming_view = SVStreamingView::new(values, sequence, dropped_samples);
                streaming(&streaming_view)
            }
        }
    }
}

/// The open ingestion contract every data source implements.
///
/// A third-party crate implements this trait — there is no privileged path and
/// no core modification needed. Built-in adapters implement the very same
/// trait, and the representation is resolved once per operation through
/// [`SVColumnSnapshot`].
pub trait SVDataSource: Send + Sync {
    /// Returns the columns of the source, with their names and roles.
    fn schema(&self) -> SVSourceSchema<'_>;

    /// Returns how changes reach the source. Static unless declared otherwise.
    fn reactivity(&self) -> SVReactivity {
        SVReactivity::Static
    }

    /// Resolves one column into a snapshot, without blocking on new data.
    ///
    /// # Errors
    ///
    /// Returns [`SVDataError::ColumnNotFound`] when the source has no column of
    /// that name, and the validation failures of the representation in use.
    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVDataError>;
}

#[cfg(test)]
#[path = "source_tests.rs"]
mod source_tests;
