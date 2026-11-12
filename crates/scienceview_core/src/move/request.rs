//! The borrowed request a move consumes.

use crate::sv_data::SVColumnView;
use crate::sv_error::SVError;

/// The values and the keys a move displaces.
///
/// The values are borrowed as one [`SVColumnView`] and never copied. The keys
/// are group codes and, optionally, category codes: a recipe resolves its
/// nominal categories to slot codes once, exactly as a mark does, so the move
/// never parses labels and never allocates.
///
/// The category keys default to the group keys, which is the common case of a
/// single categorical axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVMoveRequest<'a> {
    values: SVColumnView<'a>,
    group_keys: &'a [u32],
    category_keys: &'a [u32],
}

impl<'a> SVMoveRequest<'a> {
    /// Builds a request whose category keys are its group keys.
    pub fn new(values: SVColumnView<'a>, group_keys: &'a [u32]) -> Self {
        Self {
            values,
            group_keys,
            category_keys: group_keys,
        }
    }

    /// Sets a distinct category axis.
    pub fn with_category_keys(mut self, category_keys: &'a [u32]) -> Self {
        self.category_keys = category_keys;
        self
    }

    /// Returns the borrowed values.
    pub fn values(&self) -> &SVColumnView<'a> {
        &self.values
    }

    /// Returns the group codes.
    pub fn group_keys(&self) -> &'a [u32] {
        self.group_keys
    }

    /// Returns the category codes.
    pub fn category_keys(&self) -> &'a [u32] {
        self.category_keys
    }

    /// Returns the number of logical points.
    pub fn number_of_points(&self) -> usize {
        self.values.length()
    }

    /// Validates that every key column matches the values.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MismatchedLengths`] naming the offending key column.
    pub fn validate(&self) -> Result<(), SVError> {
        sv_validate_key_length("group_keys", self.group_keys.len(), self.number_of_points())?;
        sv_validate_key_length(
            "category_keys",
            self.category_keys.len(),
            self.number_of_points(),
        )
    }
}

/// Validates one key column against the values.
pub(crate) fn sv_validate_key_length(
    name: &'static str,
    actual: usize,
    expected: usize,
) -> Result<(), SVError> {
    if actual == expected {
        return Ok(());
    }
    Err(SVError::MismatchedLengths {
        name: name.to_string(),
        expected,
        actual,
    })
}
