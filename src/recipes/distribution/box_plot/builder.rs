//! The `SVBoxPlot` builder and its validating state machine.

use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape, SVMark, SVMarkKind};

use super::configuration::{SVBoxPlotConfiguration, sv_invalid};
use super::core_implementation::SVBoxPlot;
use super::data_contract::sv_validate_columns;
use super::whisker::SVWhiskerRule;

/// Builds an [`SVBoxPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVBoxPlotBuilder::build), so a box plot that exists can render.
#[derive(Clone)]
pub struct SVBoxPlotBuilder<'values> {
    data_source: Option<&'values dyn SVDataSource>,
    value: Option<&'values str>,
    category: Option<&'values str>,
    series: Option<&'values str>,
    quartiles: Vec<f64>,
    whisker: SVWhiskerRule,
    show_outliers: bool,
    outlier_marker: SVDotShape,
    notch: bool,
    show_mean: bool,
    width: f32,
    orientation: SVBarOrientation,
    title: Option<&'values str>,
    x_title: Option<&'values str>,
    y_title: Option<&'values str>,
    viewport: SVViewport,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
}

impl<'values> SVBoxPlotBuilder<'values> {
    /// Starts a box plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            category: None,
            series: None,
            quartiles: vec![0.25, 0.5, 0.75],
            whisker: SVWhiskerRule::default(),
            show_outliers: true,
            outlier_marker: SVDotShape::Circle,
            notch: false,
            show_mean: false,
            width: 0.8,
            orientation: SVBarOrientation::Vertical,
            title: None,
            x_title: None,
            y_title: None,
            viewport: SVViewport::new(640.0, 400.0),
            backend_kind: None,
            execution_mode: SVExecutionMode::Automatic,
        }
    }

    /// Attaches the data source.
    #[must_use]
    pub fn data_source(mut self, data_source: &'values dyn SVDataSource) -> Self {
        self.data_source = Some(data_source);
        self
    }

    /// Names the observed numeric column.
    #[must_use]
    pub fn value(mut self, value: &'values str) -> Self {
        self.value = Some(value);
        self
    }

    /// Names the categorical axis column.
    #[must_use]
    pub fn category(mut self, category: &'values str) -> Self {
        self.category = Some(category);
        self
    }

    /// Names the optional grouping column: one box per series in a category.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Replaces the three requested quantiles (default `[0.25, 0.5, 0.75]`).
    #[must_use]
    pub fn quartiles(mut self, quartiles: Vec<f64>) -> Self {
        self.quartiles = quartiles;
        self
    }

    /// Selects how the whisker ends are chosen (default [`SVWhiskerRule::Tukey`]).
    #[must_use]
    pub fn whisker(mut self, whisker: SVWhiskerRule) -> Self {
        self.whisker = whisker;
        self
    }

    /// Selects a percentile whisker rule from a lower percentile such as `0.05`.
    #[must_use]
    pub fn whisker_range(mut self, percentile: f64) -> Self {
        self.whisker = SVWhiskerRule::Percentile(percentile);
        self
    }

    /// Draws the observations outside the whiskers (default `true`).
    #[must_use]
    pub fn show_outliers(mut self, show_outliers: bool) -> Self {
        self.show_outliers = show_outliers;
        self
    }

    /// Chooses the marker drawn for each outlier (default
    /// [`SVDotShape::Circle`]).
    #[must_use]
    pub fn outlier_marker(mut self, outlier_marker: SVDotShape) -> Self {
        self.outlier_marker = outlier_marker;
        self
    }

    /// Draws a notched box around the median confidence interval (default
    /// `false`).
    #[must_use]
    pub fn notch(mut self, notch: bool) -> Self {
        self.notch = notch;
        self
    }

    /// Draws a mean marker (default `false`).
    #[must_use]
    pub fn show_mean(mut self, show_mean: bool) -> Self {
        self.show_mean = show_mean;
        self
    }

    /// Sets the box width as a fraction of the category slot (default `0.8`).
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Chooses the value orientation (default
    /// [`SVBarOrientation::Vertical`]).
    #[must_use]
    pub fn orient(mut self, orientation: SVBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the figure title.
    #[must_use]
    pub fn title(mut self, title: &'values str) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the title of the categorical axis.
    #[must_use]
    pub fn x_title(mut self, x_title: &'values str) -> Self {
        self.x_title = Some(x_title);
        self
    }

    /// Sets the title of the value axis.
    #[must_use]
    pub fn y_title(mut self, y_title: &'values str) -> Self {
        self.y_title = Some(y_title);
        self
    }

    /// Sets the logical viewport.
    #[must_use]
    pub fn viewport(mut self, viewport: SVViewport) -> Self {
        self.viewport = viewport;
        self
    }

    /// Selects the rendering backend, which is required.
    #[must_use]
    pub fn backend(mut self, backend_kind: SVBackendKind) -> Self {
        self.backend_kind = Some(backend_kind);
        self
    }

    /// Sets the execution mode (default `Automatic`).
    #[must_use]
    pub fn execution_mode(mut self, execution_mode: SVExecutionMode) -> Self {
        self.execution_mode = execution_mode;
        self
    }

    /// Validates the configuration and builds the box plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`]
    /// or a configuration error naming the missing column, the invalid
    /// quantiles, the invalid whisker percentile or the invalid width.
    pub fn build(self) -> Result<SVBoxPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a box plot needs the `value` column"))?;
        let category = self
            .category
            .ok_or_else(|| sv_invalid("a box plot needs the `category` column"))?;
        sv_validate_quartiles(&self.quartiles)?;
        sv_validate_whisker(self.whisker)?;
        if !self.width.is_finite() || self.width <= 0.0 || self.width > 1.0 {
            return Err(sv_invalid(
                "the box width must be finite and lie in 0.0..=1.0",
            ));
        }
        for mark_kind in [SVMarkKind::Ribbon, SVMarkKind::Line, SVMarkKind::Dot] {
            sv_verify_mark_supported(mark_kind, backend_kind)?;
        }
        let dot: SVMark = SVMark::dot().shape(self.outlier_marker).into();
        dot.validate()?;

        let configuration = SVBoxPlotConfiguration {
            data_source,
            value,
            category,
            series: self.series,
            quartiles: self.quartiles,
            whisker: self.whisker,
            show_outliers: self.show_outliers,
            outlier_marker: self.outlier_marker,
            notch: self.notch,
            show_mean: self.show_mean,
            width: self.width,
            orientation: self.orientation,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };
        sv_validate_columns(&configuration)?;

        Ok(SVBoxPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

impl Default for SVBoxPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_validate_quartiles(quartiles: &[f64]) -> Result<(), SVError> {
    if quartiles.len() != 3 {
        return Err(sv_invalid(
            "a box plot needs exactly three quantiles: lower, median and upper",
        ));
    }
    if quartiles
        .iter()
        .any(|quantile| !quantile.is_finite() || !(0.0..1.0).contains(quantile))
    {
        return Err(sv_invalid(
            "every box-plot quantile must be finite and lie strictly inside (0, 1)",
        ));
    }
    if !(quartiles[0] < quartiles[1] && quartiles[1] < quartiles[2]) {
        return Err(sv_invalid(
            "the box-plot quantiles must be strictly increasing",
        ));
    }
    Ok(())
}

fn sv_validate_whisker(whisker: SVWhiskerRule) -> Result<(), SVError> {
    if let Some(percentile) = whisker.percentile_value() {
        if !percentile.is_finite() || !(0.0..0.5).contains(&percentile) {
            return Err(sv_invalid(
                "the percentile whisker rule needs a percentile strictly inside (0, 0.5)",
            ));
        }
    }
    Ok(())
}
