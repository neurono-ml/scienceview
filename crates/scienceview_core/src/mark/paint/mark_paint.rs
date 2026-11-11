//! The paint of one primitive: fill, stroke, opacity and stroke attributes.

use crate::sv_mark::options::{SVDashPattern, SVLineCap, SVLineJoin};
use crate::sv_scale::SVColour;

/// How one primitive is painted.
///
/// Paint is discrete: a mark that changes a colour or an opacity interpolates
/// between two paints, but a shape or a dash pattern is swapped and never
/// tweened. Keeping the paint next to the shape instead of inside it is what
/// makes that distinction expressible (design D2).
#[derive(Clone, Debug, PartialEq)]
pub struct SVMarkPaint {
    fill: Option<SVColour>,
    stroke: Option<SVColour>,
    stroke_width: f64,
    opacity: f32,
    dash: SVDashPattern,
    join: SVLineJoin,
    cap: SVLineCap,
}

impl Default for SVMarkPaint {
    fn default() -> Self {
        Self {
            fill: None,
            stroke: None,
            stroke_width: 0.0,
            opacity: 1.0,
            dash: SVDashPattern::Solid,
            join: SVLineJoin::Round,
            cap: SVLineCap::Butt,
        }
    }
}

impl SVMarkPaint {
    /// Builds an unpainted primitive.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a filled primitive.
    pub fn filled(fill: SVColour) -> Self {
        Self {
            fill: Some(fill),
            ..Self::default()
        }
    }

    /// Builds a stroked primitive.
    pub fn stroked(stroke: SVColour, stroke_width: f64) -> Self {
        Self {
            stroke: Some(stroke),
            stroke_width: sv_sanitize_width(stroke_width),
            ..Self::default()
        }
    }

    /// Adds a fill.
    pub fn with_fill(mut self, fill: SVColour) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Adds a stroke.
    pub fn with_stroke(mut self, stroke: SVColour, stroke_width: f64) -> Self {
        self.stroke = Some(stroke);
        self.stroke_width = sv_sanitize_width(stroke_width);
        self
    }

    /// Sets the resolved opacity, clamped to `0.0..=1.0`.
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = sv_sanitize_opacity(opacity);
        self
    }

    /// Sets the dash pattern.
    pub fn with_dash(mut self, dash: SVDashPattern) -> Self {
        self.dash = dash;
        self
    }

    /// Sets the join style.
    pub fn with_join(mut self, join: SVLineJoin) -> Self {
        self.join = join;
        self
    }

    /// Sets the cap style.
    pub fn with_cap(mut self, cap: SVLineCap) -> Self {
        self.cap = cap;
        self
    }

    /// Returns the fill colour.
    pub fn fill(&self) -> Option<SVColour> {
        self.fill
    }

    /// Returns the stroke colour.
    pub fn stroke(&self) -> Option<SVColour> {
        self.stroke
    }

    /// Returns the stroke width in logical pixels.
    pub fn stroke_width(&self) -> f64 {
        self.stroke_width
    }

    /// Returns the opacity.
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Returns the dash pattern.
    pub fn dash(&self) -> &SVDashPattern {
        &self.dash
    }

    /// Returns the join style.
    pub fn join(&self) -> SVLineJoin {
        self.join
    }

    /// Returns the cap style.
    pub fn cap(&self) -> SVLineCap {
        self.cap
    }
}

fn sv_sanitize_width(width: f64) -> f64 {
    if width.is_finite() && width > 0.0 {
        width
    } else {
        0.0
    }
}

fn sv_sanitize_opacity(opacity: f32) -> f32 {
    if opacity.is_finite() {
        opacity.clamp(0.0, 1.0)
    } else {
        0.0
    }
}
