//! Borrowed test sources for the dense, sparse and streaming representations.

use scienceview_core::sv_data::{
    SVColumnDescriptor, SVColumnRole, SVColumnSnapshot, SVColumnView, SVDataSource, SVSourceSchema,
    SVSparseView, SVStreamingView,
};
use scienceview_core::sv_error::SVError;
#[cfg(feature = "render-svg")]
use scienceview_core::sv_window::{SVRollingWindow, SVWindowReadGuard};

/// One test column, already in the representation to exercise.
#[derive(Clone, Copy, Debug)]
pub(super) enum SVTestColumn<'a> {
    Dense(&'a [f64]),
    Sparse(SVSparseView<'a>),
    Streaming(&'a [f64], u64),
}

impl<'a> SVTestColumn<'a> {
    fn view(&self) -> SVColumnView<'a> {
        match self {
            SVTestColumn::Dense(values) => SVColumnView::Dense(values),
            SVTestColumn::Sparse(sparse) => SVColumnView::Sparse(*sparse),
            SVTestColumn::Streaming(values, sequence) => {
                SVColumnView::Streaming(SVStreamingView::new(values, *sequence, 0))
            }
        }
    }
}

/// A multi-column source whose columns may use different representations.
#[derive(Clone, Debug)]
pub(super) struct SVTestSource<'a> {
    columns: Vec<(&'a str, SVColumnRole, SVTestColumn<'a>)>,
}

impl<'a> SVTestSource<'a> {
    pub(super) fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    #[must_use]
    pub(super) fn with_column(
        mut self,
        name: &'a str,
        role: SVColumnRole,
        column: SVTestColumn<'a>,
    ) -> Self {
        self.columns.push((name, role, column));
        self
    }

    #[must_use]
    pub(super) fn with_dense(self, name: &'a str, role: SVColumnRole, values: &'a [f64]) -> Self {
        self.with_column(name, role, SVTestColumn::Dense(values))
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
        let column = self
            .columns
            .iter()
            .find(|(candidate, _, _)| *candidate == name)
            .map(|(_, _, column)| column)
            .ok_or_else(|| SVError::ColumnNotFound {
                name: name.to_string(),
            })?;
        Ok(SVColumnSnapshot::Borrowed(column.view()))
    }
}

/// A source wrapper that counts every column resolution.
#[derive(Clone, Debug)]
pub(super) struct SVCountingSource<'a> {
    inner: SVTestSource<'a>,
    calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl<'a> SVCountingSource<'a> {
    pub(super) fn new(inner: SVTestSource<'a>) -> Self {
        Self {
            inner,
            calls: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub(super) fn number_of_calls(&self) -> usize {
        self.calls.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl SVDataSource for SVCountingSource<'_> {
    fn schema(&self) -> SVSourceSchema<'_> {
        self.inner.schema()
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        self.calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.column(name)
    }
}

/// A live source whose two columns share a bounded rolling window each.
#[cfg(feature = "render-svg")]
#[derive(Debug)]
pub(super) struct SVLiveSource {
    horizontal: std::sync::Mutex<SVRollingWindow>,
    vertical: std::sync::Mutex<SVRollingWindow>,
}

#[cfg(feature = "render-svg")]
impl SVLiveSource {
    pub(super) fn new(horizontal: &[f64], vertical: &[f64]) -> Self {
        Self {
            horizontal: std::sync::Mutex::new(sv_window(horizontal)),
            vertical: std::sync::Mutex::new(sv_window(vertical)),
        }
    }

    /// Appends one sample to both live columns.
    pub(super) fn append(&self, horizontal: f64, vertical: f64) {
        sv_lock(&self.horizontal).push(horizontal);
        sv_lock(&self.vertical).push(vertical);
    }
}

#[cfg(feature = "render-svg")]
fn sv_window(values: &[f64]) -> SVRollingWindow {
    let mut window = SVRollingWindow::with_capacity(64).expect("the capacity is positive");
    window.extend(values.iter().copied());
    window
}

#[cfg(feature = "render-svg")]
fn sv_lock(
    window: &std::sync::Mutex<SVRollingWindow>,
) -> std::sync::MutexGuard<'_, SVRollingWindow> {
    window
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(feature = "render-svg")]
impl SVDataSource for SVLiveSource {
    fn schema(&self) -> SVSourceSchema<'_> {
        SVSourceSchema::owned(vec![
            SVColumnDescriptor::new("x", SVColumnRole::X),
            SVColumnDescriptor::new("y", SVColumnRole::Y),
        ])
    }

    fn column(&self, name: &str) -> Result<SVColumnSnapshot<'_>, SVError> {
        match name {
            "x" => Ok(SVColumnSnapshot::Window(sv_read(&self.horizontal))),
            "y" => Ok(SVColumnSnapshot::Window(sv_read(&self.vertical))),
            other => Err(SVError::ColumnNotFound {
                name: other.to_string(),
            }),
        }
    }
}

#[cfg(feature = "render-svg")]
fn sv_read(window: &std::sync::Mutex<SVRollingWindow>) -> SVWindowReadGuard<'_> {
    SVWindowReadGuard::new(sv_lock(window))
}
