//! A discrete palette: one named entry per series.

use crate::error::SVThemeError;
use crate::tokens::SVColour;

/// A named, ordered list of colours assigned one per series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVDiscretePalette {
    name: String,
    colours: Vec<SVColour>,
}

impl SVDiscretePalette {
    /// Builds a palette, rejecting an empty entry list.
    pub fn new(name: impl Into<String>, colours: Vec<SVColour>) -> Result<Self, SVThemeError> {
        let name = name.into();
        if colours.is_empty() {
            return Err(SVThemeError::EmptyPalette { palette_name: name });
        }
        Ok(Self { name, colours })
    }

    /// Builds a palette from entries already validated by the compiler.
    pub(crate) fn from_validated_entries(name: impl Into<String>, colours: Vec<SVColour>) -> Self {
        Self {
            name: name.into(),
            colours,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn colours(&self) -> &[SVColour] {
        &self.colours
    }

    /// Returns the entry for a series, cycling when there are more series than
    /// entries. The palette is never empty, so this cannot fail.
    pub fn colour_at_index(&self, index: usize) -> SVColour {
        self.colours[index % self.colours.len()]
    }
}
