//! Keyframes: one timestamped delta of model properties.

use crate::sv_error::SVError;
use crate::sv_mark::SVMarkProperty;

use super::SVDuration;

/// One property change carried by a keyframe delta.
///
/// The property belongs to the table [`sv_mark_property_table`] exports, and
/// the value is the numeric channel the transition kernel interpolates:
/// positions, sizes, opacity, font size and the colour blend factor. A
/// discrete property — the shape, the dash pattern, the font family — is
/// swapped at the end of its segment instead of tweened.
///
/// [`sv_mark_property_table`]: crate::sv_mark::sv_mark_property_table
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVKeyframeChange {
    property: SVMarkProperty,
    value: f64,
}

impl SVKeyframeChange {
    /// Builds a property change.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::NonFiniteAnimationValue`] when `value` is not
    /// finite.
    pub fn new(property: SVMarkProperty, value: f64) -> Result<Self, SVError> {
        if !value.is_finite() {
            return Err(SVError::NonFiniteAnimationValue {
                property,
                position: 0,
            });
        }
        Ok(Self { property, value })
    }

    /// Returns the property this change sets.
    pub fn property(&self) -> SVMarkProperty {
        self.property
    }

    /// Returns the value this change sets.
    pub fn value(&self) -> f64 {
        self.value
    }

    pub(crate) fn sv_unchecked(property: SVMarkProperty, value: f64) -> Self {
        Self { property, value }
    }
}

/// A keyframe's state, expressed as a delta over the model's property
/// vocabulary.
///
/// The delta is the unit `model-events` serializes: the timeline keeps no
/// parallel state representation, it keeps an ordered list of deltas. Sampling
/// is [`SVKeyframeDelta::interpolate`], which is also the seam to the
/// transition kernel: the two neighbouring deltas plus an eased progress are
/// exactly what a transition consumes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SVKeyframeDelta {
    changes: Vec<SVKeyframeChange>,
}

impl SVKeyframeDelta {
    /// Builds a delta, validating every value and rejecting a repeated
    /// property.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::NonFiniteAnimationValue`] for a non-finite value and
    /// [`SVError::DuplicateAnimationProperty`] when one property appears more
    /// than once.
    pub fn new(changes: Vec<SVKeyframeChange>) -> Result<Self, SVError> {
        for (position, change) in changes.iter().enumerate() {
            if !change.value().is_finite() {
                return Err(SVError::NonFiniteAnimationValue {
                    property: change.property(),
                    position,
                });
            }
            let repeated = changes[..position]
                .iter()
                .any(|earlier| earlier.property() == change.property());
            if repeated {
                return Err(SVError::DuplicateAnimationProperty {
                    property: change.property(),
                });
            }
        }
        Ok(Self { changes })
    }

    /// Builds a delta from static pairs.
    ///
    /// # Errors
    ///
    /// The same validation as [`SVKeyframeDelta::new`].
    pub fn from_pairs(pairs: &[(SVMarkProperty, f64)]) -> Result<Self, SVError> {
        let changes: Vec<SVKeyframeChange> = pairs
            .iter()
            .map(|(property, value)| SVKeyframeChange::sv_unchecked(*property, *value))
            .collect();
        Self::new(changes)
    }

    /// Returns the ordered changes of this delta.
    pub fn changes(&self) -> &[SVKeyframeChange] {
        &self.changes
    }

    /// Returns the number of changes.
    pub fn length(&self) -> usize {
        self.changes.len()
    }

    /// Returns `true` when the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Returns the value this delta sets for a property, if it sets it.
    pub fn value_of(&self, property: SVMarkProperty) -> Option<f64> {
        self.changes
            .iter()
            .find(|change| change.property() == property)
            .map(SVKeyframeChange::value)
    }

    /// Computes the state `progress` of the way from this delta to `target`.
    ///
    /// Progress is clamped to the unit interval. An interpolable property is
    /// tweened; a discrete property keeps the start value until the very end
    /// of the segment and then swaps. A property declared by only one side is
    /// adopted as it is, so a delta never invents a value it was not given.
    pub fn interpolate(&self, target: &SVKeyframeDelta, progress: f64) -> SVKeyframeDelta {
        let progress = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let mut changes = Vec::with_capacity(self.changes.len() + target.changes.len());

        for change in &self.changes {
            let value = match target.value_of(change.property()) {
                Some(target_value) if change.property().is_interpolable() => {
                    change.value() + (target_value - change.value()) * progress
                }
                Some(target_value) if progress >= 1.0 => target_value,
                Some(_) | None => change.value(),
            };
            changes.push(SVKeyframeChange::sv_unchecked(change.property(), value));
        }

        for change in &target.changes {
            if self.value_of(change.property()).is_none() {
                changes.push(*change);
            }
        }

        SVKeyframeDelta { changes }
    }
}

/// One state of the timeline, stamped on the model clock.
#[derive(Clone, Debug, PartialEq)]
pub struct SVKeyframe {
    timestamp: SVDuration,
    delta: SVKeyframeDelta,
}

impl SVKeyframe {
    /// Builds a keyframe from a timestamp and a delta.
    pub fn new(timestamp: SVDuration, delta: SVKeyframeDelta) -> Self {
        Self { timestamp, delta }
    }

    /// Returns the timestamp of this keyframe.
    pub fn timestamp(&self) -> SVDuration {
        self.timestamp
    }

    /// Returns the delta this keyframe applies.
    pub fn delta(&self) -> &SVKeyframeDelta {
        &self.delta
    }
}
