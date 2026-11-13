//! The backend typestate transition.

use std::marker::PhantomData;

use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_figure::{SVWithBackend, SVWithoutBackend};

use super::builder_definition::SVDensityPlotBuilder;

impl<'values> SVDensityPlotBuilder<'values, SVWithoutBackend> {
    /// Selects the rendering backend.
    ///
    /// Required: there is no automatic backend, so the rendering form is
    /// always a deliberate choice.
    #[must_use]
    pub fn backend(
        self,
        backend_kind: SVBackendKind,
    ) -> SVDensityPlotBuilder<'values, SVWithBackend> {
        SVDensityPlotBuilder {
            data_source: self.data_source,
            value: self.value,
            series: self.series,
            bandwidth: self.bandwidth,
            bandwidth_adjustment: self.bandwidth_adjustment,
            kernel: self.kernel,
            grid_count: self.grid_count,
            cut: self.cut,
            fill: self.fill,
            line_width: self.line_width,
            normalisation: self.normalisation,
            missing_value_policy: self.missing_value_policy,
            theme: self.theme,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
            backend_kind: Some(backend_kind),
            execution_mode: self.execution_mode,
            states: PhantomData,
        }
    }
}
