use core::hash::{BuildHasher, Hash, Hasher};
use crate::v2::rapid_const::{rapidhash_core, rapidhash_finish, rapidhash_seed, RAPID_SEED};

/// A [Hasher] trait compatible hasher that uses the [rapidhash](https://github.com/Nicoshev/rapidhash)
/// algorithm, and uses `#[inline(always)]` for all methods.
///
/// Using `#[inline(always)]` can deliver a large performance improvement when hashing complex
/// objects, but should be benchmarked for your specific use case. If you have HashMaps for many
/// different types this may come at the cost of some binary size increase.
///
/// See [crate::RapidHasher] for default non-forced inline methods.
///
/// See [RapidHashBuilder] for usage with [std::collections::HashMap].
///
/// # Example
/// ```
/// use std::hash::Hasher;
/// use rapidhash::v2::RapidHasher;
///
/// let mut hasher = RapidHasher::default();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(align(32))]
pub struct RapidHasher {
    // NOTE: field order is important for performance and inlining, benchmark changes!
    size: u64,
    seed: u64,
    a: u64,
    b: u64,
}

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// This is an alias for [`std::hash::BuildHasherDefault<RapidHasher>`] with a static seed.
///
/// Note there that [crate::RapidRandomState] with can be used instead for a
/// [std::hash::BuildHasher] that initialises with a random seed.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use std::hash::Hasher;
/// use rapidhash::v2::RapidBuildHasher;
///
/// let mut map = HashMap::with_hasher(RapidBuildHasher::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidBuildHasher {
    seed: u64,
}

impl RapidBuildHasher {
    /// New rapid inline build hasher, and pre-compute the seed.
    #[inline]
    pub const fn new(mut seed: u64) -> Self {
        seed = rapidhash_seed(seed);
        Self { seed }
    }
}

// Explicitly implement to inline always the hasher.
impl BuildHasher for RapidBuildHasher {
    type Hasher = RapidHasher;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new_precomputed_seed(self.seed)
    }

    /// It's incredibly important that the `x.hash()` call gets inlined, while the `hash_one` method
    /// itself isn't that critical to be inlined.
    ///
    /// - Otherwise the compiler won't be able to optimise out the sponge if as it might be full
    ///   when starting to hash each object.
    /// - The rapidhash method also makes use of callee-saved registers (at least on aarch64), which
    ///   then results in a slow ldr operation (25% of hash time!) when the x.hash method returns.
    #[inline(always)]
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

impl Default for RapidBuildHasher {
    #[inline]
    fn default() -> Self {
        Self::new(RapidHasher::DEFAULT_SEED)
    }
}

/// A [std::collections::HashMap] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// use rapidhash::v2::RapidHashMap;
///
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
///
/// // with capacity
/// let mut map = RapidHashMap::with_capacity_and_hasher(10, Default::default());
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V> = std::collections::HashMap<K, V, RapidBuildHasher>;

/// A [std::collections::HashSet] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// use rapidhash::v2::RapidHashSet;
///
/// let mut set = RapidHashSet::default();
/// set.insert("the answer");
///
/// // with capacity
/// let mut set = RapidHashSet::with_capacity_and_hasher(10, Default::default());
/// set.insert("the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K> = std::collections::HashSet<K, RapidBuildHasher>;

impl RapidHasher {
    /// Default `RapidHasher` seed.
    pub const DEFAULT_SEED: u64 = RAPID_SEED;

    /// Create a new [RapidHasher] with a custom seed.
    #[inline(always)]
    #[must_use]
    pub const fn new(mut seed: u64) -> Self {
        // do most of the rapidhash_seed initialisation here to avoid doing it on each int
        seed = rapidhash_seed(seed);
        Self::new_precomputed_seed(seed)
    }

    #[inline(always)]
    #[must_use]
    pub(super) const fn new_precomputed_seed(seed: u64) -> Self {
        Self {
            seed,
            a: 0,
            b: 0,
            size: 0,
        }
    }

    /// Create a new [RapidHasher] using the default seed.
    #[inline(always)]
    #[must_use]
    pub const fn default_const() -> Self {
        Self::new(Self::DEFAULT_SEED)
    }

    /// Const equivalent to [Hasher::write], and marked as `#[inline(always)]`.
    ///
    /// This can deliver a large performance improvement when the `bytes` length is known at compile
    /// time.
    #[inline(always)]
    #[must_use]
    pub const fn write_const(mut self, bytes: &[u8]) -> Self {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        // we use this static assert to ensure that usize is not larger than u64 for now.
        const _: () = assert!(
            usize::MAX as u128 <= u64::MAX as u128,
            "usize is wider than u64. Please raise a github issue to support this."
        );

        self.size += bytes.len() as u64;
        self.seed ^= bytes.len() as u64;
        let (a, b, seed) = rapidhash_core::<2, false, false>(self.a, self.b, self.seed, bytes);
        self.a = a;
        self.b = b;
        self.seed = seed;
        self
    }

    /// Const equivalent to [Hasher::finish], and marked as `#[inline(always)]`.
    #[inline(always)]
    #[must_use]
    pub const fn finish_const(self) -> u64 {
        rapidhash_finish::<false>(self.a, self.b, self.size)
    }
}

impl Default for RapidHasher {
    /// Create a new [RapidHasher] with the default seed.
    ///
    /// See [crate::RapidRandomState] for a [std::hash::BuildHasher] that initialises with a random
    /// seed.
    #[inline(always)]
    fn default() -> Self {
        Self::new(RAPID_SEED)
    }
}

/// This implementation implements methods for all integer types as the compiler will (hopefully...)
/// inline and heavily optimize the rapidhash_core for each. Where the bytes length is known the
/// compiler can make significant optimisations and saves us writing them out by hand.
impl Hasher for RapidHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.finish_const()
    }

    /// Write a byte slice to the hasher, marked as `#[inline(always)]`.
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        *self = self.write_const(bytes);
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_i128(&mut self, i: i128) {
        *self = self.write_const(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        *self = self.write_const(&i.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that writing a single u64 outputs the same as writing the equivalent bytes.
    ///
    /// Does not apply if the algorithm is using a sponge.
    #[ignore]
    #[test]
    fn test_hasher_write_u64() {
        assert_eq!((8 & 24) >> (8 >> 3), 4);

        let ints = [1234u64, 0, 1, u64::MAX, u64::MAX - 2385962040453523];

        for int in ints {
            let mut hasher = RapidHasher::default();
            hasher.write(int.to_le_bytes().as_slice());
            let a = hasher.finish();

            assert_eq!(int.to_le_bytes().as_slice().len(), 8);

            let mut hasher = RapidHasher::default();
            hasher.write_u64(int);
            let b = hasher.finish();

            assert_eq!(a, b, "Mismatching hash for u64 with input {int}");
        }
    }
}
