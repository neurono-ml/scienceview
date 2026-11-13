//! The backend typestate transition.

use std::marker::PhantomData;

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::{SVWithBackend, SVWithoutBackend};

use super::builder_definition::SVAreaPlotBuilder;

impl<'values> SVAreaPlotBuilder<'values, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is
    /// always a deliberate choice.
    #[must_use]
    pub fn backend(self, backend_kind: SVBackendKind) -> SVAreaPlotBuilder<'values, SVWithBackend> {
        SVAreaPlotBuilder {
            data_source: self.data_source,
            x: self.x,
            y: self.y,
            series: self.series,
            stack: self.stack,
            baseline: self.baseline,
            opacity: self.opacity,
            line: self.line,
            interpolation: self.interpolation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            x_scale_kind: self.x_scale_kind,
            temporal_unit: self.temporal_unit,
            viewport: self.viewport,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            states: PhantomData,
        }
    }
}
