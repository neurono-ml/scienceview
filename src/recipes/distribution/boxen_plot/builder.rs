//! The `SVBoxenPlot` builder and its validating state machine.

use scienceview_core::sv_backend::{SVBackendKind, sv_verify_mark_supported};
use scienceview_core::sv_data::SVDataSource;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use scienceview_core::sv_layout::SVViewport;
use scienceview_core::sv_mark::{SVBarOrientation, SVDotShape, SVMark, SVMarkKind};

use super::configuration::{SVBoxenPlotConfiguration, sv_invalid};
use super::core_implementation::SVBoxenPlot;
use super::data_contract::sv_validate_columns;
use super::depth_rule::SVBoxenTailRule;

/// Builds an [`SVBoxenPlot`].
///
/// The builder is the only public constructor. `backend(...)` is required —
/// there is no automatic backend — while `execution_mode(...)` defaults to
/// [`SVExecutionMode::Automatic`]. Every option is validated in
/// [`build`](SVBoxenPlotBuilder::build), so a boxen plot that exists can render.
#[derive(Clone)]
pub struct SVBoxenPlotBuilder<'values> {
    data_source: Option<&'values dyn SVDataSource>,
    value: Option<&'values str>,
    category: Option<&'values str>,
    series: Option<&'values str>,
    depth: Option<usize>,
    tail_rule: SVBoxenTailRule,
    show_median: bool,
    show_outliers: bool,
    outlier_marker: SVDotShape,
    width: f32,
    orientation: SVBarOrientation,
    title: Option<&'values str>,
    x_title: Option<&'values str>,
    y_title: Option<&'values str>,
    viewport: SVViewport,
    backend_kind: Option<SVBackendKind>,
    execution_mode: SVExecutionMode,
}

impl<'values> SVBoxenPlotBuilder<'values> {
    /// Starts a boxen plot with no data source, no columns and no backend.
    pub fn new() -> Self {
        Self {
            data_source: None,
            value: None,
            category: None,
            series: None,
            depth: None,
            tail_rule: SVBoxenTailRule::default(),
            show_median: true,
            show_outliers: false,
            outlier_marker: SVDotShape::Circle,
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

    /// Names the optional grouping column: one nest per series in a category.
    #[must_use]
    pub fn series(mut self, series: &'values str) -> Self {
        self.series = Some(series);
        self
    }

    /// Forces an explicit nesting depth.
    ///
    /// When omitted, [`SVBoxenTailRule`] derives the depth from the sample
    /// size. A depth of `k` produces `2k - 1` nested boxes.
    #[must_use]
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    /// Selects how the nesting depth is derived (default
    /// [`SVBoxenTailRule::Tukey`], Henderson's rule).
    #[must_use]
    pub fn tail_rule(mut self, tail_rule: SVBoxenTailRule) -> Self {
        self.tail_rule = tail_rule;
        self
    }

    /// Draws the median line across the central box (default `true`).
    #[must_use]
    pub fn show_median(mut self, show_median: bool) -> Self {
        self.show_median = show_median;
        self
    }

    /// Draws the observations outside the outermost letter values (default
    /// `false`).
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

    /// Sets the widest box width as a fraction of the category slot
    /// (default `0.8`).
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

    /// Validates the configuration and builds the boxen plot.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::MissingDataSource`], [`SVError::MissingBackendSelection`]
    /// or a configuration error naming the missing column, the invalid depth,
    /// the invalid tail proportion or the invalid width.
    pub fn build(self) -> Result<SVBoxenPlot<'values>, SVError> {
        let data_source = self.data_source.ok_or(SVError::MissingDataSource)?;
        let backend_kind = self.backend_kind.ok_or(SVError::MissingBackendSelection)?;
        let value = self
            .value
            .ok_or_else(|| sv_invalid("a boxen plot needs the `value` column"))?;
        let category = self
            .category
            .ok_or_else(|| sv_invalid("a boxen plot needs the `category` column"))?;
        sv_validate_depth(self.depth)?;
        sv_validate_tail_rule(self.tail_rule)?;
        if !self.width.is_finite() || self.width <= 0.0 || self.width > 1.0 {
            return Err(sv_invalid(
                "the boxen width must be finite and lie in 0.0..=1.0",
            ));
        }
        for mark_kind in [SVMarkKind::Ribbon, SVMarkKind::Line, SVMarkKind::Dot] {
            sv_verify_mark_supported(mark_kind, backend_kind)?;
        }
        let dot: SVMark = SVMark::dot().shape(self.outlier_marker).into();
        dot.validate()?;

        let configuration = SVBoxenPlotConfiguration {
            data_source,
            value,
            category,
            series: self.series,
            depth: self.depth,
            tail_rule: self.tail_rule,
            show_median: self.show_median,
            show_outliers: self.show_outliers,
            outlier_marker: self.outlier_marker,
            width: self.width,
            orientation: self.orientation,
            title: self.title,
            x_title: self.x_title,
            y_title: self.y_title,
            viewport: self.viewport,
        };
        sv_validate_columns(&configuration)?;

        Ok(SVBoxenPlot::from_configuration(
            configuration,
            backend_kind,
            self.execution_mode,
        ))
    }
}

impl Default for SVBoxenPlotBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

fn sv_validate_depth(depth: Option<usize>) -> Result<(), SVError> {
    match depth {
        Some(0) => Err(sv_invalid(
            "the boxen depth must be at least one level; the derived depth is one when the sample is too small",
        )),
        _ => Ok(()),
    }
}

fn sv_validate_tail_rule(tail_rule: SVBoxenTailRule) -> Result<(), SVError> {
    match tail_rule {
        SVBoxenTailRule::Tukey | SVBoxenTailRule::Full => Ok(()),
        SVBoxenTailRule::Proportion(proportion) => {
            if proportion.is_finite() && proportion > 0.0 && proportion < 1.0 {
                Ok(())
            } else {
                Err(sv_invalid(
                    "the boxen tail proportion must be finite and lie strictly inside (0, 1)",
                ))
            }
        }
    }
}
