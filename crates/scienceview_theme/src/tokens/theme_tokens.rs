//! The compiled token value. Backends read these fields; none of them parses a
//! stylesheet.

use crate::tokens::{SVColour, SVMotionTokens, SVShadow, SVShadowTokens, SVTypographyTokens};

/// Every backend-independent token of a compiled theme.
#[derive(Debug, Clone, PartialEq)]
pub struct SVThemeTokens {
    pub(crate) colour: SVColourTokens,
    pub(crate) typography: SVTypographyTokens,
    pub(crate) spacing: SVSpacingTokens,
    pub(crate) radius: SVRadiusTokens,
    pub(crate) shadow: SVShadowTokens,
    pub(crate) motion: SVMotionTokens,
}

impl SVThemeTokens {
    pub fn colour(&self) -> &SVColourTokens {
        &self.colour
    }

    pub fn typography(&self) -> &SVTypographyTokens {
        &self.typography
    }

    pub fn spacing(&self) -> &SVSpacingTokens {
        &self.spacing
    }

    pub fn radius(&self) -> &SVRadiusTokens {
        &self.radius
    }

    pub fn shadow(&self) -> &SVShadowTokens {
        &self.shadow
    }

    pub fn motion(&self) -> &SVMotionTokens {
        &self.motion
    }
}

/// Semantic colour roles. A renderer asks for `grid` or `accent`, never for a
/// literal brand value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SVColourTokens {
    pub(crate) background: SVColour,
    pub(crate) surface: SVColour,
    pub(crate) text: SVColour,
    pub(crate) text_muted: SVColour,
    pub(crate) grid: SVColour,
    pub(crate) axis: SVColour,
    pub(crate) accent: SVColour,
    pub(crate) success: SVColour,
    pub(crate) warning: SVColour,
    pub(crate) error: SVColour,
}

impl SVColourTokens {
    pub fn background(&self) -> SVColour {
        self.background
    }

    pub fn surface(&self) -> SVColour {
        self.surface
    }

    pub fn text(&self) -> SVColour {
        self.text
    }

    pub fn text_muted(&self) -> SVColour {
        self.text_muted
    }

    pub fn grid(&self) -> SVColour {
        self.grid
    }

    pub fn axis(&self) -> SVColour {
        self.axis
    }

    pub fn accent(&self) -> SVColour {
        self.accent
    }

    pub fn success(&self) -> SVColour {
        self.success
    }

    pub fn warning(&self) -> SVColour {
        self.warning
    }

    pub fn error(&self) -> SVColour {
        self.error
    }
}

/// Spacing scale, in pixels. `unit` is the base step the others are built from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVSpacingTokens {
    pub(crate) unit: f64,
    pub(crate) x_small: f64,
    pub(crate) small: f64,
    pub(crate) medium: f64,
    pub(crate) large: f64,
    pub(crate) x_large: f64,
}

impl SVSpacingTokens {
    pub fn unit(&self) -> f64 {
        self.unit
    }

    pub fn x_small(&self) -> f64 {
        self.x_small
    }

    pub fn small(&self) -> f64 {
        self.small
    }

    pub fn medium(&self) -> f64 {
        self.medium
    }

    pub fn large(&self) -> f64 {
        self.large
    }

    pub fn x_large(&self) -> f64 {
        self.x_large
    }
}

/// Corner radii, in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVRadiusTokens {
    pub(crate) none: f64,
    pub(crate) small: f64,
    pub(crate) medium: f64,
    pub(crate) large: f64,
}

impl SVRadiusTokens {
    pub fn none(&self) -> f64 {
        self.none
    }

    pub fn small(&self) -> f64 {
        self.small
    }

    pub fn medium(&self) -> f64 {
        self.medium
    }

    pub fn large(&self) -> f64 {
        self.large
    }
}

/// The token value a name resolves to, typed for its consumer.
#[derive(Debug, Clone, PartialEq)]
pub enum SVTokenValue {
    Colour(SVColour),
    Length(f64),
    Number(f64),
    FontFamily(String),
    FontWeight(u16),
    Shadow(SVShadow),
    Duration(std::time::Duration),
    Easing(crate::tokens::SVTimingFunction),
    Boolean(bool),
}

impl SVThemeTokens {
    /// Resolves a dotted token name into its typed value.
    ///
    /// Unknown names and malformed lookups return [`crate::SVThemeError`]; this
    /// method never panics. Motion durations are collapsed to zero when the
    /// theme declares reduced motion, so the caller cannot forget.
    pub fn resolve(&self, token_name: &str) -> Result<SVTokenValue, crate::SVThemeError> {
        use crate::tokens::SVMotionDuration::{Fast, Normal, Slow};
        let colour = |value: SVColour| SVTokenValue::Colour(value);
        match token_name {
            "colour.background" => Ok(colour(self.colour.background)),
            "colour.surface" => Ok(colour(self.colour.surface)),
            "colour.text" => Ok(colour(self.colour.text)),
            "colour.text_muted" => Ok(colour(self.colour.text_muted)),
            "colour.grid" => Ok(colour(self.colour.grid)),
            "colour.axis" => Ok(colour(self.colour.axis)),
            "colour.accent" => Ok(colour(self.colour.accent)),
            "colour.success" => Ok(colour(self.colour.success)),
            "colour.warning" => Ok(colour(self.colour.warning)),
            "colour.error" => Ok(colour(self.colour.error)),
            "typography.family" => Ok(SVTokenValue::FontFamily(self.typography.family.clone())),
            "typography.size.body" => Ok(SVTokenValue::Length(self.typography.size.body)),
            "typography.size.title" => Ok(SVTokenValue::Length(self.typography.size.title)),
            "typography.size.label" => Ok(SVTokenValue::Length(self.typography.size.label)),
            "typography.weight.regular" => {
                Ok(SVTokenValue::FontWeight(self.typography.weight.regular))
            }
            "typography.weight.bold" => Ok(SVTokenValue::FontWeight(self.typography.weight.bold)),
            "typography.line_height" => Ok(SVTokenValue::Number(self.typography.line_height)),
            "spacing.unit" => Ok(SVTokenValue::Length(self.spacing.unit)),
            "spacing.x_small" => Ok(SVTokenValue::Length(self.spacing.x_small)),
            "spacing.small" => Ok(SVTokenValue::Length(self.spacing.small)),
            "spacing.medium" => Ok(SVTokenValue::Length(self.spacing.medium)),
            "spacing.large" => Ok(SVTokenValue::Length(self.spacing.large)),
            "spacing.x_large" => Ok(SVTokenValue::Length(self.spacing.x_large)),
            "radius.none" => Ok(SVTokenValue::Length(self.radius.none)),
            "radius.small" => Ok(SVTokenValue::Length(self.radius.small)),
            "radius.medium" => Ok(SVTokenValue::Length(self.radius.medium)),
            "radius.large" => Ok(SVTokenValue::Length(self.radius.large)),
            "shadow.none" => Ok(SVTokenValue::Shadow(self.shadow.none)),
            "shadow.overlay" => Ok(SVTokenValue::Shadow(self.shadow.overlay)),
            "motion.duration.fast" => Ok(SVTokenValue::Duration(self.motion.duration(Fast))),
            "motion.duration.normal" => Ok(SVTokenValue::Duration(self.motion.duration(Normal))),
            "motion.duration.slow" => Ok(SVTokenValue::Duration(self.motion.duration(Slow))),
            "motion.easing.standard" => Ok(SVTokenValue::Easing(
                self.motion.easing(crate::tokens::SVMotionEasing::Standard),
            )),
            "motion.easing.emphasized" => Ok(SVTokenValue::Easing(
                self.motion
                    .easing(crate::tokens::SVMotionEasing::Emphasized),
            )),
            "motion.easing.decelerate" => Ok(SVTokenValue::Easing(
                self.motion
                    .easing(crate::tokens::SVMotionEasing::Decelerate),
            )),
            "motion.reduced" => Ok(SVTokenValue::Boolean(self.motion.reduced_motion)),
            unknown => Err(crate::SVThemeError::UnknownToken {
                token_name: unknown.to_string(),
            }),
        }
    }
}
