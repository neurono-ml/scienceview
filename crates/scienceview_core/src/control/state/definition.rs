//! The data model the reducer mutates: axes, hidden series and the selection.

use std::collections::BTreeSet;

use crate::sv_control::{SVControlAxis, SVControlRange, SVSelectionRegion, SVSeriesReference};
use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;
use crate::sv_scale::{SVDomain, SVScaleTransform};

/// One axis and the interval it currently shows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SVAxisState {
    pub(crate) axis: SVControlAxis,
    pub(crate) transform: SVScaleTransform,
    pub(crate) bounds: SVDomain,
    pub(crate) visible: SVDomain,
}

/// The figure view state controls operate on.
///
/// It holds no figure, no backend and no clock: only the registered axes, the
/// series the user hid and the current selection. The same state serves a
/// figure, a dashboard cell or a test, and every mutation is a pure function of
/// the previous state and the action.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVControlState {
    pub(crate) axes: Vec<SVAxisState>,
    pub(crate) hidden_series: BTreeSet<SVSeriesReference>,
    pub(crate) selection: Option<SVSelectionRegion>,
}

impl SVControlState {
    /// Creates an empty state with no axis and no hidden series.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an axis with an explicit full domain and the linear transform.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the axis is already
    /// registered.
    pub fn register_axis(&mut self, axis: SVControlAxis, domain: SVDomain) -> Result<(), SVError> {
        self.register_axis_with_transform(axis, domain, SVScaleTransform::Linear)
    }

    /// Registers an axis with an explicit full domain and transform.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the axis is already
    /// registered or when the transform has no value on the domain.
    pub fn register_axis_with_transform(
        &mut self,
        axis: SVControlAxis,
        domain: SVDomain,
        transform: SVScaleTransform,
    ) -> Result<(), SVError> {
        if self.axes.iter().any(|state| state.axis == axis) {
            return Err(SVError::InvalidConfiguration {
                message: format!("the {axis} axis is already registered"),
            });
        }
        if !transform.domain_is_defined(domain) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the {transform} transform is not defined on the {axis} domain ({}, {})",
                    domain.lower(),
                    domain.upper()
                ),
            });
        }
        self.axes.push(SVAxisState {
            axis,
            transform,
            bounds: domain,
            visible: domain,
        });
        Ok(())
    }

    /// Registers an axis whose full domain is derived from a column view.
    ///
    /// The representation is resolved exactly once by the domain kernel, so a
    /// dense, sparse and streaming view register the same bounds and sparse
    /// storage is never densified.
    ///
    /// # Errors
    ///
    /// Propagates the domain derivation and registration errors.
    pub fn register_axis_from_column(
        &mut self,
        axis: SVControlAxis,
        view: SVColumnView<'_>,
    ) -> Result<(), SVError> {
        self.register_axis(axis, SVDomain::from_column_view(view)?)
    }

    /// Registers an axis derived from a column view with an explicit transform.
    ///
    /// # Errors
    ///
    /// Propagates the domain derivation and registration errors.
    pub fn register_axis_from_column_with_transform(
        &mut self,
        axis: SVControlAxis,
        view: SVColumnView<'_>,
        transform: SVScaleTransform,
    ) -> Result<(), SVError> {
        self.register_axis_with_transform(axis, SVDomain::from_column_view(view)?, transform)
    }

    /// Returns the full domain of an axis, when it is registered.
    pub fn axis_domain(&self, axis: SVControlAxis) -> Option<SVDomain> {
        self.sv_axis_state(axis).map(|state| state.bounds)
    }

    /// Returns the visible domain of an axis, when it is registered.
    pub fn visible_domain(&self, axis: SVControlAxis) -> Option<SVDomain> {
        self.sv_axis_state(axis).map(|state| state.visible)
    }

    /// Returns the visible range of an axis, when it is registered.
    pub fn visible_range(&self, axis: SVControlAxis) -> Option<SVControlRange> {
        self.visible_domain(axis).map(SVControlRange::from_domain)
    }

    /// Returns the transform of an axis, when it is registered.
    pub fn axis_transform(&self, axis: SVControlAxis) -> Option<SVScaleTransform> {
        self.sv_axis_state(axis).map(|state| state.transform)
    }

    /// Narrows the bounds of an axis and clamps the visible domain into them.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnknownControlAxis`] when the axis is not registered.
    pub fn set_axis_bounds(
        &mut self,
        axis: SVControlAxis,
        bounds: SVDomain,
    ) -> Result<(), SVError> {
        let state = self
            .sv_axis_state_mut(axis)
            .ok_or(SVError::UnknownControlAxis { axis })?;
        state.bounds = bounds;
        state.visible = sv_clamp_domain(state.visible, bounds);
        Ok(())
    }

    /// Sets the visible domain of an axis explicitly.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnknownControlAxis`] when the axis is not registered.
    pub fn set_visible_domain(
        &mut self,
        axis: SVControlAxis,
        domain: SVDomain,
    ) -> Result<(), SVError> {
        let state = self
            .sv_axis_state_mut(axis)
            .ok_or(SVError::UnknownControlAxis { axis })?;
        state.visible = sv_clamp_domain(domain, state.bounds);
        Ok(())
    }

    /// Returns whether a series is visible.
    pub fn is_series_visible(&self, series: &SVSeriesReference) -> bool {
        !self.hidden_series.contains(series)
    }

    /// Returns the current selection, when one exists.
    pub fn selection(&self) -> Option<SVSelectionRegion> {
        self.selection
    }

    pub(crate) fn sv_axis_state(&self, axis: SVControlAxis) -> Option<&SVAxisState> {
        self.axes.iter().find(|state| state.axis == axis)
    }

    pub(crate) fn sv_axis_state_mut(&mut self, axis: SVControlAxis) -> Option<&mut SVAxisState> {
        self.axes.iter_mut().find(|state| state.axis == axis)
    }
}

/// Clamps a domain into bounds without inverting it.
pub(crate) fn sv_clamp_domain(domain: SVDomain, bounds: SVDomain) -> SVDomain {
    let lower = domain.lower().max(bounds.lower()).min(bounds.upper());
    let upper = domain.upper().min(bounds.upper()).max(bounds.lower());
    SVDomain::from_validated(lower, upper)
}
