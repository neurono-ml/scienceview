//! The transition configuration: the fluent surface a recipe composes.

use std::time::Duration;

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

use super::easing::SVEasing;
use super::enter_exit::{SVEnterRule, SVExitRule};
use super::interpolated_properties::SVInterpolatedProperties;
use super::key_resolution::sv_resolve_match_keys;
use super::mark_state::SVMarkState;
use super::match_key::{SVMatchKey, SVMatchKeyValue};
use super::motion_style::SVMotionStyle;
use super::plan::SVTransitionPlan;

/// The transition a recipe declares between two states.
///
/// The surface is fluent and immutable: every method returns a new
/// configuration, so a recipe may keep a base transition and derive variants
/// from it without sharing mutable state.
///
/// ```
/// use std::time::Duration;
///
/// use scienceview_core::sv_transition::{SVEasing, SVTransition};
///
/// let transition = SVTransition::new()
///     .duration(Duration::from_millis(240))
///     .easing(SVEasing::EaseInOut)
///     .delay(Duration::from_millis(20));
///
/// assert_eq!(
///     transition.motion_style().duration(),
///     Duration::from_millis(240)
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SVTransition {
    motion: SVMotionStyle,
    enter_rule: SVEnterRule,
    exit_rule: SVExitRule,
    match_key: SVMatchKey,
    interpolate: SVInterpolatedProperties,
}

impl SVTransition {
    /// Builds the default transition.
    ///
    /// The default is an instant swap: a zero duration, the `EaseInOut` easing,
    /// no delay, `FadeIn` for an appearance, `FadeOut` for a disappearance,
    /// `Category` keys and every interpolable property enabled. A recipe that
    /// wants motion resolves it from the theme or sets a duration.
    pub fn new() -> Self {
        Self {
            motion: SVMotionStyle::instant(),
            enter_rule: SVEnterRule::FadeIn,
            exit_rule: SVExitRule::FadeOut,
            match_key: SVMatchKey::Category,
            interpolate: SVInterpolatedProperties::all(),
        }
    }

    /// Replaces the whole motion style.
    #[must_use]
    pub fn motion(mut self, motion: SVMotionStyle) -> Self {
        self.motion = motion;
        self
    }

    /// Sets the duration of the transition.
    #[must_use]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.motion = self.motion.with_duration(duration);
        self
    }

    /// Sets the easing curve.
    #[must_use]
    pub fn easing(mut self, easing: SVEasing) -> Self {
        self.motion = self.motion.with_easing(easing);
        self
    }

    /// Sets a per-series delay.
    #[must_use]
    pub fn delay(mut self, delay: Duration) -> Self {
        self.motion = self.motion.with_delay(delay);
        self
    }

    /// Resolves the motion from a compiled theme's tokens, reduced motion
    /// included.
    #[cfg(feature = "theme")]
    #[must_use]
    pub fn motion_tokens(mut self, tokens: &scienceview_theme::SVMotionTokens) -> Self {
        self.motion = SVMotionStyle::from_motion_tokens(tokens);
        self
    }

    /// Sets the rule an entering mark follows.
    #[must_use]
    pub fn enter_rule(mut self, enter_rule: SVEnterRule) -> Self {
        self.enter_rule = enter_rule;
        self
    }

    /// Sets the rule an exiting mark follows.
    #[must_use]
    pub fn exit_rule(mut self, exit_rule: SVExitRule) -> Self {
        self.exit_rule = exit_rule;
        self
    }

    /// Sets the rule a state resolves its keys with.
    #[must_use]
    pub fn match_key(mut self, match_key: SVMatchKey) -> Self {
        self.match_key = match_key;
        self
    }

    /// Sets which interpolable properties tween.
    #[must_use]
    pub fn interpolate(mut self, interpolate: SVInterpolatedProperties) -> Self {
        self.interpolate = interpolate;
        self
    }

    /// Returns the resolved motion style.
    pub fn motion_style(&self) -> &SVMotionStyle {
        &self.motion
    }

    /// Returns the enter rule.
    pub fn enter_rule_value(&self) -> SVEnterRule {
        self.enter_rule
    }

    /// Returns the exit rule.
    pub fn exit_rule_value(&self) -> SVExitRule {
        self.exit_rule
    }

    /// Returns the match-key rule.
    pub fn match_key_rule(&self) -> SVMatchKey {
        self.match_key
    }

    /// Returns the interpolation mask.
    pub fn interpolated_properties(&self) -> SVInterpolatedProperties {
        self.interpolate
    }

    /// Resolves the keys of one state from the column that identifies its marks.
    ///
    /// # Errors
    ///
    /// Returns the error of [`sv_resolve_match_keys`]: a non-finite code or a
    /// key that repeats within the state.
    pub fn keys(&self, values: SVColumnView<'_>) -> Result<Vec<SVMatchKeyValue>, SVError> {
        sv_resolve_match_keys(values, self.match_key)
    }

    /// Computes the enter, update and exit sets of two states.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when either state repeats a
    /// match key.
    pub fn plan(
        &self,
        previous: &[SVMarkState],
        next: &[SVMarkState],
    ) -> Result<SVTransitionPlan, SVError> {
        SVTransitionPlan::new(
            self.motion,
            self.interpolate,
            self.enter_rule,
            self.exit_rule,
            previous,
            next,
        )
    }
}

impl Default for SVTransition {
    fn default() -> Self {
        Self::new()
    }
}
