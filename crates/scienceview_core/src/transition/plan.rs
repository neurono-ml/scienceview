//! The computed enter, update and exit sets and their frame evaluation.

use std::collections::BTreeMap;
use std::time::Duration;

use crate::sv_error::SVError;

use super::enter_exit::{SVEnterRule, SVExitRule};
use super::interpolated_properties::SVInterpolatedProperties;
use super::key_resolution::sv_verify_unique_match_keys;
use super::mark_state::SVMarkState;
use super::match_key::SVMatchKeyValue;
use super::motion_style::SVMotionStyle;

/// A mark that persists between the two states.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransitionUpdate {
    pub(super) key: SVMatchKeyValue,
    pub(super) from: SVMarkState,
    pub(super) to: SVMarkState,
}

impl SVTransitionUpdate {
    /// Returns the match key of the mark.
    pub fn key(&self) -> SVMatchKeyValue {
        self.key
    }

    /// Returns the state the mark came from.
    pub fn from(&self) -> &SVMarkState {
        &self.from
    }

    /// Returns the state the mark is going to.
    pub fn to(&self) -> &SVMarkState {
        &self.to
    }
}

/// A mark that only exists in the next state.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransitionEnter {
    pub(super) key: SVMatchKeyValue,
    pub(super) target: SVMarkState,
}

impl SVTransitionEnter {
    /// Returns the match key of the mark.
    pub fn key(&self) -> SVMatchKeyValue {
        self.key
    }

    /// Returns the state the mark enters at.
    pub fn target(&self) -> &SVMarkState {
        &self.target
    }
}

/// A mark that only exists in the previous state.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransitionExit {
    pub(super) key: SVMatchKeyValue,
    pub(super) source: SVMarkState,
}

impl SVTransitionExit {
    /// Returns the match key of the mark.
    pub fn key(&self) -> SVMatchKeyValue {
        self.key
    }

    /// Returns the state the mark exits from.
    pub fn source(&self) -> &SVMarkState {
        &self.source
    }
}

/// The three sets of one transition and the rules that drive them.
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransitionPlan {
    pub(super) motion: SVMotionStyle,
    pub(super) interpolate: SVInterpolatedProperties,
    pub(super) enter_rule: SVEnterRule,
    pub(super) exit_rule: SVExitRule,
    pub(super) updates: Vec<SVTransitionUpdate>,
    pub(super) enters: Vec<SVTransitionEnter>,
    pub(super) exits: Vec<SVTransitionExit>,
}

impl SVTransitionPlan {
    /// Computes the three sets from two states.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when either state repeats a
    /// match key, because the pairing would be ambiguous.
    pub(crate) fn new(
        motion: SVMotionStyle,
        interpolate: SVInterpolatedProperties,
        enter_rule: SVEnterRule,
        exit_rule: SVExitRule,
        previous: &[SVMarkState],
        next: &[SVMarkState],
    ) -> Result<Self, SVError> {
        let previous_keys: Vec<SVMatchKeyValue> = previous.iter().map(SVMarkState::key).collect();
        let next_keys: Vec<SVMatchKeyValue> = next.iter().map(SVMarkState::key).collect();
        sv_verify_unique_match_keys(&previous_keys)?;
        sv_verify_unique_match_keys(&next_keys)?;

        let mut previous_positions: BTreeMap<SVMatchKeyValue, usize> = BTreeMap::new();
        for (position, key) in previous_keys.iter().enumerate() {
            previous_positions.insert(*key, position);
        }
        let next_positions: BTreeMap<SVMatchKeyValue, usize> =
            next_keys.iter().copied().zip(0..).collect();

        let mut updates = Vec::new();
        let mut enters = Vec::new();
        for (position, state) in next.iter().enumerate() {
            match previous_positions.get(&next_keys[position]) {
                Some(previous_position) => updates.push(SVTransitionUpdate {
                    key: next_keys[position],
                    from: previous[*previous_position].clone(),
                    to: state.clone(),
                }),
                None => enters.push(SVTransitionEnter {
                    key: next_keys[position],
                    target: state.clone(),
                }),
            }
        }

        let exits = previous
            .iter()
            .zip(previous_keys.iter())
            .filter(|(_, key)| !next_positions.contains_key(key))
            .map(|(state, key)| SVTransitionExit {
                key: *key,
                source: state.clone(),
            })
            .collect();

        Ok(Self {
            motion,
            interpolate,
            enter_rule,
            exit_rule,
            updates,
            enters,
            exits,
        })
    }

    /// Returns the resolved motion style.
    pub fn motion(&self) -> &SVMotionStyle {
        &self.motion
    }

    /// Returns the persists set.
    pub fn update(&self) -> &[SVTransitionUpdate] {
        &self.updates
    }

    /// Returns the appearances set.
    pub fn enter(&self) -> &[SVTransitionEnter] {
        &self.enters
    }

    /// Returns the disappearances set.
    pub fn exit(&self) -> &[SVTransitionExit] {
        &self.exits
    }

    /// Maps a caller-supplied elapsed time onto the progress of the timeline.
    pub fn progress_at(&self, elapsed: Duration) -> f64 {
        self.motion.progress_at(elapsed)
    }
}
