//! The palette model: discrete, continuous and data-derived.
//!
//! The three are distinct types because their animation rule differs: a
//! nominal colour assigned by a discrete palette stays stable when data
//! arrives, while a continuous colour interpolates and a derived palette
//! recomputes from the data.

mod continuous;
mod discrete;

pub use continuous::{SVColourMapKind, SVContinuousPalette};
pub use discrete::SVDiscretePalette;

/// Every palette a compiled theme carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVColourPalettes {
    pub(crate) categorical: Vec<SVDiscretePalette>,
    pub(crate) continuous: Vec<SVContinuousPalette>,
}

impl SVColourPalettes {
    /// Finds a categorical palette by name, or `None` when the theme does not
    /// define it. Lookup never panics.
    pub fn categorical_palette(&self, name: &str) -> Option<&SVDiscretePalette> {
        self.categorical
            .iter()
            .find(|palette| palette.name() == name)
    }

    /// Finds a continuous colour map by name, or `None`.
    pub fn continuous_palette(&self, name: &str) -> Option<&SVContinuousPalette> {
        self.continuous
            .iter()
            .find(|palette| palette.name() == name)
    }

    /// Every categorical palette, in the order the stylesheet declared them.
    pub fn categorical_palettes(&self) -> &[SVDiscretePalette] {
        &self.categorical
    }

    /// Every continuous colour map, in declaration order.
    pub fn continuous_palettes(&self) -> &[SVContinuousPalette] {
        &self.continuous
    }

    /// Every continuous colour map of one kind.
    pub fn continuous_palettes_of_kind(
        &self,
        kind: SVColourMapKind,
    ) -> impl Iterator<Item = &SVContinuousPalette> {
        self.continuous
            .iter()
            .filter(move |palette| palette.kind() == kind)
    }
}
