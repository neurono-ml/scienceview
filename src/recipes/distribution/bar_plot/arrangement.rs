//! How several series share the space of one category slot.

/// The arrangement of the series inside one category slot.
///
/// The layout itself is produced by [`SVMove`](crate::sv_move::SVMove): the
/// recipe only chooses which move of the kernel it applies, never how the
/// displacement is computed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBarArrangement {
    /// Every series occupies its own slice of the category slot (default).
    #[default]
    Dodge,
    /// Every series grows from the top of the preceding one.
    Stack,
}

impl SVBarArrangement {
    /// Returns the arrangement name as written in errors and documentation.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVBarArrangement::Dodge => "Dodge",
            SVBarArrangement::Stack => "Stack",
        }
    }
}
