//! A continuous colour map: ordered stops interpolated by a fraction.

use crate::error::SVThemeError;
use crate::tokens::SVColour;

/// How a colour map is meant to be read, so a legend can be labelled honestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SVColourMapKind {
    Sequential,
    Diverging,
    Cyclical,
}

/// A named colour map with at least two evenly spaced stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVContinuousPalette {
    name: String,
    kind: SVColourMapKind,
    stops: Vec<SVColour>,
}

impl SVContinuousPalette {
    /// Builds a colour map, rejecting fewer than two stops.
    pub fn new(
        name: impl Into<String>,
        kind: SVColourMapKind,
        stops: Vec<SVColour>,
    ) -> Result<Self, SVThemeError> {
        let name = name.into();
        if stops.len() < 2 {
            return Err(SVThemeError::EmptyPalette { palette_name: name });
        }
        Ok(Self { name, kind, stops })
    }

    /// Builds a colour map from stops already validated by the compiler.
    pub(crate) fn from_validated_stops(
        name: impl Into<String>,
        kind: SVColourMapKind,
        stops: Vec<SVColour>,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            stops,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> SVColourMapKind {
        self.kind
    }

    pub fn stops(&self) -> &[SVColour] {
        &self.stops
    }

    /// Interpolates the map at `fraction`.
    ///
    /// The fraction is clamped to `0.0..=1.0` and a non-finite fraction maps to
    /// the first stop, so an out-of-range value never panics. With `n` stops,
    /// fraction `0.0` is the first stop, `1.0` the last, and `0.5` the middle
    /// stop when `n` is odd.
    pub fn colour_at(&self, fraction: f64) -> SVColour {
        let fraction = if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let last_index = self.stops.len() - 1;
        let scaled = fraction * last_index as f64;
        let lower_index = scaled.floor() as usize;
        let upper_index = lower_index.saturating_add(1).min(last_index);
        let local_fraction = scaled - lower_index as f64;
        self.stops[lower_index].interpolate(self.stops[upper_index], local_fraction)
    }
}
