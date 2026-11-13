//! The gesture table and its resolution rules.

use crate::sv_control::SVControlAction;

use super::input::{SVGesture, SVGestureKind};
/// One row of the gesture table.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGestureBinding {
    gesture: SVGestureKind,
    action: SVControlAction,
}

impl SVGestureBinding {
    /// Pairs a gesture class with the action it resolves to.
    pub fn new(gesture: SVGestureKind, action: SVControlAction) -> Self {
        Self { gesture, action }
    }

    /// Returns the gesture class.
    pub fn gesture(&self) -> SVGestureKind {
        self.gesture
    }

    /// Returns the bound action.
    pub fn action(&self) -> &SVControlAction {
        &self.action
    }
}

/// A table from gesture to action.
#[derive(Clone, Debug, PartialEq)]
pub struct SVGestureMap {
    bindings: Vec<SVGestureBinding>,
}

impl SVGestureMap {
    /// Builds a map from explicit bindings.
    pub fn new(bindings: impl IntoIterator<Item = SVGestureBinding>) -> Self {
        Self {
            bindings: bindings.into_iter().collect(),
        }
    }

    /// Returns every binding, in table order.
    pub fn bindings(&self) -> &[SVGestureBinding] {
        &self.bindings
    }

    /// Binds a gesture class, replacing an existing binding for it.
    pub fn bind(&mut self, gesture: SVGestureKind, action: SVControlAction) {
        if let Some(binding) = self
            .bindings
            .iter_mut()
            .find(|binding| binding.gesture == gesture)
        {
            binding.action = action;
        } else {
            self.bindings.push(SVGestureBinding::new(gesture, action));
        }
    }

    /// Resolves a gesture into exactly one action, or `None`.
    ///
    /// A wheel keeps the direction of its delta by inverting the bound zoom
    /// factor when the delta is negative. A drag replaces the bound pan
    /// fraction with the dragged fraction, so the reducer sees the real
    /// distance instead of a fixed step.
    pub fn resolve(&self, gesture: &SVGesture) -> Option<SVControlAction> {
        match gesture {
            SVGesture::Key(key) => self
                .bindings
                .iter()
                .find(|binding| binding.gesture == SVGestureKind::Key(*key))
                .map(|binding| binding.action.clone()),
            SVGesture::Wheel { axis, delta } => {
                if !delta.is_finite() || *delta == 0.0 {
                    return None;
                }
                let binding = self
                    .bindings
                    .iter()
                    .find(|binding| binding.gesture == SVGestureKind::Wheel(*axis))?;
                match &binding.action {
                    SVControlAction::Zoom {
                        axis,
                        factor,
                        anchor,
                    } => {
                        let factor = if *delta > 0.0 {
                            *factor
                        } else if *factor > 0.0 {
                            1.0 / factor
                        } else {
                            return None;
                        };
                        Some(SVControlAction::Zoom {
                            axis: *axis,
                            factor,
                            anchor: *anchor,
                        })
                    }
                    action => Some(action.clone()),
                }
            }
            SVGesture::Drag {
                button,
                axis,
                fraction,
            } => {
                if !fraction.is_finite() {
                    return None;
                }
                let binding = self
                    .bindings
                    .iter()
                    .find(|binding| binding.gesture == SVGestureKind::Drag(*button, *axis))?;
                match &binding.action {
                    SVControlAction::Pan { axis, .. } => Some(SVControlAction::Pan {
                        axis: *axis,
                        fraction: *fraction,
                    }),
                    action => Some(action.clone()),
                }
            }
        }
    }
}
