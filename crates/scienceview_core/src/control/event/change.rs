//! What changed, with the previous and the current value.

use core::fmt;

use super::payload::{
    SVCustomPayload, SVPlaybackChange, SVRangeChange, SVSelectionChange, SVSeriesVisibilityChange,
    SVViewReset,
};
use crate::sv_control::{SVAxisRange, SVSelectionRegion};

/// What changed, with the previous and the current value.
#[derive(Clone, Debug, PartialEq)]
pub enum SVControlChange {
    /// An axis range changed.
    Range(SVRangeChange),
    /// A series became visible or hidden.
    SeriesVisibility(SVSeriesVisibilityChange),
    /// The selection changed.
    Selection(SVSelectionChange),
    /// The animation playback state changed.
    Playback(SVPlaybackChange),
    /// The visible domains were reset.
    Reset(SVViewReset),
    /// An application-defined payload travelled unchanged.
    Custom(SVCustomPayload),
}

impl fmt::Display for SVControlChange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range(change) => write!(
                formatter,
                "change=range;axis={:?};previous={};current={}",
                change.axis, change.previous, change.current
            ),
            Self::SeriesVisibility(change) => write!(
                formatter,
                "change=series;series={};previous={};current={}",
                change.series, change.previous, change.current
            ),
            Self::Selection(change) => {
                formatter.write_str("change=selection;previous=")?;
                sv_write_selection(formatter, change.previous)?;
                formatter.write_str(";current=")?;
                sv_write_selection(formatter, change.current)
            }
            Self::Playback(change) => write!(
                formatter,
                "change=playback;previous_state={};state={};previous_position={};position={};previous_speed={};speed={}",
                change.previous_state,
                change.state,
                change.previous_position_seconds,
                change.position_seconds,
                change.previous_speed,
                change.speed
            ),
            Self::Reset(change) => {
                formatter.write_str("change=reset;previous=")?;
                sv_write_ranges(formatter, &change.previous)?;
                formatter.write_str(";current=")?;
                sv_write_ranges(formatter, &change.current)
            }
            Self::Custom(payload) => write!(
                formatter,
                "change=custom;name={};payload={}",
                payload.name, payload.payload
            ),
        }
    }
}

fn sv_write_selection(
    formatter: &mut fmt::Formatter<'_>,
    selection: Option<SVSelectionRegion>,
) -> fmt::Result {
    match selection {
        Some(region) => write!(formatter, "{region}"),
        None => formatter.write_str("none"),
    }
}

fn sv_write_ranges(formatter: &mut fmt::Formatter<'_>, ranges: &[SVAxisRange]) -> fmt::Result {
    if ranges.is_empty() {
        return formatter.write_str("none");
    }
    for (position, range) in ranges.iter().enumerate() {
        if position > 0 {
            formatter.write_str("|")?;
        }
        write!(formatter, "{range}")?;
    }
    Ok(())
}
