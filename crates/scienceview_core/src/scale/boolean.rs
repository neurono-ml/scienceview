//! A boolean scale: the two values placed on two slots.

use super::{SVDomain, SVScaleKind, SVScaleRange};

/// A scale over the two boolean values.
///
/// `false` occupies the start of the range and `true` its end, so an inverted
/// range simply swaps the two slots. The labels are part of the scale because
/// an axis must be able to name the slots without consulting the theme.
#[derive(Clone, Debug, PartialEq)]
pub struct SVBooleanScale {
    range: SVScaleRange,
    labels: (String, String),
}

impl SVBooleanScale {
    /// Assembles a validated boolean scale.
    pub(crate) fn new(range: SVScaleRange, labels: (String, String)) -> Self {
        Self { range, labels }
    }

    /// Returns the kind of this scale.
    pub fn kind(&self) -> SVScaleKind {
        SVScaleKind::Boolean
    }

    /// Returns the display interval.
    pub fn range(&self) -> SVScaleRange {
        self.range
    }

    /// Returns the labels of `false` and `true`, in that order.
    pub fn labels(&self) -> (&str, &str) {
        (self.labels.0.as_str(), self.labels.1.as_str())
    }

    /// Returns the label of one boolean value.
    pub fn label_for(&self, value: bool) -> &str {
        if value {
            self.labels.1.as_str()
        } else {
            self.labels.0.as_str()
        }
    }

    /// Returns the two-slot interval.
    pub fn domain(&self) -> SVDomain {
        SVDomain::sv_from_validated(0.0, 1.0)
    }

    /// Places a boolean value on the range.
    pub fn position(&self, value: bool) -> f64 {
        self.range.interpolate(if value { 1.0 } else { 0.0 })
    }
}
