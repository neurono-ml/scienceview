//! Stack options.

/// How a stack arranges its group.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVStackMode {
    /// Every group starts at zero: positives grow up, negatives grow down.
    #[default]
    Full,
    /// The whole group is centred on zero after stacking.
    Center,
}

/// The option surface of the stack move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVStackOptions {
    mode: SVStackMode,
}

impl Default for SVStackOptions {
    fn default() -> Self {
        Self {
            mode: SVStackMode::Full,
        }
    }
}

impl SVStackOptions {
    /// Builds the default stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets how the group is arranged.
    pub fn mode(mut self, mode: SVStackMode) -> Self {
        self.mode = mode;
        self
    }

    /// Returns the arrangement mode.
    pub fn mode_value(&self) -> SVStackMode {
        self.mode
    }
}
