//! Public option values of the parallel-coordinates plot.

/// How each axis derives its value domain.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVParallelCoordinatesNormalization {
    /// Every axis maps its own minimum and maximum onto the full plot height.
    #[default]
    PerAxis,
    /// Every axis shares one domain derived from all visible variables.
    Shared,
}

impl SVParallelCoordinatesNormalization {
    /// Returns the rule name as written in tests and observability.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVParallelCoordinatesNormalization::PerAxis => "PerAxis",
            SVParallelCoordinatesNormalization::Shared => "Shared",
        }
    }
}

/// What happens to a non-finite value at one axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVParallelCoordinatesMissingPolicy {
    /// Break the polyline at the missing axis; each continuous run is drawn
    /// separately (default).
    #[default]
    Gap,
    /// Skip the missing axis and join its neighbours with one straight segment.
    Bridge,
    /// Drop the whole observation when any visible axis is missing.
    DropObservation,
    /// Reject the operation, naming the observation.
    Error,
}

impl SVParallelCoordinatesMissingPolicy {
    /// Returns the policy name as written in tests and observability.
    pub fn as_str(&self) -> &'static str {
        match self {
            SVParallelCoordinatesMissingPolicy::Gap => "Gap",
            SVParallelCoordinatesMissingPolicy::Bridge => "Bridge",
            SVParallelCoordinatesMissingPolicy::DropObservation => "DropObservation",
            SVParallelCoordinatesMissingPolicy::Error => "Error",
        }
    }
}

/// One axis of a parallel-coordinates plot.
///
/// A dimension names one configured variable and overrides how its axis is
/// drawn: its domain, whether it is visible and whether its axis line is
/// tinted with the categorical palette. The order of the dimensions is the
/// axis order.
#[derive(Clone, Debug, PartialEq)]
pub struct SVDimension<'values> {
    variable: &'values str,
    range: Option<(f64, f64)>,
    visible: bool,
    coloured: bool,
}

impl<'values> SVDimension<'values> {
    /// Starts a visible, uncoloured dimension over one variable.
    pub fn new(variable: &'values str) -> Self {
        Self {
            variable,
            range: None,
            visible: true,
            coloured: false,
        }
    }

    /// Overrides the domain of this axis.
    #[must_use]
    pub fn range(mut self, lower: f64, upper: f64) -> Self {
        self.range = Some((lower, upper));
        self
    }

    /// Hides or shows the axis. A hidden axis is not drawn and its value is not
    /// part of any polyline.
    #[must_use]
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Tints the axis line and its label with the categorical palette.
    #[must_use]
    pub fn coloured(mut self, coloured: bool) -> Self {
        self.coloured = coloured;
        self
    }

    /// Returns the variable the dimension names.
    pub fn variable(&self) -> &'values str {
        self.variable
    }

    /// Returns the explicit range, when one is configured.
    pub fn range_override(&self) -> Option<(f64, f64)> {
        self.range
    }

    /// Returns whether the axis is drawn.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Returns whether the axis is tinted with the categorical palette.
    pub fn is_coloured(&self) -> bool {
        self.coloured
    }
}
