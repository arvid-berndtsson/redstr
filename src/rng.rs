use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SEED_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Simple pseudo-random number generator using LCG algorithm
pub struct SimpleRng {
    state: u64,
}

fn mix64(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}

impl SimpleRng {
    /// Creates a RNG seeded from system time.
    pub fn new() -> Self {
        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed);
        let seed = time_seed ^ counter.rotate_left(17) ^ 0x9E3779B97F4A7C15;

        Self::from_seed(seed)
    }

    /// Creates a deterministic RNG with a provided seed.
    pub fn from_seed(seed: u64) -> Self {
        SimpleRng { state: mix64(seed) }
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
