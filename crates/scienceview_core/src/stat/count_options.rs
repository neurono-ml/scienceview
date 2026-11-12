//! The counting options.

/// How the counted categories are ordered in the result.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVCountOrder {
    /// Keep the order in which the categories first appear in the column.
    #[default]
    DataOrder,
    /// Sort the categories from the smallest to the largest value.
    Ascending,
    /// Sort the categories from the largest to the smallest value.
    Descending,
}

impl SVCountOrder {
    /// Returns the order name as written in spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVCountOrder::DataOrder => "DataOrder",
            SVCountOrder::Ascending => "Ascending",
            SVCountOrder::Descending => "Descending",
        }
    }
}

/// The configuration of [`SVStat::Count`](super::SVStat::Count).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SVCountOptions {
    order: SVCountOrder,
}

impl SVCountOptions {
    /// Starts a counting configuration in data order.
    pub fn new() -> Self {
        Self::default()
    }

    /// Chooses the order of the returned categories.
    #[must_use]
    pub fn order(mut self, order: SVCountOrder) -> Self {
        self.order = order;
        self
    }

    /// Returns the requested order.
    pub fn requested_order(&self) -> SVCountOrder {
        self.order
    }
}
