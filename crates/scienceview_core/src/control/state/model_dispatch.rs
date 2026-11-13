//! Model actions: series visibility, selection and custom payloads.

use super::SVControlState;
use crate::sv_control::{
    SVControlAxis, SVControlChange, SVControlEvent, SVControlOrigin, SVControlRange, SVRangeChange,
    SVSelectionChange, SVSeriesVisibilityChange,
};
use crate::sv_error::SVError;

impl SVControlState {
    pub(super) fn sv_dispatch_series(
        &mut self,
        origin: SVControlOrigin,
        series: &crate::sv_control::SVSeriesReference,
        visible: bool,
    ) -> Vec<SVControlEvent> {
        let previous = self.is_series_visible(series);
        if previous == visible {
            return Vec::new();
        }
        if visible {
            self.hidden_series.remove(series);
        } else {
            self.hidden_series.insert(series.clone());
        }
        vec![SVControlEvent::new(
            origin,
            SVControlChange::SeriesVisibility(SVSeriesVisibilityChange {
                series: series.clone(),
                previous,
                current: visible,
            }),
        )]
    }

    pub(super) fn sv_dispatch_range(
        &mut self,
        origin: SVControlOrigin,
        axis: SVControlAxis,
        range: SVControlRange,
    ) -> Result<Vec<SVControlEvent>, SVError> {
        let state = self
            .sv_axis_state_mut(axis)
            .ok_or(SVError::UnknownControlAxis { axis })?;
        let bounds = SVControlRange::from_domain(state.bounds);
        let current = range.clamp_into(bounds);
        let previous = SVControlRange::from_domain(state.visible);
        if previous == current {
            return Ok(Vec::new());
        }
        state.visible = current.to_domain();
        Ok(vec![SVControlEvent::new(
            origin,
            SVControlChange::Range(SVRangeChange {
                axis,
                previous,
                current,
            }),
        )])
    }

    pub(super) fn sv_dispatch_selection(
        &mut self,
        origin: SVControlOrigin,
        selection: Option<crate::sv_control::SVSelectionRegion>,
    ) -> Vec<SVControlEvent> {
        let previous = self.selection;
        if previous == selection {
            return Vec::new();
        }
        self.selection = selection;
        vec![SVControlEvent::new(
            origin,
            SVControlChange::Selection(SVSelectionChange {
                previous,
                current: selection,
            }),
        )]
    }
}
