//! Jitter options.

/// The distribution a jitter draws from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SVJitterMode {
    /// A flat distribution over the allowed band (default).
    #[default]
    Uniform,
    /// A bell-shaped distribution over the same band.
    Gaussian,
}

/// The option surface of the jitter move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SVJitterOptions {
    amount: f32,
    mode: SVJitterMode,
    seed: Option<u64>,
}

impl Default for SVJitterOptions {
    fn default() -> Self {
        Self {
            amount: 0.1,
            mode: SVJitterMode::Uniform,
            seed: None,
        }
    }
}

impl SVJitterOptions {
    /// Builds the default jitter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the amount as a fraction of the category width.
    pub fn amount(mut self, amount: f32) -> Self {
        self.amount = amount;
        self
    }

    /// Sets the distribution.
    pub fn mode(mut self, mode: SVJitterMode) -> Self {
        self.mode = mode;
        self
    }

    /// Fixes the seed, making the jitter reproducible.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Returns the amount as a fraction of the category width.
    pub fn amount_value(&self) -> f32 {
        self.amount
    }

    /// Returns the distribution.
    pub fn mode_value(&self) -> SVJitterMode {
        self.mode
    }

    /// Returns the explicit seed, when one is set.
    pub fn seed_value(&self) -> Option<u64> {
        self.seed
    }
}
