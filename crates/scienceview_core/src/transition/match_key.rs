//! The match-key rule and the typed value a state resolves it into.

use core::fmt;

/// The rule a recipe declares for identifying one mark within its series.
///
/// Matching by position is never an option (design D1): a mark that moved must
/// be recognized as the same mark, so every state carries a key and the
/// transition matches on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVMatchKey {
    /// The value of a categorical column, as its numeric slot code.
    Category,
    /// The logical position in the column, stable across states.
    Index,
    /// An explicit identifier column, as its numeric code.
    Identity,
    /// A deterministic hash of the mark's discriminating value with a seed.
    Derived {
        /// The seed that distinguishes one derivation from another.
        seed: u64,
    },
}

impl SVMatchKey {
    /// Returns the rule name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Index => "index",
            Self::Identity => "identity",
            Self::Derived { .. } => "derived",
        }
    }
}

impl fmt::Display for SVMatchKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A resolved match key.
///
/// The variant carries the rule as well as the code, so a category code never
/// equals an identity code by accident (design D1). Numeric codes and derived
/// hashes are stored as their 64-bit representation, which makes equality and
/// hashing exact and platform-independent; the constructors reject a
/// non-finite code up front.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SVMatchKeyValue {
    /// A categorical slot code.
    Category(u64),
    /// A logical position in the column.
    Index(usize),
    /// An explicit identifier code.
    Identity(u64),
    /// A deterministic derived hash.
    Derived(u64),
}

impl SVMatchKeyValue {
    /// Resolves a categorical code into a key.
    ///
    /// Returns `None` for a non-finite code: a mark without a finite identity
    /// cannot be matched, and the caller reports the position.
    pub fn category(code: f64) -> Option<Self> {
        sv_code_bits(code).map(Self::Category)
    }

    /// Resolves an explicit identifier code into a key.
    ///
    /// Returns `None` for a non-finite code, for the same reason as
    /// [`SVMatchKeyValue::category`].
    pub fn identity(code: f64) -> Option<Self> {
        sv_code_bits(code).map(Self::Identity)
    }

    /// Resolves a logical position into a key.
    pub const fn index(position: usize) -> Self {
        Self::Index(position)
    }

    /// Wraps a derived hash as a key.
    pub const fn derived(hash: u64) -> Self {
        Self::Derived(hash)
    }

    /// Returns the key name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Category(_) => "category",
            Self::Index(_) => "index",
            Self::Identity(_) => "identity",
            Self::Derived(_) => "derived",
        }
    }
}

impl fmt::Display for SVMatchKeyValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Category(bits) => write!(formatter, "category key {}", f64::from_bits(*bits)),
            Self::Index(position) => write!(formatter, "index key {position}"),
            Self::Identity(bits) => write!(formatter, "identity key {}", f64::from_bits(*bits)),
            Self::Derived(hash) => write!(formatter, "derived key {hash:#018x}"),
        }
    }
}

fn sv_code_bits(code: f64) -> Option<u64> {
    code.is_finite().then(|| code.to_bits())
}
