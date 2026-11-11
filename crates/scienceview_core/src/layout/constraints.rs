//! Minimum and maximum bounds on a resolved size.
//!
//! Constraints are total: a non-finite or negative bound is normalized before
//! use, and a contradictory pair (minimum greater than maximum) is resolved
//! deterministically in favour of the minimum.

/// The minimum and maximum sizes a layout node accepts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVLayoutConstraints {
    minimum_width: f64,
    maximum_width: f64,
    minimum_height: f64,
    maximum_height: f64,
}

impl SVLayoutConstraints {
    /// Returns constraints that accept every non-negative size.
    pub const fn unbounded() -> Self {
        Self {
            minimum_width: 0.0,
            maximum_width: f64::INFINITY,
            minimum_height: 0.0,
            maximum_height: f64::INFINITY,
        }
    }

    /// Sets the minimum width; a non-finite or negative value becomes zero.
    pub fn with_minimum_width(mut self, minimum_width: f64) -> Self {
        self.minimum_width = sv_sanitize_bound(minimum_width);
        self
    }

    /// Sets the maximum width; a non-finite value means unbounded.
    pub fn with_maximum_width(mut self, maximum_width: f64) -> Self {
        self.maximum_width = sv_sanitize_ceiling(maximum_width);
        self
    }

    /// Sets the minimum height; a non-finite or negative value becomes zero.
    pub fn with_minimum_height(mut self, minimum_height: f64) -> Self {
        self.minimum_height = sv_sanitize_bound(minimum_height);
        self
    }

    /// Sets the maximum height; a non-finite value means unbounded.
    pub fn with_maximum_height(mut self, maximum_height: f64) -> Self {
        self.maximum_height = sv_sanitize_ceiling(maximum_height);
        self
    }

    /// Returns the minimum width.
    pub const fn minimum_width(&self) -> f64 {
        self.minimum_width
    }

    /// Returns the maximum width, possibly infinite.
    pub const fn maximum_width(&self) -> f64 {
        self.maximum_width
    }

    /// Returns the minimum height.
    pub const fn minimum_height(&self) -> f64 {
        self.minimum_height
    }

    /// Returns the maximum height, possibly infinite.
    pub const fn maximum_height(&self) -> f64 {
        self.maximum_height
    }

    /// Clamps a candidate width into the configured bounds.
    ///
    /// When the minimum exceeds the maximum the minimum wins, so a
    /// contradictory pair never produces a size outside the declared floor.
    pub fn clamp_width(&self, width: f64) -> f64 {
        sv_clamp(width, self.minimum_width, self.maximum_width)
    }

    /// Clamps a candidate height into the configured bounds.
    pub fn clamp_height(&self, height: f64) -> f64 {
        sv_clamp(height, self.minimum_height, self.maximum_height)
    }
}

impl Default for SVLayoutConstraints {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Normalizes a lower bound: non-finite and negative values become zero.
fn sv_sanitize_bound(bound: f64) -> f64 {
    if bound.is_finite() && bound > 0.0 {
        bound
    } else {
        0.0
    }
}

/// Normalizes an upper bound: non-finite values mean unbounded.
fn sv_sanitize_ceiling(ceiling: f64) -> f64 {
    if ceiling.is_finite() {
        ceiling.max(0.0)
    } else {
        f64::INFINITY
    }
}

/// Clamps `value` with a deterministic precedence: the minimum wins.
fn sv_clamp(value: f64, minimum: f64, maximum: f64) -> f64 {
    let value = if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    };
    if minimum > maximum {
        minimum
    } else {
        value.max(minimum).min(maximum)
    }
}
