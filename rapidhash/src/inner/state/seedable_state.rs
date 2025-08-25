use core::hash::BuildHasher;
use crate::inner::RapidHasher;
use crate::inner::seeding::secrets::GlobalSecrets;

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// Note there that [crate::fast::RandomState] can be used instead for a [std::hash::BuildHasher]
/// that initialises with a random seed.
///
/// The lifetime `'s` is for the reference to the secrets. When using [`SeedableState::random`] or
/// [`SeedableState::fixed`] secrets, this lifetime will be `'static`.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
///
/// use rapidhash::quality::SeedableState;
///
/// let mut map = HashMap::with_hasher(SeedableState::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SeedableState<'s, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> {
    seed: u64,
    secrets: &'s [u64; 7],
}

impl<'s, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> Default for SeedableState<'s, AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Create a new [SeedableState] with a random seed. See [crate::fast::RandomState::random] for more details.
    #[inline]
    fn default() -> Self {
        Self::random()
    }
}

impl<'s, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> SeedableState<'s, AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    /// Create a new seedable state with a random seed.
    #[inline]
    pub fn random() -> Self {
        Self {
            seed: crate::inner::seeding::seed::get_seed(),
            secrets: GlobalSecrets::new().get(),
        }
    }

    /// Create a new seedable state with the default seed and secrets.
    ///
    /// Using the default secrets does not offer HashDoS resistance.
    #[inline]
    pub fn fixed() -> Self {
        Self {
            seed: crate::inner::seed::rapidhash_seed(crate::inner::seed::DEFAULT_SEED),
            secrets: &crate::inner::seed::DEFAULT_SECRETS,
        }
    }

    /// Create a new seedable state with a custom seed and secrets.
    #[inline]
    pub fn with_seed(seed: u64, secrets: &'s [u64; 7]) -> Self {
        Self {
            seed,
            secrets,
        }
    }
}

impl<'s, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool>  BuildHasher for SeedableState<'s, AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    type Hasher = RapidHasher<'s, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        RapidHasher::new_precomputed_seed(self.seed, self.secrets)
    }
}

#[cfg(test)]
mod tests {
    use core::hash::BuildHasher;

    type SeedableState<'s> = super::SeedableState<'s, false, true, false, false>;

    #[test]
    fn test_random_init() {
        assert_eq!(core::mem::size_of::<SeedableState>(), 16);

        let state1 = SeedableState::random();
        let state2 = SeedableState::random();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_ne!(finish1a, finish2a);
    }

    #[test]
    fn test_fixed_init() {
        assert_eq!(core::mem::size_of::<SeedableState>(), 16);

        let state1 = SeedableState::fixed();
        let state2 = SeedableState::fixed();

        let finish1a = state1.hash_one(b"hello");
        let finish1b = state1.hash_one(b"hello");
        let finish2a = state2.hash_one(b"hello");

        assert_eq!(finish1a, finish1b);
        assert_eq!(finish1a, finish2a);
    }
}
