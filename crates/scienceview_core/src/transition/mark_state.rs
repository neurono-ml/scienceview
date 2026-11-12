//! One state of one mark: its key and the properties it carries.

use super::mark_properties::SVMarkProperties;
use super::mark_property_value::SVMarkPropertyValue;
use super::match_key::SVMatchKeyValue;

/// The state of one mark at one moment.
///
/// A transition is computed between two slices of states. The key is what makes
/// the two of them comparable (design D1); the properties are what may
/// interpolate.
///
/// ```
/// use scienceview_core::sv_mark::SVMarkPoint;
/// use scienceview_core::sv_transition::{SVMarkPropertyValue, SVMarkState, SVMatchKeyValue};
///
/// let state = SVMarkState::new(SVMatchKeyValue::index(3))
///     .with_property(SVMarkPropertyValue::position(SVMarkPoint::new(12.0, 4.0)))
///     .with_property(SVMarkPropertyValue::opacity(0.8));
///
/// assert_eq!(state.properties().number_of_properties(), 2);
/// assert_eq!(state.properties().position(), Some(SVMarkPoint::new(12.0, 4.0)));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SVMarkState {
    key: SVMatchKeyValue,
    properties: SVMarkProperties,
}

impl SVMarkState {
    /// Builds an empty state for one key.
    pub fn new(key: SVMatchKeyValue) -> Self {
        Self {
            key,
            properties: SVMarkProperties::new(),
        }
    }

    /// Adds or replaces one property of the state.
    #[must_use]
    pub fn with_property(mut self, value: SVMarkPropertyValue) -> Self {
        self.properties.set(value);
        self
    }

    /// Returns the match key of the mark.
    pub fn key(&self) -> SVMatchKeyValue {
        self.key
    }

    /// Returns the properties the mark carries.
    pub fn properties(&self) -> &SVMarkProperties {
        &self.properties
    }
}
