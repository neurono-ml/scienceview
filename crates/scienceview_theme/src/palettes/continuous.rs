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
}
