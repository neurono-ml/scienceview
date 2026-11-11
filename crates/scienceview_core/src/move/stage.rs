//! The fixed composition stages (design D2).

/// A stage of the fixed composition order.
///
/// Moves only compose in stage order: [`Self::Displacement`] first — one of
/// stack, dodge, jitter or swarm — then [`Self::Shift`], then
/// [`Self::Normalisation`]. The ordering is normative because reversing it
/// changes the chart: normalising before dodging is not dodging before
/// normalising.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVMoveStage {
    /// Stacking, dodging, jittering or swarming: where a mark sits.
    Displacement,
    /// A fixed displacement on top of the layout.
    Shift,
    /// Rescaling against the group total.
    Normalisation,
}

impl SVMoveStage {
    /// Every stage, in the order they apply.
    pub const ALL: [Self; 3] = [Self::Displacement, Self::Shift, Self::Normalisation];

    /// Returns a stable lowercase name for diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Displacement => "displacement",
            Self::Shift => "shift",
            Self::Normalisation => "normalisation",
        }
    }
}

impl core::fmt::Display for SVMoveStage {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
