//! A nominal scale: category labels placed on display slots.

use super::{SVDomain, SVDomainSource, SVScaleKind, SVScalePalette, SVScaleRange, SVSrgbColour};
use crate::sv_error::SVError;

/// The order in which a nominal scale displays its categories.
///
/// The order affects only the slots; it never changes the colour assignment,
/// which follows the order the categories were first seen so a series keeps its
/// colour when the display order changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVCategoryOrder {
    /// Keep the order in which the categories were first assigned.
    DataOrder,
    /// Sort the categories lexicographically for display.
    Alphabetical,
}

impl SVCategoryOrder {
    /// Returns the order name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVCategoryOrder::DataOrder => "DataOrder",
            SVCategoryOrder::Alphabetical => "Alphabetical",
        }
    }
}

/// A category scale.
///
/// The scale owns the category list, which is its domain, and the display
/// range. Categories keep their assignment index when new categories arrive,
/// so a colour assigned to a series is stable across updates (design D3).
#[derive(Clone, Debug, PartialEq)]
pub struct SVNominalScale<'a> {
    categories: Vec<&'a str>,
    order: SVCategoryOrder,
    range: SVScaleRange,
    palette: SVScalePalette,
}

impl<'a> SVNominalScale<'a> {
    /// Assembles a validated nominal scale from unique categories.
    pub(crate) fn new(
        categories: Vec<&'a str>,
        order: SVCategoryOrder,
        range: SVScaleRange,
        palette: SVScalePalette,
    ) -> Self {
        Self {
            categories,
            order,
            range,
            palette,
        }
    }

    /// Returns the kind of this scale.
    pub fn kind(&self) -> SVScaleKind {
        SVScaleKind::Nominal
    }

    /// Returns the category slots.
    ///
    /// The order is the display order, so an alphabetical axis lists its
    /// labels sorted while the assignment indices used by colours stay put.
    pub fn categories(&self) -> Vec<&'a str> {
        let mut categories = self.categories.clone();
        if self.order == SVCategoryOrder::Alphabetical {
            categories.sort_unstable();
        }
        categories
    }

    /// Returns the configured display order.
    pub fn category_order(&self) -> SVCategoryOrder {
        self.order
    }

    /// Returns the display interval.
    pub fn range(&self) -> SVScaleRange {
        self.range
    }

    /// Returns the categorical palette.
    pub fn palette(&self) -> &SVScalePalette {
        &self.palette
    }

    /// Returns the slot interval, one slot per category.
    pub fn domain(&self) -> SVDomain {
        let upper = self.categories.len().saturating_sub(1) as f64;
        SVDomain::sv_from_validated(0.0, upper)
    }

    /// Returns where the domain came from.
    pub fn domain_source(&self) -> SVDomainSource {
        SVDomainSource::Intrinsic
    }

    /// Returns the assignment index of a category.
    ///
    /// The index is where the category first appeared, which is also the
    /// palette index it keeps for the lifetime of the scale.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the category is not part
    /// of the scale.
    pub fn category_index(&self, category: &str) -> Result<usize, SVError> {
        self.categories
            .iter()
            .position(|candidate| *candidate == category)
            .ok_or_else(|| SVError::InvalidConfiguration {
                message: format!(
                    "the category `{category}` is not part of this nominal scale; assign it first"
                ),
            })
    }

    /// Places a category on the range.
    ///
    /// A single category sits in the middle of the range; otherwise the slots
    /// are evenly spaced from the start of the range to its end.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the category is unknown.
    pub fn position(&self, category: &str) -> Result<f64, SVError> {
        let assignment_index = self.category_index(category)?;
        let slot = self.display_index(assignment_index);
        let last_slot = self.categories.len().saturating_sub(1);
        let fraction = if last_slot == 0 {
            0.5
        } else {
            slot as f64 / last_slot as f64
        };
        Ok(self.range.interpolate(fraction))
    }

    /// Returns the colour of a category.
    ///
    /// The colour follows the assignment index, so it never changes when the
    /// display order changes or when another category arrives.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the category is unknown.
    pub fn colour(&self, category: &str) -> Result<SVSrgbColour, SVError> {
        Ok(self.palette.colour_at_index(self.category_index(category)?))
    }

    /// Returns the colour of a category, re-resolving a theme-bound palette.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the category is unknown or
    /// the theme no longer carries the palette token.
    #[cfg(feature = "theme")]
    pub fn colour_with_theme(
        &self,
        category: &str,
        theme: &scienceview_theme::SVTheme,
    ) -> Result<SVSrgbColour, SVError> {
        let index = self.category_index(category)?;
        self.palette.colour_at_index_with_theme(index, theme)
    }

    /// Adds categories that are not part of the scale yet.
    ///
    /// Existing categories keep their assignment index and therefore their
    /// colour and no category is removed. This is how a live figure absorbs a
    /// new series without recolouring the old ones.
    pub fn assign_categories(&mut self, categories: &[&'a str]) {
        for category in categories {
            if !self.categories.contains(category) {
                self.categories.push(category);
            }
        }
    }

    /// Returns the slot index used for display.
    fn display_index(&self, assignment_index: usize) -> usize {
        if self.order == SVCategoryOrder::DataOrder {
            return assignment_index;
        }

        let category = self.categories[assignment_index];
        self.categories
            .iter()
            .filter(|candidate| **candidate < category)
            .count()
    }
}
