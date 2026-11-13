//! The integration contract a third-party widget implements.

use crate::sv_error::SVError;

use super::{SVControl, SVControlAction, SVControlDispatch, SVControlOrigin, SVControlState};

/// Anything that can drive a figure with the control vocabulary.
///
/// A built-in [`SVControl`] and a widget from a third-party user-interface
/// framework implement this trait. The widget supplies its name, its enabled
/// state and the action it produces; the trait supplies activation, the origin
/// attribution and the dispatch into the reducer, so both behave identically.
pub trait SVControlSurface {
    /// Returns the widget or control name used as the event origin.
    fn name(&self) -> &str;

    /// Returns whether the surface reacts to activation.
    fn enabled(&self) -> bool;

    /// Returns the action activation would dispatch.
    fn action(&self) -> SVControlAction;

    /// Returns the event origin this surface attributes its actions to.
    fn origin(&self) -> SVControlOrigin {
        SVControlOrigin::ExternalWidget(self.name().to_string())
    }

    /// Returns the action to dispatch, or the typed disabled error.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ControlDisabled`] when the surface is disabled.
    fn activate(&self) -> Result<SVControlAction, SVError> {
        if self.enabled() {
            Ok(self.action())
        } else {
            Err(SVError::ControlDisabled {
                name: self.name().to_string(),
            })
        }
    }

    /// Activates the surface and applies the resulting action.
    ///
    /// # Errors
    ///
    /// Propagates the activation and dispatch errors.
    fn dispatch_into(&self, state: &mut SVControlState) -> Result<SVControlDispatch, SVError> {
        let action = self.activate()?;
        state.dispatch_from(self.origin(), &action)
    }
}

impl SVControl {
    /// Returns the action to dispatch, or the typed disabled error.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::ControlDisabled`] when the control is disabled.
    pub fn activate(&self) -> Result<SVControlAction, SVError> {
        if self.enabled() {
            Ok(self.action())
        } else {
            Err(SVError::ControlDisabled {
                name: self.name().to_string(),
            })
        }
    }
}

impl SVControlSurface for SVControl {
    fn name(&self) -> &str {
        SVControl::name(self)
    }

    fn enabled(&self) -> bool {
        SVControl::enabled(self)
    }

    fn action(&self) -> SVControlAction {
        SVControl::action(self)
    }

    fn origin(&self) -> SVControlOrigin {
        SVControlOrigin::Control(self.name().to_string())
    }
}

/// A widget supplied by an external user-interface framework.
///
/// The adapter stores nothing but a name, an enabled flag and one action, which
/// is the whole surface a toolkit needs to expose. Swapping a built-in control
/// for this adapter changes the event origin and nothing else.
#[derive(Clone, Debug, PartialEq)]
pub struct SVWidgetAdapter {
    name: String,
    action: SVControlAction,
    enabled: bool,
}

impl SVWidgetAdapter {
    /// Builds an enabled widget with one action.
    pub fn new(name: impl Into<String>, action: SVControlAction) -> Self {
        Self {
            name: name.into(),
            action,
            enabled: true,
        }
    }

    /// Returns the widget name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the action the widget dispatches.
    pub fn action(&self) -> SVControlAction {
        self.action.clone()
    }

    /// Returns whether the widget is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables the widget.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Replaces the action the widget dispatches.
    pub fn set_action(&mut self, action: SVControlAction) {
        self.action = action;
    }
}

impl SVControlSurface for SVWidgetAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn action(&self) -> SVControlAction {
        self.action.clone()
    }
}
