//! The order in which the categories are drawn.

/// The order the categorical axis follows.
///
/// `Data` keeps the order in which the categories first appear in the value
/// column. The two sorted orders arrange the categories by their total
/// aggregate value, so a reader can rank the categories instead of reading the
/// axis twice; ties keep the data order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVBarOrdering {
    /// The order the categories first appear in (default).
    #[default]
    Data,
    /// Ascending total aggregate value.
    Ascending,
    /// Descending total aggregate value.
    Descending,
}

impl SVBarOrdering {
    /// Returns the ordering name as written in errors and documentation.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVBarOrdering::Data => "Data",
            SVBarOrdering::Ascending => "Ascending",
            SVBarOrdering::Descending => "Descending",
        }
    }
}
