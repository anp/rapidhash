use core::hash::{BuildHasher, Hash, Hasher};
use crate::v3::rapid_const::{rapid_mix, rapidhash_core, rapidhash_finish, RAPID_SECRET, RAPID_SEED};

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
///
/// #[cfg(not(feature = "v2"))]
/// use rapidhash::RapidHasher;
/// #[cfg(feature = "v2")]
/// use rapidhash::v2::RapidHasher;
///
/// let mut hasher = RapidHasher::default();
/// hasher.write(b"hello world");
/// let hash = hasher.finish();
/// ```
#[derive(Clone)]
#[repr(C)]
pub struct RapidHasher<const AVALANCHE: bool> {
    // NOTE: field order is important for performance and inlining, benchmark changes!
    // size: u64,
    a: u64,
    b: u64,
    seed: u64,
    buf_len: u8,
    buf: u128,
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
///
/// #[cfg(not(feature = "v2"))]
/// use rapidhash::RapidBuildHasher;
/// #[cfg(feature = "v2")]
/// use rapidhash::v2::RapidBuildHasher;
///
/// let mut map = HashMap::with_hasher(RapidBuildHasher::default());
/// map.insert(42, "the answer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RapidBuildHasher<const AVALANCHE: bool> {
    seed: u64,
}

impl<const AVALANCHE: bool> RapidBuildHasher<AVALANCHE> {
    /// New rapid inline build hasher, and pre-compute the seed.
    #[inline]
    pub const fn new(mut seed: u64) -> Self {
        seed ^= rapid_mix(seed ^ RAPID_SECRET[2], RAPID_SECRET[1]);
        Self { seed }
    }
}

// Explicitly implement to inline always the hasher.
impl<const AVALANCHE: bool> BuildHasher for RapidBuildHasher<AVALANCHE> {
    type Hasher = RapidHasher<AVALANCHE>;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::new_precomputed_seed(self.seed)
    }

    /// The aim of the game here is twofold:
    /// - let the compiler inline as much as possible
    /// - ensure the compiler prioritises inlining `x.hash()`, which has the biggest boost to
    ///   performance by allowing it to optimise out most of the sponge logic
    ///
    /// This is evident in the realworld benchmarks — only run one benchmark with inline everything
    /// and hashing is up to 2x faster. Play your cards wrong with variables in the wrong order or
    /// too many instructions and if x.hash() isn't inlined it can be 5x slower in a bunch of
    /// benchmarks... Frustrating voodoo to be up against! An alternative is to remove the sponge
    /// and hash numbers in a more optimal way.
    #[inline]  // TODO: choose what to set here
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

impl<const AVALANCHE: bool> Default for RapidBuildHasher<AVALANCHE> {
    #[inline]
    fn default() -> Self {
        Self::new(RapidHasher::<AVALANCHE>::DEFAULT_SEED)
    }
}

/// A [std::collections::HashMap] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// #[cfg(not(feature = "v2"))]
/// use rapidhash::RapidHashMap;
/// #[cfg(feature = "v2")]
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
pub type RapidHashMap<K, V, const AVALANCHE: bool> = std::collections::HashMap<K, V, RapidBuildHasher<AVALANCHE>>;

/// A [std::collections::HashSet] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// #[cfg(not(feature = "v2"))]
/// use rapidhash::RapidHashSet;
/// #[cfg(feature = "v2")]
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
pub type RapidHashSet<K, const AVALANCHE: bool> = std::collections::HashSet<K, RapidBuildHasher<AVALANCHE>>;

impl<const AVALANCHE: bool> RapidHasher<AVALANCHE> {
    /// Default `RapidHasher` seed.
    pub const DEFAULT_SEED: u64 = RAPID_SEED;

    /// Create a new [RapidHasher] with a custom seed.
    #[inline(always)]
    #[must_use]
    pub const fn new(mut seed: u64) -> Self {
        // do most of the rapidhash_seed initialisation here to avoid doing it on each int
        seed ^= rapid_mix(seed ^ RAPID_SECRET[2], RAPID_SECRET[1]);
        Self::new_precomputed_seed(seed)
    }

    #[inline(always)]
    #[must_use]
    pub(super) const fn new_precomputed_seed(seed: u64) -> Self {
        Self {
            // size: 0,
            seed,
            a: 0,
            b: 0,
            buf: 0,
            buf_len: 0,
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
    pub const fn write_const(&mut self, bytes: &[u8]) {
        // FUTURE: wyhash processes the bytes as u64::MAX chunks in case chunk.len() > usize.
        // we use this static assert to ensure that usize is not larger than u64 for now.
        const _: () = assert!(
            usize::MAX as u128 <= u64::MAX as u128,
            "usize is wider than u64. Please raise a github issue to support this."
        );

        self.seed = self.seed.wrapping_add(bytes.len() as u64);
        let (a, b, seed) = rapidhash_core(self.a, self.b, self.seed, bytes);
        self.a = a;
        self.b = b;
        self.seed = seed;
    }

    // TODO: 32 and 64 bit usize
    #[inline(always)]
    const fn write_num<const N: u8>(&mut self, bytes: u128) {
        // the order of this if/else is temperamental and can cause the compiler to not inline it
        if self.buf_len + N > 128 {
            // let mut a = self.a ^ self.buf as u64 ^ RAPID_SECRET[1];
            // let mut b = self.b ^ (self.buf >> self.buf_len.saturating_sub(64)) as u64 ^ self.seed;
            // let mut a = self.buf as u64; //  ^ RAPID_SECRET[1];
            // let mut b = (self.buf >> 64) as u64; // ^ self.seed;
            // let r = (a as u128) * (b as u128);
            // self.seed = (r >> 64) as u64 ^ r as u64;
            self.flush_buf();
            self.buf = bytes;
            self.buf_len = N;
        } else {
            self.buf |= bytes << self.buf_len;
            self.buf_len += N;
        }
    }

    #[cold]
    #[inline(never)]
    const fn flush_buf_no_inline(&mut self) {
        self.flush_buf()
    }

    #[inline(always)]
    #[cold]
    const fn flush_buf(&mut self) {
        // we use a saturating sub here so that if only half of the buffer has been written, we can
        // also take as much of the buffer as we can into the b state.
        let mut a = self.a ^ self.buf as u64 ^ RAPID_SECRET[1];
        let mut b = self.b ^ (self.buf >> self.buf_len.saturating_sub(64)) as u64 ^ self.seed;
        // let mut b = self.b ^ self.buf ^ self.seed;
        (a, b) = super::rapid_const::rapid_mum(a, b);
        self.a = a;
        self.b = b;
    }

    #[inline(always)]
    const fn write_128(&mut self, bytes: u128) {
        // we use a saturating sub here so that if only half of the buffer has been written, we can
        // also take as much of the buffer as we can into the b state.
        self.a ^= bytes as u64 ^ RAPID_SECRET[1];
        self.b ^= (bytes >> 64) as u64 ^ self.seed;
        let (a, b) = super::rapid_const::rapid_mum(self.a, self.b);
        self.a = a;
        self.b = b;
    }

    /// Const equivalent to [Hasher::finish], and marked as `#[inline(always)]`.
    #[inline(always)]
    #[must_use]
    pub const fn finish_const(&self) -> u64 {
        let mut a = self.a;
        let mut b = self.b;
        if self.buf_len > 0 {
            a ^= self.buf as u64 ^ RAPID_SECRET[1];
            b ^= (self.buf >> self.buf_len.saturating_sub(64)) as u64 ^ self.seed;
            (a, b) = super::rapid_const::rapid_mum(a, b);
        }

        if AVALANCHE {
            rapidhash_finish(a, b, self.seed)
        } else {
            a ^ b ^ self.seed
        }
    }
}

impl<const AVALANCHE: bool> Default for RapidHasher<AVALANCHE> {
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
impl<const AVALANCHE: bool> Hasher for RapidHasher<AVALANCHE> {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.finish_const()
    }

    /// Write a byte slice to the hasher, marked as `#[inline(always)]`.
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.write_const(bytes);
    }

    // #[inline(always)]
    // fn write_u8(&mut self, i: u8) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_u16(&mut self, i: u16) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_u32(&mut self, i: u32) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_u64(&mut self, i: u64) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_u128(&mut self, i: u128) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_usize(&mut self, i: usize) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_i8(&mut self, i: i8) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_i16(&mut self, i: i16) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_i32(&mut self, i: i32) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_i64(&mut self, i: i64) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_i128(&mut self, i: i128) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }
    //
    // #[inline(always)]
    // fn write_isize(&mut self, i: isize) {
    //     *self = self.write_const(&i.to_le_bytes());
    // }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.write_num::<8>(i.into());
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.write_num::<16>(i as u128);
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.write_num::<32>(i as u128);
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.write_num::<64>(i as u128);
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        self.write_128(i);
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.write_num::<64>(i as u128);
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        self.write_num::<8>(i as u128);
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        self.write_num::<16>(i as u128);
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        self.write_num::<32>(i as u128);
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.write_num::<64>(i as u128);
    }

    #[inline(always)]
    fn write_i128(&mut self, i: i128) {
        self.write_128(i as u128);
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        self.write_num::<64>(i as u128);
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
            let mut hasher = RapidHasher::<true>::default();
            hasher.write(int.to_le_bytes().as_slice());
            let a = hasher.finish();

            assert_eq!(int.to_le_bytes().as_slice().len(), 8);

            let mut hasher = RapidHasher::<true>::default();
            hasher.write_u64(int);
            let b = hasher.finish();

            assert_eq!(a, b, "Mismatching hash for u64 with input {int}");
        }
    }
}
