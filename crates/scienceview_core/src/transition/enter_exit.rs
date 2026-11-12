//! The enter and exit rules applied to a mark that appears or disappears.
//!
//! Enter and exit are simultaneous by default (FDR-005, open questions): an
//! entering mark fades, scales or slides in from its target state, while an
//! exiting mark performs the mirror motion from its source state and is dropped
//! when the transition completes.

use core::fmt;

use crate::sv_mark::{SVMarkPoint, SVMarkProperty};

use super::mark_properties::SVMarkProperties;
use super::mark_property_value::SVMarkPropertyValue;
use super::mark_state::SVMarkState;

/// How far a sliding enter or exit travels, in logical pixels.
///
/// The direction is always vertical and downward: a mark slides in from below
/// its target and out below its last position, so two entering marks never cross
/// each other's path.
pub const SV_TRANSITION_SLIDE_DISTANCE: f64 = 16.0;

/// What an entering mark does before it reaches its target state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVEnterRule {
    /// The mark appears at its target state with no intermediary.
    #[default]
    None,
    /// The mark starts transparent and fades to its target opacity.
    FadeIn,
    /// The mark starts at zero size and grows to its target size.
    ScaleIn,
    /// The mark starts below its target and slides up.
    SlideIn,
}

impl SVEnterRule {
    /// Returns the rule name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::FadeIn => "fade in",
            Self::ScaleIn => "scale in",
            Self::SlideIn => "slide in",
        }
    }
}

/// What a disappearing mark does before it is dropped.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVExitRule {
    /// The mark disappears immediately.
    #[default]
    None,
    /// The mark fades from its previous opacity to transparent.
    FadeOut,
    /// The mark shrinks from its previous size to zero.
    ScaleOut,
    /// The mark slides below its previous position.
    SlideOut,
}

impl SVExitRule {
    /// Returns the rule name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::FadeOut => "fade out",
            Self::ScaleOut => "scale out",
            Self::SlideOut => "slide out",
        }
    }
}

impl fmt::Display for SVEnterRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for SVExitRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Returns the properties an entering mark draws at `progress`.
pub(crate) fn sv_apply_enter_rule(
    rule: SVEnterRule,
    target: &SVMarkState,
    progress: f64,
) -> SVMarkProperties {
    let mut properties = target.properties().clone();
    match rule {
        SVEnterRule::None => {}
        SVEnterRule::FadeIn => {
            sv_scale_property(&mut properties, SVMarkProperty::Opacity, progress)
        }
        SVEnterRule::ScaleIn => sv_scale_property(&mut properties, SVMarkProperty::Size, progress),
        SVEnterRule::SlideIn => {
            sv_slide(
                &mut properties,
                SV_TRANSITION_SLIDE_DISTANCE * (1.0 - progress),
            );
        }
    }
    properties
}

/// Returns the properties an exiting mark draws at `progress`.
pub(crate) fn sv_apply_exit_rule(
    rule: SVExitRule,
    source: &SVMarkState,
    progress: f64,
) -> SVMarkProperties {
    let mut properties = source.properties().clone();
    match rule {
        SVExitRule::None => {}
        SVExitRule::FadeOut => {
            sv_scale_property(&mut properties, SVMarkProperty::Opacity, 1.0 - progress);
        }
        SVExitRule::ScaleOut => {
            sv_scale_property(&mut properties, SVMarkProperty::Size, 1.0 - progress);
        }
        SVExitRule::SlideOut => {
            sv_slide(&mut properties, SV_TRANSITION_SLIDE_DISTANCE * progress);
        }
    }
    properties
}

fn sv_scale_property(properties: &mut SVMarkProperties, property: SVMarkProperty, factor: f64) {
    let scaled = match property {
        SVMarkProperty::Opacity => properties
            .get(property)
            .and_then(SVMarkPropertyValue::opacity_value)
            .map(|opacity| SVMarkPropertyValue::opacity(opacity * factor)),
        SVMarkProperty::Size => properties
            .get(property)
            .and_then(SVMarkPropertyValue::size_value)
            .map(|size| SVMarkPropertyValue::size(size * factor)),
        _ => None,
    };

    if let Some(value) = scaled {
        properties.set(value);
    }
}

fn sv_slide(properties: &mut SVMarkProperties, offset: f64) {
    let slid = properties
        .get(SVMarkProperty::Position)
        .and_then(SVMarkPropertyValue::position_value)
        .map(|position| {
            SVMarkPropertyValue::position(SVMarkPoint::new(position.x(), position.y() + offset))
        });

    if let Some(value) = slid {
        properties.set(value);
    }
}
