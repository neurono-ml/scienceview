//! The easing curves a transition may apply to its progress.

use core::fmt;

/// An easing curve expressed as a cubic Bézier, using the CSS definitions for
/// the named curves.
///
/// The kernel takes no dependency for the curve: the Bézier is solved in-crate
/// by bisection, exactly like the theme's timing function, so a terminal or
/// native backend samples the same curve a browser applies declaratively and the
/// result is identical across platforms.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVEasing {
    /// Constant speed.
    Linear,
    /// The CSS `ease` curve.
    Ease,
    /// Slow start.
    EaseIn,
    /// Slow end.
    EaseOut,
    /// Slow start and slow end (the transition default).
    EaseInOut,
    /// A caller-defined cubic Bézier.
    CubicBezier {
        /// The horizontal component of the first control point.
        x1: f64,
        /// The vertical component of the first control point.
        y1: f64,
        /// The horizontal component of the second control point.
        x2: f64,
        /// The vertical component of the second control point.
        y2: f64,
    },
}

impl Default for SVEasing {
    fn default() -> Self {
        Self::EaseInOut
    }
}

impl SVEasing {
    /// Evaluates the curve at `progress`, clamped to `0.0..=1.0`.
    ///
    /// A linear curve returns the progress itself, so an interpolation that
    /// uses it is exact; a named curve is evaluated as its standard cubic
    /// Bézier definition, which returns exactly `0.0` at the start and `1.0` at
    /// the end.
    pub fn evaluate(&self, progress: f64) -> f64 {
        let progress = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };
        if matches!(self, Self::Linear) {
            return progress;
        }

        let (x1, y1, x2, y2) = self.control_components();
        sv_evaluate_cubic_bezier(x1, y1, x2, y2, progress)
    }

    /// The four control components, using the CSS definitions for the named
    /// curves.
    pub fn control_components(&self) -> (f64, f64, f64, f64) {
        match self {
            Self::Linear => (0.0, 0.0, 1.0, 1.0),
            Self::Ease => (0.25, 0.1, 0.25, 1.0),
            Self::EaseIn => (0.42, 0.0, 1.0, 1.0),
            Self::EaseOut => (0.0, 0.0, 0.58, 1.0),
            Self::EaseInOut => (0.42, 0.0, 0.58, 1.0),
            Self::CubicBezier { x1, y1, x2, y2 } => (*x1, *y1, *x2, *y2),
        }
    }

    /// Returns the curve name as written in observability spans.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linear => "linear",
            Self::Ease => "ease",
            Self::EaseIn => "ease-in",
            Self::EaseOut => "ease-out",
            Self::EaseInOut => "ease-in-out",
            Self::CubicBezier { .. } => "cubic-bezier",
        }
    }
}

impl fmt::Display for SVEasing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(feature = "theme")]
impl From<scienceview_theme::SVTimingFunction> for SVEasing {
    fn from(timing_function: scienceview_theme::SVTimingFunction) -> Self {
        match timing_function {
            scienceview_theme::SVTimingFunction::Linear => Self::Linear,
            scienceview_theme::SVTimingFunction::Ease => Self::Ease,
            scienceview_theme::SVTimingFunction::EaseIn => Self::EaseIn,
            scienceview_theme::SVTimingFunction::EaseOut => Self::EaseOut,
            scienceview_theme::SVTimingFunction::EaseInOut => Self::EaseInOut,
            scienceview_theme::SVTimingFunction::CubicBezier { x1, y1, x2, y2 } => {
                Self::CubicBezier { x1, y1, x2, y2 }
            }
        }
    }
}

/// Solves the cubic Bézier for `y` at a given `x` by bisection, which is
/// deterministic and needs no dependency.
fn sv_evaluate_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
    let sample = |parameter: f64, first: f64, second: f64| -> f64 {
        let inverse = 1.0 - parameter;
        3.0 * inverse * inverse * parameter * first
            + 3.0 * inverse * parameter * parameter * second
            + parameter * parameter * parameter
    };
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let mut lower = 0.0;
    let mut upper = 1.0;
    for _ in 0..48 {
        let middle = (lower + upper) / 2.0;
        if sample(middle, x1, x2) < x {
            lower = middle;
        } else {
            upper = middle;
        }
    }
    sample((lower + upper) / 2.0, y1, y2)
}
