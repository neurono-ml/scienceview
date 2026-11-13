//! How the series of a count plot share the space of a category.
//!
//! The arrangement is a vocabulary type only: the geometry comes from
//! [`SVMove`](crate::sv_move::SVMove), never from this module.

/// The arrangement of several series inside one category.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVCountMultiple {
    /// Every series draws its own full-width bar, layered in draw order
    /// (default).
    #[default]
    Layer,
    /// Every series grows from the top of the preceding one.
    Stack,
    /// Every series occupies its own slice of the category slot.
    Dodge,
}

impl SVCountMultiple {
    /// Returns the arrangement name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVCountMultiple::Layer => "Layer",
            SVCountMultiple::Stack => "Stack",
            SVCountMultiple::Dodge => "Dodge",
        }
    }
}
