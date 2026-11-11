//! The documented tolerance policy of representation parity.
//!
//! # Policy
//!
//! - **Dense against sparse is exact.** A sparse view stores the same `f64`
//!   values at the same positions, so a kernel that treats an implicit zero as
//!   zero must produce bit-identical output. Any difference is a defect, not a
//!   rounding artefact.
//! - **Dense against streaming uses [`SVRepresentationTolerance::DEFAULT`].**
//!   A streaming view is a bounded window of the same values, so the results
//!   are still compared value by value; the small absolute and relative margin
//!   only absorbs the order in which a kernel accumulates the window.
//! - **The tolerance is part of the kernel's contract.** A kernel whose
//!   mathematics needs a wider margin raises it explicitly through
//!   [`crate::sv_data::SVRepresentationParityHarness::with_tolerance`] and records
//!   the reason in its Feature Description Record.

/// The margin within which two representation results count as equal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVRepresentationTolerance {
    absolute: f64,
    relative: f64,
}

impl SVRepresentationTolerance {
    /// No difference is accepted at all.
    pub const EXACT: Self = Self {
        absolute: 0.0,
        relative: 0.0,
    };

    /// The default margin of the streaming comparison.
    ///
    /// One part in a trillion, absolute and relative, which is far below the
    /// precision any rendered pixel can express.
    pub const DEFAULT: Self = Self {
        absolute: 1e-12,
        relative: 1e-12,
    };

    /// Builds a tolerance from an absolute and a relative margin.
    pub fn new(absolute: f64, relative: f64) -> Self {
        Self { absolute, relative }
    }

    /// Returns the absolute margin.
    pub fn absolute(&self) -> f64 {
        self.absolute
    }

    /// Returns the relative margin.
    pub fn relative(&self) -> f64 {
        self.relative
    }

    /// Returns `true` when two values agree within this tolerance.
    pub fn is_within_tolerance(&self, left: f64, right: f64) -> bool {
        if left == right {
            return true;
        }

        if !left.is_finite() || !right.is_finite() {
            return false;
        }

        let difference = (left - right).abs();
        let scale = left.abs().max(right.abs());
        difference <= self.absolute + self.relative * scale
    }
}

impl Default for SVRepresentationTolerance {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
#[path = "tolerance_tests.rs"]
mod tolerance_tests;
