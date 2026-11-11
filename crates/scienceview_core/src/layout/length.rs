//! The length model: how a figure states its width and height.
//!
//! A length is a declaration, not a number. It is resolved against the space a
//! parent offers, the figure's own measured content and, for an aspect ratio,
//! the opposite dimension. Resolution is total: non-finite input resolves to
//! zero and a non-positive ratio falls back to one, so a solver never panics.

use crate::sv_layout::SVLayoutConstraints;

/// How a width or a height is declared (PRD §4.6).
///
/// Every variant carries a full name; `ratio` is always the width divided by
/// the height.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVLength {
    /// A fixed number of logical pixels.
    Absolute { pixels: f64 },
    /// A fraction of the space offered by the parent, clamped to `0.0..=1.0`.
    Fraction { fraction: f64 },
    /// The size measured from the figure's own content.
    Intrinsic,
    /// A size derived from the opposite dimension through `ratio` (width / height).
    AspectRatio { ratio: f64 },
    /// The remaining space offered by the parent.
    Fill,
}

impl SVLength {
    /// Declares an absolute length in logical pixels.
    pub const fn absolute(pixels: f64) -> Self {
        Self::Absolute { pixels }
    }

    /// Declares a fraction of the parent's space.
    pub const fn fraction(fraction: f64) -> Self {
        Self::Fraction { fraction }
    }

    /// Declares the measured content size.
    pub const fn intrinsic() -> Self {
        Self::Intrinsic
    }

    /// Declares a size derived from the opposite dimension.
    pub const fn aspect_ratio(ratio: f64) -> Self {
        Self::AspectRatio { ratio }
    }

    /// Declares the remaining space.
    pub const fn fill() -> Self {
        Self::Fill
    }

    /// Resolves this declaration as a width.
    ///
    /// `available_width` is the space the parent offers, `intrinsic_width` is
    /// the content measurement and `height` is the already resolved opposite
    /// dimension, used only by [`SVLength::AspectRatio`].
    pub fn resolve_width(&self, available_width: f64, intrinsic_width: f64, height: f64) -> f64 {
        match *self {
            Self::Absolute { pixels } => sv_sanitize_length(pixels),
            Self::Fraction { fraction } => {
                sv_sanitize_length(available_width) * sv_sanitize_fraction(fraction)
            }
            Self::Intrinsic => sv_sanitize_length(intrinsic_width),
            Self::AspectRatio { ratio } => sv_sanitize_length(height) * sv_normalize_ratio(ratio),
            Self::Fill => sv_sanitize_length(available_width),
        }
    }

    /// Resolves this declaration as a height.
    ///
    /// `available_height` is the space the parent offers, `intrinsic_height` is
    /// the content measurement and `width` is the already resolved opposite
    /// dimension, used only by [`SVLength::AspectRatio`].
    pub fn resolve_height(&self, available_height: f64, intrinsic_height: f64, width: f64) -> f64 {
        match *self {
            Self::Absolute { pixels } => sv_sanitize_length(pixels),
            Self::Fraction { fraction } => {
                sv_sanitize_length(available_height) * sv_sanitize_fraction(fraction)
            }
            Self::Intrinsic => sv_sanitize_length(intrinsic_height),
            Self::AspectRatio { ratio } => sv_sanitize_length(width) / sv_normalize_ratio(ratio),
            Self::Fill => sv_sanitize_length(available_height),
        }
    }

    /// Returns `true` for [`SVLength::Fill`].
    pub const fn is_fill(&self) -> bool {
        matches!(self, Self::Fill)
    }

    /// Returns `true` for [`SVLength::Intrinsic`].
    pub const fn is_intrinsic(&self) -> bool {
        matches!(self, Self::Intrinsic)
    }

    /// Returns `true` for [`SVLength::Absolute`].
    pub const fn is_absolute(&self) -> bool {
        matches!(self, Self::Absolute { .. })
    }

    /// Resolves as a width and clamps the result to `constraints`.
    pub fn resolve_constrained_width(
        &self,
        constraints: SVLayoutConstraints,
        available_width: f64,
        intrinsic_width: f64,
        height: f64,
    ) -> f64 {
        constraints.clamp_width(self.resolve_width(available_width, intrinsic_width, height))
    }

    /// Resolves as a height and clamps the result to `constraints`.
    pub fn resolve_constrained_height(
        &self,
        constraints: SVLayoutConstraints,
        available_height: f64,
        intrinsic_height: f64,
        width: f64,
    ) -> f64 {
        constraints.clamp_height(self.resolve_height(available_height, intrinsic_height, width))
    }
}

impl Default for SVLength {
    /// The default declaration is the measured content size.
    fn default() -> Self {
        Self::Intrinsic
    }
}

/// Normalizes a value that must be a finite, non-negative length.
pub(crate) fn sv_sanitize_length(value: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        0.0
    }
}

/// Normalizes a fraction to `0.0..=1.0`, mapping non-finite input to zero.
pub(crate) fn sv_sanitize_fraction(fraction: f64) -> f64 {
    if fraction.is_finite() {
        fraction.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// Normalizes an aspect ratio, falling back to one when it is not positive.
pub(crate) fn sv_normalize_ratio(ratio: f64) -> f64 {
    if ratio.is_finite() && ratio > 0.0 {
        ratio
    } else {
        1.0
    }
}
