//! Domain actions: zoom, pan, reset and range.

use crate::sv_error::SVError;

use super::SVControlState;
use super::geometry::{sv_pan_domain, sv_zoom_domain};
use crate::sv_control::{
    SVAxisRange, SVControlAxis, SVControlChange, SVControlEvent, SVControlOrigin, SVControlRange,
    SVRangeChange, SVSelectionChange, SVViewReset,
};

impl SVControlState {
    pub(super) fn sv_dispatch_zoom(
        &mut self,
        origin: SVControlOrigin,
        axis: SVControlAxis,
        factor: f64,
        anchor: f64,
    ) -> Result<Vec<SVControlEvent>, SVError> {
        if !factor.is_finite() || factor <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "a zoom factor must be finite and greater than zero, received {factor}"
                ),
            });
        }
        if !anchor.is_finite() || !(0.0..=1.0).contains(&anchor) {
            return Err(SVError::InvalidConfiguration {
                message: format!("a zoom anchor must lie in 0..=1, received {anchor}"),
            });
        }
        let state = self
            .sv_axis_state_mut(axis)
            .ok_or(SVError::UnknownControlAxis { axis })?;
        let previous = state.visible;
        let Some(current) = sv_zoom_domain(state, factor, anchor) else {
            return Ok(Vec::new());
        };
        state.visible = current;
        Ok(vec![SVControlEvent::new(
            origin,
            SVControlChange::Range(SVRangeChange {
                axis,
                previous: SVControlRange::from_domain(previous),
                current: SVControlRange::from_domain(current),
            }),
        )])
    }

    pub(super) fn sv_dispatch_pan(
        &mut self,
        origin: SVControlOrigin,
        axis: SVControlAxis,
        fraction: f64,
    ) -> Result<Vec<SVControlEvent>, SVError> {
        if !fraction.is_finite() {
            return Err(SVError::InvalidConfiguration {
                message: format!("a pan fraction must be finite, received {fraction}"),
            });
        }
        let state = self
            .sv_axis_state_mut(axis)
            .ok_or(SVError::UnknownControlAxis { axis })?;
        let previous = state.visible;
        let Some(current) = sv_pan_domain(state, fraction) else {
            return Ok(Vec::new());
        };
        state.visible = current;
        Ok(vec![SVControlEvent::new(
            origin,
            SVControlChange::Range(SVRangeChange {
                axis,
                previous: SVControlRange::from_domain(previous),
                current: SVControlRange::from_domain(current),
            }),
        )])
    }

    pub(super) fn sv_dispatch_reset(&mut self, origin: SVControlOrigin) -> Vec<SVControlEvent> {
        let previous: Vec<SVAxisRange> = self
            .axes
            .iter()
            .map(|state| SVAxisRange::new(state.axis, SVControlRange::from_domain(state.visible)))
            .collect();
        for state in &mut self.axes {
            state.visible = state.bounds;
        }
        let current: Vec<SVAxisRange> = self
            .axes
            .iter()
            .map(|state| SVAxisRange::new(state.axis, SVControlRange::from_domain(state.visible)))
            .collect();

        let mut events = Vec::new();
        if previous != current {
            events.push(SVControlEvent::new(
                origin.clone(),
                SVControlChange::Reset(SVViewReset { previous, current }),
            ));
        }
        if let Some(previous) = self.selection.take() {
            events.push(SVControlEvent::new(
                origin,
                SVControlChange::Selection(SVSelectionChange {
                    previous: Some(previous),
                    current: None,
                }),
            ));
        }
        events
    }
}
