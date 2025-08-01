use core::hash::{BuildHasher, Hash, Hasher};
use crate::inner::RapidHasher;
use crate::inner::seeding::secrets::GlobalSecrets;

/// A [std::collections::hash_map::RandomState] compatible hasher that initializes the [RapidHasher]
/// algorithm with a random seed and random global secrets.
///
/// This is designed to provide some HashDoS resistance by using a random seed per hashmap, and
/// a global random set of secrets.
///
/// # Example
/// ```rust
/// use std::collections::HashMap;
/// use std::hash::Hasher;
///
/// use rapidhash::quality::RandomState;
///
/// let mut map = HashMap::with_hasher(RandomState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandomState<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> {
    seed: u64,

    /// The global secrets is a zero-sized type to keep HashMap<K, V, RandomState> small.
    secrets: GlobalSecrets,
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
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
            secrets: GlobalSecrets::new(),
        }
    }
}

#[cfg(target_has_atomic = "ptr")]
impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool>  BuildHasher for RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    type Hasher = RapidHasher<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(self.seed, self.secrets.get())
    }

    #[inline]
    fn hash_one<T: Hash>(&self, x: T) -> u64
    where
        Self: Sized,
        Self::Hasher: Hasher,
    {
        let mut hasher = self.build_hasher();
        x.hash(&mut hasher);  // <-- trying hard to inline this
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasher;

    type RandomState = super::RandomState<false, true, false, false>;

    #[test]
    fn test_random_state() {
        assert_eq!(core::mem::size_of::<RandomState>(), 8);

        let state1 = RandomState::new();
        let state2 = RandomState::new();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }
}
