//! Build-time validation and column probing for the hexbin builder.

use super::SVHexbinAggregation;
use super::builder::SVHexbinPlotBuilder;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_stat::SVNormalisation;

/// Rejects every impossible option before any column is read.
pub(super) fn sv_validate_options(builder: &SVHexbinPlotBuilder<'_>) -> Result<(), SVError> {
    if builder.grid_size == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "the hexbin grid size must be at least one".to_string(),
        });
    }
    if builder.min_count == 0 {
        return Err(SVError::InvalidConfiguration {
            message: "the hexbin minimum count must be at least one".to_string(),
        });
    }
    if !builder.orientation.is_finite() {
        return Err(SVError::InvalidConfiguration {
            message: "the hexbin orientation must be a finite angle in radians".to_string(),
        });
    }
    if let Some((x_lower, x_upper, y_lower, y_upper)) = builder.extent {
        if !x_lower.is_finite()
            || !x_upper.is_finite()
            || !y_lower.is_finite()
            || !y_upper.is_finite()
        {
            return Err(SVError::InvalidConfiguration {
                message: "the hexbin extent bounds must be finite".to_string(),
            });
        }
        if x_lower >= x_upper || y_lower >= y_upper {
            return Err(SVError::InvalidConfiguration {
                message: "the hexbin extent needs lower bounds below the upper bounds".to_string(),
            });
        }
    }
    match (builder.aggregation, builder.value.as_deref()) {
        (SVHexbinAggregation::Mean, None) => {
            return Err(SVError::InvalidConfiguration {
                message: "mean aggregation requires a `value` column; call `value(\"<column>\")`"
                    .to_string(),
            });
        }
        (SVHexbinAggregation::Count, Some(_)) => {
            return Err(SVError::InvalidConfiguration {
                message: "a `value` column is only consumed by mean aggregation; call `aggregation(SVHexbinAggregation::Mean)` or drop `value`"
                    .to_string(),
            });
        }
        _ => {}
    }
    if builder.aggregation == SVHexbinAggregation::Mean
        && builder.normalisation != SVNormalisation::Count
    {
        return Err(SVError::InvalidConfiguration {
            message:
                "mean aggregation colours the mean itself; normalisation applies only to counts"
                    .to_string(),
        });
    }
    Ok(())
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected: Option<(usize, SVRepresentation)> = None;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let view = snapshot.column_view();
        match expected {
            None => expected = Some((view.length(), view.representation())),
            Some((length, _)) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some((_, seen)) => {
                expected = Some((view.length(), sv_dominant(seen, view.representation())));
            }
        }
    }
    Ok(expected.unwrap_or((0, SVRepresentation::Dense)))
}

fn sv_dominant(left: SVRepresentation, right: SVRepresentation) -> SVRepresentation {
    use SVRepresentation::{Dense, Sparse, Streaming};
    match (left, right) {
        (Streaming, _) | (_, Streaming) => Streaming,
        (Sparse, _) | (_, Sparse) => Sparse,
        (Dense, Dense) => Dense,
    }
}
