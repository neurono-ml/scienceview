//! The closed set of plot kinds the pure conversion layer can build.

use scienceview::sv_error::SVError;

/// A plot kind the conversion layer knows how to build.
///
/// The kind is a runtime value, exactly like the backend kind: the Python
/// surface passes a string, the conversion rejects an unknown name before any
/// builder is touched, and the catalogue of options is looked up by kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVPythonPlotKind {
    /// The backend-agnostic figure builder, with no recipe on top.
    Figure,
    /// `SVScatterPlot`.
    ScatterPlot,
    /// `SVLinePlot`.
    LinePlot,
    /// `SVAreaPlot`.
    AreaPlot,
    /// `SVHistogramPlot`.
    HistogramPlot,
}

impl SVPythonPlotKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Figure,
        Self::ScatterPlot,
        Self::LinePlot,
        Self::AreaPlot,
        Self::HistogramPlot,
    ];

    /// The stable lowercase name used by the Python surface.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Figure => "figure",
            Self::ScatterPlot => "scatter",
            Self::LinePlot => "line",
            Self::AreaPlot => "area",
            Self::HistogramPlot => "histogram",
        }
    }

    /// Parses the name the Python surface accepts.
    ///
    /// # Errors
    ///
    /// Returns [`SVError::InvalidConfiguration`] naming the accepted kinds.
    pub fn parse(name: &str) -> Result<Self, SVError> {
        match name {
            "figure" => Ok(Self::Figure),
            "scatter" | "scatter_plot" => Ok(Self::ScatterPlot),
            "line" | "line_plot" => Ok(Self::LinePlot),
            "area" | "area_plot" => Ok(Self::AreaPlot),
            "histogram" | "histogram_plot" => Ok(Self::HistogramPlot),
            other => Err(SVError::InvalidConfiguration {
                message: format!(
                    "`{other}` is not a bound plot kind; the bound kinds are figure, scatter, line, area and histogram"
                ),
            }),
        }
    }
}
