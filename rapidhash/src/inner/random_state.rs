use core::hash::BuildHasher;
use crate::inner::RapidHasher;

/// A [std::collections::hash_map::RandomState] compatible hasher that initializes the [RapidHasher]
/// algorithm with a random seed.
///
/// Note this is not sufficient to prevent HashDoS attacks. The rapidhash algorithm is not proven to
/// be resistant, and the seed used is not wide enough.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use std::hash::Hasher;
///
/// use rapidhash::inner::RapidRandomState;
///
/// let mut map = HashMap::with_hasher(RapidRandomState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidRandomState {
    seed: u64,
    secrets: &'static [u64; 7],
}

impl RapidRandomState {
    /// Create a new random state with a random seed.
    ///
    /// With the `rand` feature enabled, this will use [rand::random] to initialise the seed.
    ///
    /// Without `rand` but with the `std` feature enabled, this will use [crate::rapidrng_time] to
    /// initialise the seed.
    #[inline]
    pub fn new() -> Self {
        Self {
            seed: super::seeding::seed::get_seed(),
            secrets: super::seeding::secrets::get_secrets(),
        }
    }
}

impl Default for RapidRandomState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RapidRandomState {
    type Hasher = RapidHasher<true, false, false, false>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(self.seed)
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{BuildHasher, RandomState};

    #[test]
    fn test_random_state() {
        let state1 = RandomState::new();
        let state2 = RandomState::new();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }
}
