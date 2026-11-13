//! The typed options a scatterplot matrix exposes beyond its variable list.

/// What a diagonal cell of the matrix draws.
///
/// The diagonal shows the univariate distribution of the variable that owns
/// the cell, so the matrix is readable without a separate marginal figure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVSplomDiagonal {
    /// Binned counts, the distribution form of the reference libraries.
    #[default]
    Histogram,
    /// A kernel density estimate drawn as a filled curve.
    Density,
    /// No diagonal content; only the off-diagonal cells draw.
    Empty,
}

impl SVSplomDiagonal {
    /// The option name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Histogram => "Histogram",
            Self::Density => "Density",
            Self::Empty => "Empty",
        }
    }
}

/// How the axes of the matrix relate to each other.
///
/// A scatterplot matrix can either give every variable its own domain — the
/// behaviour of `seaborn.pairplot`, where a column of cells shares the
/// horizontal axis of that column — or place every variable on one common
/// domain, which makes the cells directly comparable at the cost of wasting
/// space when the variables have very different ranges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVSplomScaleMode {
    /// Every variable owns one domain, shared by its row and its column.
    #[default]
    Independent,
    /// Every cell shares one domain derived from all variables together.
    Shared,
}

impl SVSplomScaleMode {
    /// The option name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Independent => "Independent",
            Self::Shared => "Shared",
        }
    }
}

/// What the matrix does with an observation that is not finite.
///
/// An observation is one row of the data set: it enters every cell together.
/// A non-finite value in any of its variable columns makes the whole
/// observation unplottable, so the policy decides the fate of that observation
/// in every cell at once — never in one cell only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVSplomMissingValuePolicy {
    /// Drop the observation from every cell (default).
    ///
    /// The remaining observations stay aligned across the matrix, so an
    /// observation keeps the same index in every cell.
    #[default]
    Omit,
    /// Reject the whole render with an explicit configuration error.
    Error,
}

impl SVSplomMissingValuePolicy {
    /// The option name as written in errors and spans.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omit => "Omit",
            Self::Error => "Error",
        }
    }
}
