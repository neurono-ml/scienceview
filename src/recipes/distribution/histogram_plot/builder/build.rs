//! The single validation point of the histogram builder.

use super::super::SVBarElement;
use super::super::SVHistogramMultiple;
use super::super::configuration::SVHistogramConfiguration;
use super::super::core_implementation::SVHistogramPlot;
use super::definition::SVHistogramPlotBuilder;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::{SVDataSource, SVRepresentation};
use crate::sv_error::SVError;
use crate::sv_mark::SVBarOrientation;
use crate::sv_mark::SVMarkKind;

impl<'data> SVHistogramPlotBuilder<'data> {
    /// Validates the configuration and builds the histogram.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`] or
    /// [`SVError::MissingBackendSelection`] for a required step, a
    /// configuration error for a missing value column, an impossible bin
    /// boundary, a dodge combined with a non-bar element or a stack combined
    /// with independent bins, [`SVError::ColumnNotFound`] for a column the
    /// source does not expose, [`SVError::MismatchedLengths`] when the columns
    /// disagree, and [`SVError::UnsupportedMark`] when the backend cannot draw
    /// the selected element.
    pub fn build(self) -> Result<SVHistogramPlot<'data>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self.value.ok_or_else(|| SVError::InvalidConfiguration {
            message: "SVHistogramPlot requires a `value` column; call `value(\"<column>\")`"
                .to_string(),
        })?;

        sv_validate_bin_options(self.bin_count, self.bin_width, self.bin_edges.as_deref())?;
        if !self.shrink.is_finite() || self.shrink <= 0.0 || self.shrink > 1.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the histogram shrink must be greater than zero and at most one"
                    .to_string(),
            });
        }
        if self.multiple == SVHistogramMultiple::Dodge && self.element != SVBarElement::Bar {
            return Err(SVError::InvalidConfiguration {
                message: "a dodge is a bar arrangement; the step and polygon elements draw one outline per series"
                    .to_string(),
            });
        }
        if self.multiple == SVHistogramMultiple::Stack && !self.common_bins {
            return Err(SVError::InvalidConfiguration {
                message: "stacking matches bins by index and therefore requires common bins; enable `common_bins(true)`"
                    .to_string(),
            });
        }
        if self.orientation == SVBarOrientation::Horizontal && self.element != SVBarElement::Bar {
            return Err(SVError::InvalidConfiguration {
                message: "the horizontal orientation is implemented for the bar element; the step and polygon elements draw vertical outlines"
                    .to_string(),
            });
        }

        let mark_kind = match self.element {
            SVBarElement::Bar => SVMarkKind::Bar,
            SVBarElement::Step => SVMarkKind::Line,
            SVBarElement::Polygon => SVMarkKind::Area,
        };
        sv_verify_mark_supported(mark_kind, backend_kind)?;

        let mut names = vec![value.as_str()];
        names.extend(self.series.as_deref());
        names.extend(self.weight.as_deref());
        let (number_of_observations, representation) = sv_probe_columns(data_source, &names)?;

        Ok(SVHistogramPlot {
            configuration: SVHistogramConfiguration {
                data_source,
                value,
                series: self.series,
                weight: self.weight,
                series_labels: self.series_labels,
                bin_count: self.bin_count,
                bin_width: self.bin_width,
                bin_edges: self.bin_edges,
                normalisation: self.normalisation,
                cumulative: self.cumulative,
                common_normalisation: self.common_normalisation,
                common_bins: self.common_bins,
                shrink: self.shrink,
                element: self.element,
                orientation: self.orientation,
                multiple: self.multiple,
                missing_value_policy: self.missing_value_policy,
                title: self.title,
                x_label: self.x_label,
                y_label: self.y_label,
                backend_kind,
                execution_mode: self.execution_mode,
                number_of_observations,
                representation,
            },
        })
    }
}

fn sv_validate_bin_options(
    bin_count: Option<usize>,
    bin_width: Option<f64>,
    bin_edges: Option<&[f64]>,
) -> Result<(), SVError> {
    if let Some(count) = bin_count {
        if count == 0 {
            return Err(SVError::InvalidConfiguration {
                message: "the histogram bin count must be at least one".to_string(),
            });
        }
    }
    if let Some(width) = bin_width {
        if !width.is_finite() || width <= 0.0 {
            return Err(SVError::InvalidConfiguration {
                message: "the histogram bin width must be a positive finite number".to_string(),
            });
        }
        if bin_count.is_some() {
            return Err(SVError::InvalidConfiguration {
                message: "a histogram accepts a bin width or a bin count, not both".to_string(),
            });
        }
    }
    if let Some(edges) = bin_edges {
        if edges.len() < 2 {
            return Err(SVError::InvalidConfiguration {
                message: "a histogram needs at least two bin edges".to_string(),
            });
        }
        if edges.iter().any(|edge| !edge.is_finite()) {
            return Err(SVError::InvalidConfiguration {
                message: "the histogram bin edges must be finite".to_string(),
            });
        }
        if edges.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(SVError::InvalidConfiguration {
                message: "the histogram bin edges must be strictly increasing".to_string(),
            });
        }
    }
    Ok(())
}

/// Probes the named columns once for existence and equal length.
pub(super) fn sv_probe_columns(
    data_source: &dyn SVDataSource,
    names: &[&str],
) -> Result<(usize, SVRepresentation), SVError> {
    let mut expected_length: Option<usize> = None;
    let mut representation = SVRepresentation::Dense;
    for name in names {
        let mut snapshot = data_source.column(name)?;
        let representation_of_column = snapshot.representation();
        let view = snapshot.column_view();
        match expected_length {
            None => expected_length = Some(view.length()),
            Some(length) if view.length() != length => {
                return Err(SVError::MismatchedLengths {
                    name: (*name).to_string(),
                    expected: length,
                    actual: view.length(),
                });
            }
            Some(_) => {}
        }
        representation = sv_combine_representation(representation, representation_of_column);
    }
    Ok((expected_length.unwrap_or(0), representation))
}

/// Chooses the strongest representation among two columns.
pub(super) fn sv_combine_representation(
    left: SVRepresentation,
    right: SVRepresentation,
) -> SVRepresentation {
    match (left, right) {
        (SVRepresentation::Streaming, _) | (_, SVRepresentation::Streaming) => {
            SVRepresentation::Streaming
        }
        (SVRepresentation::Sparse, _) | (_, SVRepresentation::Sparse) => SVRepresentation::Sparse,
        _ => SVRepresentation::Dense,
    }
}
