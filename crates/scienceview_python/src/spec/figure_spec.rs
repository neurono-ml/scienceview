//! The declarative figure specification the Python surface fills in.

use scienceview::sv_backend::SVBackendKind;
use scienceview::sv_figure::SVExecutionMode;

use super::data_source::SVPythonData;
use super::options::SVPythonPlotOptions;
use super::plot_kind::SVPythonPlotKind;

/// Capabilities a caller demands from the selected backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVPythonRequirements {
    /// The backend must deliver events.
    pub events: bool,
    /// The backend must host a scripting engine.
    pub scripting: bool,
    /// The backend must accept streaming data.
    pub streaming: bool,
    /// The backend must update the figure in place.
    pub in_place_update: bool,
}

/// Everything a Python caller configured, before any Rust builder is touched.
///
/// The specification mirrors the Rust builder surface: the same data, the same
/// required backend selection, the same execution mode and the same optional
/// requirements. Conversion is a pure function of this value, which is what
/// makes the binding testable without an interpreter.
#[derive(Clone, Debug)]
pub struct SVPythonFigureSpec {
    /// The kind of plot to build.
    pub kind: SVPythonPlotKind,
    /// The data source, in one of the three representations.
    pub data: SVPythonData,
    /// The explicit backend selection; `None` is rejected at conversion time.
    pub backend: Option<SVBackendKind>,
    /// The execution mode; defaults to `Automatic`, like every Rust builder.
    pub execution_mode: SVExecutionMode,
    /// The keyword options of the kind.
    pub options: SVPythonPlotOptions,
    /// The capabilities the figure demands.
    pub requirements: SVPythonRequirements,
}

impl SVPythonFigureSpec {
    /// Starts a specification for `kind` over `data`.
    pub fn new(kind: SVPythonPlotKind, data: SVPythonData) -> Self {
        Self {
            kind,
            data,
            backend: None,
            execution_mode: SVExecutionMode::default(),
            options: SVPythonPlotOptions::default(),
            requirements: SVPythonRequirements::default(),
        }
    }

    /// Selects the backend kind explicitly.
    pub fn with_backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend = Some(backend_kind);
        self
    }

    /// Selects the execution mode.
    pub fn with_execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Attaches the keyword options.
    pub fn with_options(mut self, options: SVPythonPlotOptions) -> Self {
        self.options = options;
        self
    }

    /// Declares the requirements the backend must satisfy.
    pub fn with_requirements(mut self, requirements: SVPythonRequirements) -> Self {
        self.requirements = requirements;
        self
    }

    /// Whether the data is a live stream.
    pub fn is_streaming(&self) -> bool {
        self.data.is_streaming()
    }
}
