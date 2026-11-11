//! The closed set of scale kinds and the closed set of values they place.

use std::fmt;

/// The kind of a scale.
///
/// The enumeration is deliberately closed (patterns P5): every visual channel
/// is driven by one of these four kinds, and a new kind is a change to the
/// scale boundary rather than a third-party extension.
///
/// | Kind | Consumes | Typical channel |
/// | --- | --- | --- |
/// | [`SVScaleKind::Continuous`] | numeric values | position, size, opacity |
/// | [`SVScaleKind::Nominal`] | category labels | position of a category slot, series colour |
/// | [`SVScaleKind::Temporal`] | timestamps | position on a time axis |
/// | [`SVScaleKind::Boolean`] | `true`/`false` | one of two slots |
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVScaleKind {
    /// A numeric scale over a continuous domain.
    Continuous,
    /// A category scale over labelled slots.
    Nominal,
    /// A numeric scale whose values are timestamps.
    Temporal,
    /// A scale over the two boolean slots.
    Boolean,
}

impl SVScaleKind {
    /// Returns the kind name as written in observability spans and errors.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVScaleKind::Continuous => "Continuous",
            SVScaleKind::Nominal => "Nominal",
            SVScaleKind::Temporal => "Temporal",
            SVScaleKind::Boolean => "Boolean",
        }
    }
}

impl fmt::Display for SVScaleKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A value offered to a scale for placement.
///
/// The value and the scale are matched at call time: a [`SVScaleKind::Nominal`]
/// scale places only [`SVScaleValue::Category`] values, and so on. Offering the
/// wrong variant is a configuration failure, never a silent coercion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVScaleValue<'a> {
    /// A numeric value, placed by a continuous scale.
    Numeric(f64),
    /// A category label, placed by a nominal scale.
    Category(&'a str),
    /// A timestamp in epoch milliseconds, placed by a temporal scale.
    Timestamp(f64),
    /// A boolean value, placed by a boolean scale.
    Boolean(bool),
}

impl SVScaleValue<'_> {
    /// Returns the scale kind this value belongs to.
    pub fn kind(&self) -> SVScaleKind {
        match self {
            SVScaleValue::Numeric(_) => SVScaleKind::Continuous,
            SVScaleValue::Category(_) => SVScaleKind::Nominal,
            SVScaleValue::Timestamp(_) => SVScaleKind::Temporal,
            SVScaleValue::Boolean(_) => SVScaleKind::Boolean,
        }
    }
}
