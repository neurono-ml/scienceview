//! Motion tokens: durations and easings, plus the reduced-motion declaration.
//!
//! Reduced motion is a theme concern, not a per-call flag: when the theme
//! declares it, every duration resolves to zero at the token boundary, so the
//! transition layer honours accessibility without knowing about it.

use std::time::Duration;

/// The motion half of a compiled theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVMotionTokens {
    pub(crate) durations: SVMotionDurations,
    pub(crate) easings: SVMotionEasings,
    pub(crate) reduced_motion: bool,
}

impl SVMotionTokens {
    pub fn durations(&self) -> &SVMotionDurations {
        &self.durations
    }

    pub fn easings(&self) -> &SVMotionEasings {
        &self.easings
    }

    pub fn reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Resolves one duration.
    pub fn duration(&self, which: SVMotionDuration) -> Duration {
        let milliseconds = match which {
            SVMotionDuration::Fast => self.durations.fast_milliseconds,
            SVMotionDuration::Normal => self.durations.normal_milliseconds,
            SVMotionDuration::Slow => self.durations.slow_milliseconds,
        };
        Duration::from_secs_f64(milliseconds / 1_000.0)
    }

    /// Resolves one easing.
    pub fn easing(&self, which: SVMotionEasing) -> SVTimingFunction {
        match which {
            SVMotionEasing::Standard => self.easings.standard,
            SVMotionEasing::Emphasized => self.easings.emphasized,
            SVMotionEasing::Decelerate => self.easings.decelerate,
        }
    }
}

/// The named durations a transition or animation may request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SVMotionDuration {
    Fast,
    Normal,
    Slow,
}

/// The named easings a transition or animation may request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SVMotionEasing {
    Standard,
    Emphasized,
    Decelerate,
}

/// The authored durations, in milliseconds, before reduced-motion collapse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVMotionDurations {
    pub(crate) fast_milliseconds: f64,
    pub(crate) normal_milliseconds: f64,
    pub(crate) slow_milliseconds: f64,
}

impl SVMotionDurations {
    pub fn fast_milliseconds(&self) -> f64 {
        self.fast_milliseconds
    }

    pub fn normal_milliseconds(&self) -> f64 {
        self.normal_milliseconds
    }

    pub fn slow_milliseconds(&self) -> f64 {
        self.slow_milliseconds
    }
}

/// The authored easings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SVMotionEasings {
    pub(crate) standard: SVTimingFunction,
    pub(crate) emphasized: SVTimingFunction,
    pub(crate) decelerate: SVTimingFunction,
}

impl SVMotionEasings {
    pub fn standard(&self) -> SVTimingFunction {
        self.standard
    }

    pub fn emphasized(&self) -> SVTimingFunction {
        self.emphasized
    }

    pub fn decelerate(&self) -> SVTimingFunction {
        self.decelerate
    }
}

/// A timing function a backend can evaluate: a named CSS curve or a cubic
/// Bézier with its four control components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SVTimingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier { x1: f64, y1: f64, x2: f64, y2: f64 },
}

impl SVTimingFunction {
    /// Evaluates the curve at `progress`, clamped to `0.0..=1.0`.
    ///
    /// The named CSS curves are evaluated as their standard cubic Bézier
    /// definitions, so a terminal or native backend can sample the same curve
    /// a browser applies declaratively.
    pub fn evaluate(&self, progress: f64) -> f64 {
        let progress = if progress.is_finite() {
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        };
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
