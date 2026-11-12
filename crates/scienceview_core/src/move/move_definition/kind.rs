//! The kind of a move, independent of its options.

/// The kind of a move, independent of its options.
///
/// Observability and the stage table are keyed by this value, never by an
/// option struct, so a new variant fails both until it declares itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMoveKind {
    /// The stack move.
    Stack,
    /// The dodge move.
    Dodge,
    /// The jitter move.
    Jitter,
    /// The shift move.
    Shift,
    /// The swarm move.
    Swarm,
    /// The normalize move.
    Normalize,
}

impl SVMoveKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stack,
        Self::Dodge,
        Self::Jitter,
        Self::Shift,
        Self::Swarm,
        Self::Normalize,
    ];

    /// Returns a stable lowercase name for spans and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stack => "stack",
            Self::Dodge => "dodge",
            Self::Jitter => "jitter",
            Self::Shift => "shift",
            Self::Swarm => "swarm",
            Self::Normalize => "normalize",
        }
    }
}
