use core::hash::BuildHasher;
use crate::inner::rapid_const::rapidhash_seed;
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
/// use rapidhash::inner::RandomState;
///
/// let mut map = HashMap::with_hasher(RandomState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandomState {
    seed: u64,
    secrets: &'static [u64; 7],
}

impl RandomState {
    /// Create a new random state with a random seed.
    ///
    /// With the `rand` feature enabled, this will use [rand::random] to initialise the seed.
    ///
    /// Without `rand` but with the `std` feature enabled, this will use [crate::rapidrng_time] to
    /// initialise the seed.
    #[inline]
    #[cfg(target_has_atomic = "ptr")]
    pub fn new() -> Self {
        Self {
            seed: super::seeding::seed::get_seed(),
            secrets: super::seeding::secrets::get_secrets(),
        }
    }

    /// Create a state with a specific seed.
    #[inline]
    #[cfg(target_has_atomic = "ptr")]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed: rapidhash_seed(seed),
            secrets: super::seeding::secrets::get_secrets(),
        }
    }

    /// Create a state with a specific seed and secrets.
    #[inline]
    pub fn with_seed_and_static_secrets(seed: u64, secrets: &'static [u64; 7]) -> Self {
        Self {
            seed: rapidhash_seed(seed),
            secrets,
        }
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = RapidHasher<true, false, false, false>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(self.seed, self.secrets)
    }
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasher;
    use super::*;

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

    #[test]
    fn test_static_secrets() {
        static SECRETS: [u64; 7] = [
            0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7
        ];

        let state1a = RandomState::with_seed_and_static_secrets(0, &SECRETS);
        let state1b = RandomState::with_seed_and_static_secrets(0, &SECRETS);
        let state2a = RandomState::with_seed_and_static_secrets(1, &SECRETS);

        let finish1a = state1a.hash_one(b"hello");
        let finish1b = state1b.hash_one(b"hello");
        let finish2a = state2a.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);

    }
}
