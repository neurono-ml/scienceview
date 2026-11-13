//! The order in which the categories are drawn.

/// The order the categorical axis follows.
///
/// `Data` keeps the order in which the categories first appear in the value
/// column. The two sorted orders arrange the categories by their aggregated
/// estimate, so a reader can rank the categories instead of reading the axis
/// twice; ties keep the data order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVPointOrdering {
    /// The order the categories first appear in (default).
    #[default]
    Data,
    /// Ascending aggregated estimate.
    Ascending,
    /// Descending aggregated estimate.
    Descending,
}

impl SVPointOrdering {
    /// Returns the ordering name as written in errors and documentation.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVPointOrdering::Data => "Data",
            SVPointOrdering::Ascending => "Ascending",
            SVPointOrdering::Descending => "Descending",
        }
    }
}
