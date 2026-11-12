//! The binning kernel.

use crate::sv_error::SVError;
use crate::sv_stat::bin_options::{SVBinOptions, SVNormalisation};
use crate::sv_stat::bin_result::SVBinResult;
use crate::sv_stat::input::SVStatInput;
use crate::sv_stat::observations::SVObservationValues;
use crate::sv_stat::stat_definition::SVStatResult;

/// The number of bins used when no boundary option is given.
const SV_DEFAULT_BIN_COUNT: usize = 30;

/// Applies the binning configuration to the input column.
pub(crate) fn sv_apply_bin(
    options: &SVBinOptions,
    input: &SVStatInput<'_>,
) -> Result<SVStatResult, SVError> {
    let observations = SVObservationValues::from_view(input.values());
    if observations.length() == 0 {
        return Err(SVError::EmptyInput { operation: "bin" });
    }
    observations.validate_finite("bin")?;
    let edges = sv_resolve_edges(options, &observations)?;
    let mut counts = vec![0.0; edges.len() - 1];

    let first_edge = edges[0];
    let last_edge = edges[edges.len() - 1];
    let mut add = |value: f64, weight: f64| {
        if value < first_edge || value > last_edge {
            return;
        }
        let position = if value == last_edge {
            edges.len() - 2
        } else {
            edges.partition_point(|edge| *edge <= value) - 1
        };
        counts[position] += weight;
    };

    observations.for_each_stored(|value| add(value, 1.0));
    let implicit_zeros = observations.implicit_zero_count() as f64;
    if implicit_zeros > 0.0 {
        add(0.0, implicit_zeros);
    }

    sv_normalise(&mut counts, &edges, options);

    Ok(SVStatResult::Bins(SVBinResult::new(
        edges,
        counts,
        options.normalisation(),
        options.is_cumulative(),
        observations.length(),
    )))
}

/// Resolves the edges from the explicit edges, the width, the count or the
/// default, in that order of precedence.
fn sv_resolve_edges(
    options: &SVBinOptions,
    observations: &SVObservationValues<'_>,
) -> Result<Vec<f64>, SVError> {
    if let Some(edges) = options.explicit_edges() {
        sv_validate_edges(edges)?;
        return Ok(edges.to_vec());
    }

    if options.requested_bin_width().is_some() && options.requested_bin_count().is_some() {
        return Err(SVError::InvalidConfiguration {
            message: "binning accepts a bin width or a bin count, not both".to_string(),
        });
    }

    let (low, high) = match options.explicit_range() {
        Some(range) => range,
        None => observations
            .minimum_and_maximum()
            .ok_or(SVError::EmptyInput { operation: "bin" })?,
    };

    if !low.is_finite() || !high.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: "the binning range must be finite".to_string(),
        });
    }
    if low > high {
        return Err(SVError::InvalidConfiguration {
            message: "the binning range starts after it ends".to_string(),
        });
    }

    let (low, high) = if low == high {
        if options.explicit_range().is_some() {
            return Err(SVError::InvalidConfiguration {
                message: "an explicit binning range must be non-degenerate".to_string(),
            });
        }
        (low, low + 1.0)
    } else {
        (low, high)
    };

    if let Some(width) = options.requested_bin_width() {
        if !width.is_finite() || width <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the bin width must be a positive finite number".to_string(),
            });
        }
        let bin_count = ((high - low) / width).ceil().max(1.0) as usize;
        return Ok((0..=bin_count)
            .map(|position| low + position as f64 * width)
            .collect());
    }

    let bin_count = options
        .requested_bin_count()
        .unwrap_or(SV_DEFAULT_BIN_COUNT);
    if bin_count == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "the bin count must be at least one".to_string(),
        });
    }

    let width = (high - low) / bin_count as f64;
    Ok((0..=bin_count)
        .map(|position| low + position as f64 * width)
        .collect())
}

/// Validates explicit edges once, before any observation is read.
fn sv_validate_edges(edges: &[f64]) -> Result<(), SVError> {
    if edges.len() < 2 {
        return Err(SVError::InvalidConfiguration {
            message: "at least two bin edges are required".to_string(),
        });
    }
    if edges.iter().any(|edge| !edge.is_finite()) {
        return Err(SVError::InvalidConfiguration {
            message: "bin edges must be finite".to_string(),
        });
    }
    if edges.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(SVError::InvalidConfiguration {
            message: "bin edges must be strictly increasing".to_string(),
        });
    }
    Ok(())
}

/// Applies the requested normalisation and the optional cumulative pass.
fn sv_normalise(counts: &mut [f64], edges: &[f64], options: &SVBinOptions) {
    let observed_total: f64 = counts.iter().sum();
    if observed_total > 0.0 {
        match options.normalisation() {
            SVNormalisation::Count => {}
            SVNormalisation::Probability => {
                counts.iter_mut().for_each(|count| *count /= observed_total);
            }
            SVNormalisation::Density => counts.iter_mut().enumerate().for_each(|(index, count)| {
                *count /= observed_total * (edges[index + 1] - edges[index])
            }),
        }
    }

    if options.is_cumulative() {
        let mut running = 0.0;
        counts.iter_mut().for_each(|count| {
            running += *count;
            *count = running;
        });
    }
}
