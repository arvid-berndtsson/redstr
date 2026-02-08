use std::time::{SystemTime, UNIX_EPOCH};

/// Simple pseudo-random number generator using LCG algorithm
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    /// Creates a RNG seeded from system time.
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        SimpleRng { state: seed }
    }

    /// Creates a deterministic RNG with a provided seed.
    pub fn from_seed(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    /// Returns the next pseudo-random number.
    pub fn next_u64(&mut self) -> u64 {
        // Linear Congruential Generator
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
}

impl Default for SimpleRng {
    fn default() -> Self {
        Self::new()
    }
}
