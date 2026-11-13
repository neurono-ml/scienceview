//! How the series of a histogram share the space of a bin.

/// The arrangement of several series inside one bin.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVHistogramMultiple {
    /// Every series draws its own full-width bar, layered in draw order
    /// (default).
    #[default]
    Layer,
    /// Every series grows from the top of the preceding one.
    Stack,
    /// Every series occupies its own slice of the bin slot.
    Dodge,
}

impl SVHistogramMultiple {
    /// Returns the arrangement name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVHistogramMultiple::Layer => "Layer",
            SVHistogramMultiple::Stack => "Stack",
            SVHistogramMultiple::Dodge => "Dodge",
        }
    }
}
