//! Control exposure: the semantic action vocabulary from Python.
//!
//! The Python surface dispatches the very same `SVControlAction` values a Rust
//! widget dispatches, through the same reducer, so the two surfaces cannot
//! disagree about what zoom, pan, reset or a series toggle means.

use pyo3::prelude::*;
use scienceview::sv_control::{SVControlAction, SVControlAxis, SVControlState, SVSeriesReference};
use scienceview::sv_error::SVError;
use scienceview::sv_scale::SVDomain;

use super::error::sv_python_error;

/// A control session a Python caller drives directly.
#[pyclass(name = "ControlState")]
#[derive(Clone, Debug, Default)]
pub struct PyControlState {
    state: SVControlState,
}

#[pymethods]
impl PyControlState {
    /// Starts an empty control session.
    #[new]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Registers an axis with its full domain.
    pub(crate) fn register_axis(&mut self, axis: &str, minimum: f64, maximum: f64) -> PyResult<()> {
        let axis = sv_control_axis(axis)?;
        let domain = SVDomain::new(minimum, maximum).map_err(sv_python_error)?;
        self.state
            .register_axis(axis, domain)
            .map_err(sv_python_error)
    }

    /// Dispatches one semantic action and returns the number of events it
    /// produced.
    #[pyo3(signature = (action, *, axis=None, factor=None, anchor=None, fraction=None, series=None, visible=None))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn dispatch(
        &mut self,
        action: &str,
        axis: Option<&str>,
        factor: Option<f64>,
        anchor: Option<f64>,
        fraction: Option<f64>,
        series: Option<&str>,
        visible: Option<bool>,
    ) -> PyResult<usize> {
        let action = sv_control_action(action, axis, factor, anchor, fraction, series, visible)?;
        let dispatch = self.state.dispatch(&action).map_err(sv_python_error)?;
        Ok(dispatch.events().len())
    }

    /// The visible range of an axis as `(minimum, maximum)`, when registered.
    pub(crate) fn visible_range(&self, axis: &str) -> PyResult<Option<(f64, f64)>> {
        let axis = sv_control_axis(axis)?;
        Ok(self
            .state
            .visible_range(axis)
            .map(|range| (range.lower(), range.upper())))
    }
}

fn sv_control_axis(name: &str) -> PyResult<SVControlAxis> {
    match name {
        "horizontal" | "x" => Ok(SVControlAxis::Horizontal),
        "vertical" | "y" => Ok(SVControlAxis::Vertical),
        other => Err(sv_python_error(SVError::InvalidConfiguration {
            message: format!(
                "`{other}` is not a control axis; the accepted names are horizontal and vertical"
            ),
        })),
    }
}

#[allow(clippy::too_many_arguments)]
fn sv_control_action(
    action: &str,
    axis: Option<&str>,
    factor: Option<f64>,
    anchor: Option<f64>,
    fraction: Option<f64>,
    series: Option<&str>,
    visible: Option<bool>,
) -> PyResult<SVControlAction> {
    match action {
        "zoom" => Ok(SVControlAction::Zoom {
            axis: sv_control_axis(axis.unwrap_or("horizontal"))?,
            factor: factor.unwrap_or(0.8),
            anchor: anchor.unwrap_or(0.5),
        }),
        "pan" => Ok(SVControlAction::Pan {
            axis: sv_control_axis(axis.unwrap_or("horizontal"))?,
            fraction: fraction.unwrap_or(0.1),
        }),
        "reset" => Ok(SVControlAction::Reset),
        "series" => {
            let series = series.ok_or_else(|| {
                sv_python_error(SVError::InvalidConfiguration {
                    message: "the `series` action needs the `series` keyword".to_string(),
                })
            })?;
            Ok(SVControlAction::SetSeriesVisibility {
                series: SVSeriesReference::Name(series.to_string()),
                visible: visible.unwrap_or(true),
            })
        }
        "clear_selection" => Ok(SVControlAction::ClearSelection),
        other => Err(sv_python_error(SVError::InvalidConfiguration {
            message: format!(
                "`{other}` is not a bound control action; the bound actions are zoom, pan, reset, series and clear_selection"
            ),
        })),
    }
}
