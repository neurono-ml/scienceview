//! Configuration validation and the shared line options.

use super::definition::SVStepPlot;
use scienceview_core::sv_data::{SVColumnRole, SVDataSource};
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_mark::{SVLineOptions, SVMark};
use scienceview_core::sv_scale::SVScaleKind;
use scienceview_core::sv_stat::SVStat;

impl SVStepPlot<'_> {
    pub(crate) fn validate(&self) -> Result<(), SVError> {
        if !matches!(
            self.x_scale_kind,
            SVScaleKind::Continuous | SVScaleKind::Temporal
        ) {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the x axis of SVStepPlot is continuous or temporal, received `{}`",
                    self.x_scale_kind
                ),
            });
        }
        if let Some(stat) = &self.aggregate {
            if !matches!(stat, SVStat::Aggregate(_)) {
                return Err(SVError::InvalidConfiguration {
                    message: format!(
                        "SVStepPlot collapses duplicate x values with `SVStat::Aggregate`, received `{}`",
                        stat.kind().as_str()
                    ),
                });
            }
        }
        SVMark::Line(self.line_options()).validate()?;

        sv_validate_column(self.data_source, &self.x, "x", false)?;
        sv_validate_column(self.data_source, &self.y, "y", false)?;
        if let Some(series) = &self.series {
            sv_validate_column(self.data_source, series, "series", true)?;
        }

        let expected = sv_column_length(self.data_source, &self.x)?;
        let actual = sv_column_length(self.data_source, &self.y)?;
        if actual != expected {
            return Err(SVError::MismatchedLengths {
                name: self.y.clone(),
                expected,
                actual,
            });
        }
        if let Some(series) = &self.series {
            let actual = sv_column_length(self.data_source, series)?;
            if actual != expected {
                return Err(SVError::MismatchedLengths {
                    name: series.clone(),
                    expected,
                    actual,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn line_options(&self) -> SVLineOptions {
        let mut options = SVLineOptions::new();
        if let Some(width) = self.line_width {
            options = options.width(width);
        }
        if let Some(dash) = &self.dash {
            options = options.dash(dash.clone());
        }
        options
    }
}

fn sv_validate_column(
    data_source: &dyn SVDataSource,
    name: &str,
    channel: &str,
    allow_labels: bool,
) -> Result<(), SVError> {
    let schema = data_source.schema();
    if let Some(descriptor) = schema.column(name) {
        if !allow_labels && descriptor.role() == SVColumnRole::Label {
            return Err(SVError::InvalidConfiguration {
                message: format!(
                    "the {channel} column `{name}` carries labels; supply a numeric column instead"
                ),
            });
        }
    }
    data_source.column(name)?;
    Ok(())
}

fn sv_column_length(data_source: &dyn SVDataSource, name: &str) -> Result<usize, SVError> {
    let mut snapshot = data_source.column(name)?;
    Ok(snapshot.column_view().length())
}
