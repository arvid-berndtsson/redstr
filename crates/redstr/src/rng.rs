use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicU64, Ordering};

/// Simple pseudo-random number generator using LCG algorithm
pub(crate) struct SimpleRng {
    state: u64,
}

static RNG_SEED_COUNTER: AtomicU64 = AtomicU64::new(0);

impl SimpleRng {
    pub(crate) fn new() -> Self {
        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let counter_seed = RNG_SEED_COUNTER.fetch_add(1, Ordering::Relaxed);
        let seed = time_seed ^ counter_seed.rotate_left(17) ^ 0x9E37_79B9_7F4A_7C15;

        SimpleRng { state: seed }
    }

    pub(crate) fn next(&mut self) -> u64 {
        // Linear Congruential Generator
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Mix bits so modulo operations don't rely on low-bit LCG patterns.
        let mut x = self.state;
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^ (x >> 33)
    }
}
