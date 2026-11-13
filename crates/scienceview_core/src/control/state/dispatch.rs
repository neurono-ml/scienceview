//! Dispatching an action into the state and emitting the payload vocabulary.

use crate::sv_animation::SVAnimationAction;
use crate::sv_error::SVError;

use super::SVControlState;
use crate::sv_control::{
    SVControlAction, SVControlChange, SVControlEvent, SVControlOrigin, SVCustomPayload,
};

/// What one dispatch changed.
///
/// A state change produces one event per changed value. A playback action
/// produces no state event: it is returned as [`SVControlDispatch::animation_action`]
/// so the session can apply it to the timeline it owns and emit the playback
/// payload with the real before and after values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVControlDispatch {
    events: Vec<SVControlEvent>,
    animation_action: Option<SVAnimationAction>,
}

impl SVControlDispatch {
    /// Returns the events the action produced.
    pub fn events(&self) -> &[SVControlEvent] {
        &self.events
    }

    /// Returns the playback command the action delegates, when it delegates one.
    pub fn animation_action(&self) -> Option<SVAnimationAction> {
        self.animation_action
    }

    /// Returns `true` when the action changed nothing and delegated nothing.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty() && self.animation_action.is_none()
    }
}

impl SVControlState {
    /// Applies one action and returns the events it produced.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::UnknownControlAxis`] when the action targets an axis
    /// that was never registered, and [`SVError::InvalidConfiguration`] for a
    /// non-finite or out-of-range numeric parameter.
    pub fn dispatch(&mut self, action: &SVControlAction) -> Result<SVControlDispatch, SVError> {
        self.dispatch_from(SVControlOrigin::Programmatic, action)
    }

    /// Applies one action attributed to an origin.
    ///
    /// # Errors
    ///
    /// Propagates the dispatch errors of [`SVControlState::dispatch`].
    pub fn dispatch_from(
        &mut self,
        origin: SVControlOrigin,
        action: &SVControlAction,
    ) -> Result<SVControlDispatch, SVError> {
        let events = match action {
            SVControlAction::Zoom {
                axis,
                factor,
                anchor,
            } => self.sv_dispatch_zoom(origin, *axis, *factor, *anchor)?,
            SVControlAction::Pan { axis, fraction } => {
                self.sv_dispatch_pan(origin, *axis, *fraction)?
            }
            SVControlAction::Reset => self.sv_dispatch_reset(origin),
            SVControlAction::ToggleSeries(series) => {
                let visible = self.is_series_visible(series);
                self.sv_dispatch_series(origin, series, !visible)
            }
            SVControlAction::SetSeriesVisibility { series, visible } => {
                self.sv_dispatch_series(origin, series, *visible)
            }
            SVControlAction::SetRange { axis, range } => {
                self.sv_dispatch_range(origin, *axis, *range)?
            }
            SVControlAction::SelectRegion(region) => {
                self.sv_dispatch_selection(origin, Some(*region))
            }
            SVControlAction::ClearSelection => self.sv_dispatch_selection(origin, None),
            SVControlAction::SetPlayback(action) => {
                return Ok(SVControlDispatch {
                    events: Vec::new(),
                    animation_action: Some(*action),
                });
            }
            SVControlAction::Custom { name, payload } => vec![SVControlEvent::new(
                origin,
                SVControlChange::Custom(SVCustomPayload {
                    name: name.clone(),
                    payload: payload.clone(),
                }),
            )],
        };

        Ok(SVControlDispatch {
            events,
            animation_action: None,
        })
    }
}
