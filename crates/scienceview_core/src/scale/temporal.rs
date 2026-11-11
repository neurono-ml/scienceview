//! A temporal scale: timestamps placed on a time axis.

use super::{
    SVContinuousScale, SVDomain, SVDomainSource, SVScaleKind, SVScaleRange, SVScaleTransform,
};
use crate::sv_error::SVError;

/// The unit a temporal axis ticks at.
///
/// [`SVTemporalUnit::Auto`] lets the tick derivation pick a unit from the space
/// the axis was granted; an explicit unit pins the ladder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SVTemporalUnit {
    /// Choose the unit from the available space.
    Auto,
    /// One millisecond steps.
    Millisecond,
    /// One second steps.
    Second,
    /// One minute steps.
    Minute,
    /// One hour steps.
    Hour,
    /// One day steps.
    Day,
    /// One week steps.
    Week,
    /// One calendar month at the mean month length.
    Month,
    /// One calendar year at the mean year length.
    Year,
}

impl SVTemporalUnit {
    /// Returns the unit name as written in observability spans and labels.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVTemporalUnit::Auto => "Auto",
            SVTemporalUnit::Millisecond => "ms",
            SVTemporalUnit::Second => "s",
            SVTemporalUnit::Minute => "min",
            SVTemporalUnit::Hour => "h",
            SVTemporalUnit::Day => "d",
            SVTemporalUnit::Week => "wk",
            SVTemporalUnit::Month => "mo",
            SVTemporalUnit::Year => "y",
        }
    }

    /// Returns how many milliseconds one step of the unit spans.
    ///
    /// Months and years use the mean Gregorian lengths, so a label is honest
    /// about being approximate rather than pretending to be calendar-aware.
    /// [`SVTemporalUnit::Auto`] has no fixed length and answers `None`.
    pub fn milliseconds(&self) -> Option<f64> {
        match self {
            SVTemporalUnit::Auto => None,
            SVTemporalUnit::Millisecond => Some(1.0),
            SVTemporalUnit::Second => Some(1_000.0),
            SVTemporalUnit::Minute => Some(60_000.0),
            SVTemporalUnit::Hour => Some(3_600_000.0),
            SVTemporalUnit::Day => Some(86_400_000.0),
            SVTemporalUnit::Week => Some(604_800_000.0),
            SVTemporalUnit::Month => Some(2_629_800_000.0),
            SVTemporalUnit::Year => Some(31_557_600_000.0),
        }
    }
}

/// A time axis.
///
/// Timestamps are epoch milliseconds and interpolate numerically, so the axis
/// re-ticks as the domain moves. The scale wraps a continuous scale rather than
/// duplicating its projection: the only temporal knowledge it adds is the unit
/// of the tick ladder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVTemporalScale {
    continuous: SVContinuousScale,
    unit: SVTemporalUnit,
}

impl SVTemporalScale {
    /// Assembles a validated temporal scale.
    pub(crate) fn new(continuous: SVContinuousScale, unit: SVTemporalUnit) -> Self {
        Self { continuous, unit }
    }

    /// Returns the kind of this scale.
    pub fn kind(&self) -> SVScaleKind {
        SVScaleKind::Temporal
    }

    /// Returns the timestamp domain, in epoch milliseconds.
    pub fn domain(&self) -> SVDomain {
        self.continuous.domain()
    }

    /// Returns where the domain came from.
    pub fn domain_source(&self) -> SVDomainSource {
        self.continuous.domain_source()
    }

    /// Returns the display interval.
    pub fn range(&self) -> SVScaleRange {
        self.continuous.range()
    }

    /// Returns the transform applied to the timestamp domain.
    ///
    /// A time axis interpolates linearly; the accessor exists so the axis layer
    /// can report it without matching on the kind.
    pub fn transform(&self) -> SVScaleTransform {
        self.continuous.transform()
    }

    /// Returns the requested tick unit.
    pub fn unit(&self) -> SVTemporalUnit {
        self.unit
    }

    /// Returns the explicit tick count, or `None` when the layout decides.
    pub fn tick_count(&self) -> Option<usize> {
        self.continuous.tick_count()
    }

    /// Places a timestamp on the range.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the timestamp is not
    /// finite.
    pub fn position(&self, timestamp: f64) -> Result<f64, SVError> {
        self.continuous.position(timestamp)
    }

    /// Returns the timestamp at a position on the range.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] when the position is not
    /// finite.
    pub fn inverse_position(&self, position: f64) -> Result<f64, SVError> {
        self.continuous.inverse_position(position)
    }
}
