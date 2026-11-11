//! The figure builder and its state machine.
//!
//! ```compile_fail
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_figure::SVDataBinding;
//! use scienceview_core::sv_figure::SVFigureBuilder;
//!
//! let values = [1.0, 2.0];
//! // Missing backend: `build` does not exist on this state.
//! let figure = SVFigureBuilder::new()
//!     .data_source(SVDataBinding::dense(&values))
//!     .build();
//! # let _ = figure;
//! ```
//!
//! ```compile_fail
//! use scienceview_core::sv_backend::SVBackendKind;
//! use scienceview_core::sv_figure::SVFigureBuilder;
//!
//! // Missing data source: `build` does not exist on this state either.
//! let figure = SVFigureBuilder::new()
//!     .backend(SVBackendKind::Svg)
//!     .build();
//! # let _ = figure;
//! ```

use super::SVDataBinding;
use super::SVExecutionMode;
use super::SVFigure;
use super::builder_state::SVWithBackend;
use super::builder_state::SVWithDataSource;
use super::builder_state::SVWithoutBackend;
use super::builder_state::SVWithoutDataSource;
use crate::sv_backend::SVBackendKind;
use crate::sv_backend::SVBackendRegistry;
use crate::sv_backend::SVBackendRequirements;
use crate::sv_backend::sv_backend_registry;
use crate::sv_backend::sv_verify_backend_requirements;
use crate::sv_error::SVError;
use std::marker::PhantomData;
use std::sync::Arc;

/// Builds an [`SVFigure`].
///
/// The two required steps are encoded in the type: the builder starts in
/// `SVWithoutDataSource`/`SVWithoutBackend` and `build` exists only on
/// `SVWithDataSource`/`SVWithBackend`, so a figure cannot be built without
/// both. The order of the two calls is free, and every optional setting can be
/// applied at any stage, including before the required ones.
pub struct SVFigureBuilder<'values, DataSourceState, BackendState> {
    data: Option<SVDataBinding<'values>>,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
    requirements: SVBackendRequirements,
    registry: Option<Arc<SVBackendRegistry>>,
    title: Option<&'values str>,
    states: PhantomData<(DataSourceState, BackendState)>,
}

impl SVFigureBuilder<'_, SVWithoutDataSource, SVWithoutBackend> {
    /// Starts a figure with no data source and no backend.
    pub fn new() -> Self {
        Self {
            data: None,
            backend_kind: None,
            execution_mode: SVExecutionMode::default(),
            requirements: SVBackendRequirements::default(),
            registry: None,
            title: None,
            states: PhantomData,
        }
    }
}

impl Default for SVFigureBuilder<'_, SVWithoutDataSource, SVWithoutBackend> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'values, BackendState> SVFigureBuilder<'values, SVWithoutDataSource, BackendState> {
    /// Attaches the figure's data.
    pub fn data_source(
        self,
        data: SVDataBinding<'values>,
    ) -> SVFigureBuilder<'values, SVWithDataSource, BackendState> {
        SVFigureBuilder {
            data: Some(data),
            backend_kind: self.backend_kind,
            execution_mode: self.execution_mode,
            requirements: self.requirements,
            registry: self.registry,
            title: self.title,
            states: PhantomData,
        }
    }
}

impl<'values, DataSourceState> SVFigureBuilder<'values, DataSourceState, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is
    /// always a deliberate choice.
    pub fn backend(
        self,
        backend_kind: SVBackendKind,
    ) -> SVFigureBuilder<'values, DataSourceState, SVWithBackend> {
        SVFigureBuilder {
            data: self.data,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            requirements: self.requirements,
            registry: self.registry,
            title: self.title,
            states: PhantomData,
        }
    }
}

impl<'values, DataSourceState, BackendState>
    SVFigureBuilder<'values, DataSourceState, BackendState>
{
    /// Selects the execution mode. Defaults to `Automatic`.
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Sets an optional figure title.
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Requires the backend to deliver events.
    pub fn requires_events(mut self) -> Self {
        self.requirements = self.requirements.requiring_events();
        self
    }

    /// Requires the backend to host a scripting engine.
    pub fn requires_scripting(mut self) -> Self {
        self.requirements = self.requirements.requiring_scripting();
        self
    }

    /// Requires the backend to accept streaming data.
    pub fn requires_streaming(mut self) -> Self {
        self.requirements = self.requirements.requiring_streaming();
        self
    }

    /// Requires the backend to update the figure in place.
    pub fn requires_in_place_update(mut self) -> Self {
        self.requirements = self.requirements.requiring_in_place_update();
        self
    }

    /// Substitutes the backend registry.
    ///
    /// Defaults to the process-wide registry backend crates register into.
    /// Tests use their own registry so they never touch global state.
    pub fn backend_registry(mut self, registry: Arc<SVBackendRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }
}

impl<'values> SVFigureBuilder<'values, SVWithDataSource, SVWithBackend> {
    /// Validates the configuration and builds the figure.
    ///
    /// The capability check runs here, against the static matrix; no backend is
    /// resolved. An unsupported combination — scripting on an export-only
    /// backend, streaming on a backend that cannot accept it — is an error now,
    /// not a callback that silently never fires.
    pub fn build(self) -> Result<SVFigure<'values>, SVError> {
        let data = self.data.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;

        let requirements = if data.representation() == super::SVDataRepresentation::Streaming {
            self.requirements.requiring_streaming()
        } else {
            self.requirements
        };
        sv_verify_backend_requirements(backend_kind, requirements)?;

        let registry = self.registry.unwrap_or_else(sv_backend_registry);

        Ok(SVFigure::from_configuration(
            backend_kind,
            self.execution_mode,
            data,
            self.title,
            registry,
        ))
    }
}
