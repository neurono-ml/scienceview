//! The rollover policy of a bounded rolling window.

/// What a full window does when another sample arrives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVWindowOverflow {
    /// Evict the oldest sample so the newest one is always visible.
    DropOldest,
    /// Reject the newest sample so the window keeps its first samples.
    DropNewest,
}

/// The capacity and the overflow behaviour of a rolling window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SVWindowPolicy {
    maximum_number_of_points: usize,
    overflow: SVWindowOverflow,
}

impl SVWindowPolicy {
    /// Keeps the last `maximum_number_of_points` samples, dropping the oldest.
    pub fn drop_oldest(maximum_number_of_points: usize) -> Self {
        Self {
            maximum_number_of_points,
            overflow: SVWindowOverflow::DropOldest,
        }
    }

    /// Keeps the first `maximum_number_of_points` samples, dropping the newest.
    pub fn drop_newest(maximum_number_of_points: usize) -> Self {
        Self {
            maximum_number_of_points,
            overflow: SVWindowOverflow::DropNewest,
        }
    }

    /// Returns the maximum number of samples the window holds.
    pub fn maximum_number_of_points(&self) -> usize {
        self.maximum_number_of_points
    }

    /// Returns the overflow behaviour.
    pub fn overflow(&self) -> SVWindowOverflow {
        self.overflow
    }
}

impl Default for SVWindowPolicy {
    fn default() -> Self {
        Self::drop_oldest(1024)
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod policy_tests;
