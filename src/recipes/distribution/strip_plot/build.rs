//! The single validation point of the `SVStripPlot` builder.

use super::builder::SVStripPlotBuilder;
use super::configuration::SVStripConfiguration;
use super::core_implementation::SVStripPlot;
use super::data_contract::sv_validate_columns;
use crate::sv_backend::sv_verify_mark_supported;
use crate::sv_data::SVRepresentation;
use crate::sv_error::SVError;
use crate::sv_mark::{SVMark, SVMarkKind};
use crate::sv_move::SVMove;

/// Validates and builds the strip plot.
pub(super) fn sv_build_strip_plot(
    builder: SVStripPlotBuilder<'_>,
) -> Result<SVStripPlot<'_>, SVError> {
    let data_source = builder.data_source.ok_or(SVError::MissingDataSource)?;
    let backend_kind = builder
        .backend_kind
        .ok_or(SVError::MissingBackendSelection)?;
    let value = sv_required_column(builder.value, "value")?;
    let category = sv_required_column(builder.category, "category")?;

    let mut dot = SVMark::dot().shape(builder.marker).opacity(builder.alpha);
    if let Some(size) = builder.size {
        dot = dot.size(size);
    }
    let mark: SVMark = dot.into();
    mark.validate()?;
    sv_verify_mark_supported(SVMarkKind::Dot, backend_kind)?;

    sv_validate_jitter(builder.jitter, builder.seed, builder.jitter_mode)?;
    sv_validate_columns(data_source, &value, &category, builder.series.as_deref())?;

    let (number_of_observations, representation) =
        sv_probe_columns(data_source, &value, &category, builder.series.as_deref())?;

    Ok(SVStripPlot {
        configuration: SVStripConfiguration {
            data_source,
            value,
            category,
            series: builder.series,
            series_labels: builder.series_labels,
            jitter: builder.jitter,
            jitter_mode: builder.jitter_mode,
            seed: builder.seed,
            marker: builder.marker,
            alpha: builder.alpha,
            size: builder.size,
            orientation: builder.orientation,
            missing_value_policy: builder.missing_value_policy,
            title: builder.title,
            value_label: builder.value_label,
            category_label: builder.category_label,
            backend_kind,
            execution_mode: builder.execution_mode,
            number_of_observations,
            representation,
        },
    })
}

fn sv_required_column(column: Option<String>, role: &str) -> Result<String, SVError> {
    column.ok_or_else(|| SVError::InvalidConfiguration {
        message: format!("SVStripPlot requires the `{role}` column; pass it with `{role}(...)`"),
    })
}

fn sv_validate_jitter(
    jitter: f32,
    seed: Option<u64>,
    jitter_mode: crate::sv_move::SVJitterMode,
) -> Result<(), SVError> {
    let valid = jitter.is_finite() && (0.0..=1.0).contains(&jitter);
    if !valid {
        return Err(SVError::InvalidConfiguration {
            message: format!(
                "the strip jitter is a finite fraction of the category slot between 0.0 and 1.0, received {jitter}"
            ),
        });
    }
    let mut options = SVMove::jitter().amount(jitter).mode(jitter_mode);
    if let Some(seed) = seed {
        options = options.seed(seed);
    }
    SVMove::from(options).validate()
}

/// Probes the configured columns once, so the figure carries its length and
/// representation before any operation runs.
fn sv_probe_columns(
    data_source: &dyn crate::sv_data::SVDataSource,
    value: &str,
    category: &str,
    series: Option<&str>,
) -> Result<(usize, SVRepresentation), SVError> {
    let mut value_snapshot = data_source.column(value)?;
    let mut category_snapshot = data_source.column(category)?;
    let mut series_snapshot = series.map(|name| data_source.column(name)).transpose()?;

    let value_view = value_snapshot.column_view();
    let category_view = category_snapshot.column_view();
    let number_of_observations = value_view.length();
    sv_check_length("value", number_of_observations, "category", category_view)?;
    let mut representation = sv_widest(value_view.representation(), category_view.representation());
    if let Some(series_snapshot) = series_snapshot.as_mut() {
        let series_view = series_snapshot.column_view();
        sv_check_length("value", number_of_observations, "series", series_view)?;
        representation = sv_widest(representation, series_view.representation());
    }
    Ok((number_of_observations, representation))
}

fn sv_check_length(
    first_name: &str,
    expected: usize,
    second_name: &str,
    second: crate::sv_data::SVColumnView<'_>,
) -> Result<(), SVError> {
    if second.length() == expected {
        return Ok(());
    }
    Err(SVError::MismatchedLengths {
        name: format!("{second_name} disagrees with {first_name}"),
        expected,
        actual: second.length(),
    })
}

fn sv_widest(first: SVRepresentation, second: SVRepresentation) -> SVRepresentation {
    fn width(representation: SVRepresentation) -> u8 {
        match representation {
            SVRepresentation::Dense => 0,
            SVRepresentation::Sparse => 1,
            SVRepresentation::Streaming => 2,
        }
    }
    if width(second) > width(first) {
        second
    } else {
        first
    }
}
