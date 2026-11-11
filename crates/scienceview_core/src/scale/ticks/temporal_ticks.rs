//! The temporal tick ladder: readable time steps instead of arbitrary ones.

use super::derivation::sv_tick_count;
use super::{SVTick, SVTickFormat, SVTickSpace};
use crate::sv_scale::{SVTemporalScale, SVTemporalUnit};

/// The readable time steps, ordered from a millisecond to a year.
const TEMPORAL_LADDER: [(SVTemporalUnit, f64); 24] = [
    (SVTemporalUnit::Millisecond, 1.0),
    (SVTemporalUnit::Millisecond, 5.0),
    (SVTemporalUnit::Millisecond, 10.0),
    (SVTemporalUnit::Millisecond, 50.0),
    (SVTemporalUnit::Millisecond, 100.0),
    (SVTemporalUnit::Millisecond, 500.0),
    (SVTemporalUnit::Second, 1_000.0),
    (SVTemporalUnit::Second, 5_000.0),
    (SVTemporalUnit::Second, 15_000.0),
    (SVTemporalUnit::Second, 30_000.0),
    (SVTemporalUnit::Minute, 60_000.0),
    (SVTemporalUnit::Minute, 300_000.0),
    (SVTemporalUnit::Minute, 900_000.0),
    (SVTemporalUnit::Minute, 1_800_000.0),
    (SVTemporalUnit::Hour, 3_600_000.0),
    (SVTemporalUnit::Hour, 10_800_000.0),
    (SVTemporalUnit::Hour, 21_600_000.0),
    (SVTemporalUnit::Hour, 43_200_000.0),
    (SVTemporalUnit::Day, 86_400_000.0),
    (SVTemporalUnit::Day, 172_800_000.0),
    (SVTemporalUnit::Day, 604_800_000.0),
    (SVTemporalUnit::Month, 2_629_800_000.0),
    (SVTemporalUnit::Month, 7_889_400_000.0),
    (SVTemporalUnit::Year, 31_557_600_000.0),
];

/// Derives the ticks of a temporal axis.
pub(crate) fn sv_temporal_ticks(
    scale: &SVTemporalScale,
    space: SVTickSpace,
    format: SVTickFormat,
) -> Vec<SVTick> {
    let domain = scale.domain();
    let count = sv_tick_count(scale.tick_count(), space);
    let (step, unit) = sv_temporal_step(domain.length(), scale.unit(), count);
    if domain.is_degenerate() {
        return sv_temporal_tick(scale, domain.lower(), unit, format)
            .into_iter()
            .collect();
    }

    let start = (domain.lower() / step).ceil() * step;
    let mut ticks = Vec::new();
    let mut index = 0_usize;
    while index <= count.saturating_add(2) {
        let value = start + step * index as f64;
        if value > domain.upper() + step * 1e-9 {
            break;
        }
        if let Some(tick) = sv_temporal_tick(scale, value, unit, format) {
            ticks.push(tick);
        }
        index += 1;
    }

    if ticks.len() >= 2 {
        return ticks;
    }

    [domain.lower(), domain.upper()]
        .into_iter()
        .filter_map(|value| sv_temporal_tick(scale, value, unit, format))
        .collect()
}

/// Chooses the step and the unit the axis ticks at.
fn sv_temporal_step(span: f64, requested: SVTemporalUnit, count: usize) -> (f64, SVTemporalUnit) {
    let intervals = count.saturating_sub(1).max(1);
    let target = span / intervals as f64;

    if let Some(milliseconds) = requested.milliseconds() {
        let multiples = (target / milliseconds).round().max(1.0);
        return (milliseconds * multiples, requested);
    }

    for (unit, step) in TEMPORAL_LADDER {
        if step >= target {
            return (step, unit);
        }
    }

    let (unit, step) = TEMPORAL_LADDER[TEMPORAL_LADDER.len() - 1];
    (step, unit)
}

/// Builds one temporal tick, labelling it in the step's unit.
fn sv_temporal_tick(
    scale: &SVTemporalScale,
    value: f64,
    unit: SVTemporalUnit,
    format: SVTickFormat,
) -> Option<SVTick> {
    let position = scale.position(value).ok()?;
    let milliseconds = unit.milliseconds().unwrap_or(1.0);
    let label = format!("{} {}", format.format(value / milliseconds), unit.as_str());
    Some(SVTick::new(value, position, label))
}
