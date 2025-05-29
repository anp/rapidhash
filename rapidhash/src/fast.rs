//! RapidHasher with a focus on hashing and data structure speed.
//!
//! This is a specific instantiation of the [rapidhash::inner] module with the following settings:
//! - `AVALANCHE` is disabled.
//! - `FNV` is enabled.
//! - `COMPACT` is disabled.
//! - `PROTECTED` is disabled.

const AVALANCHE: bool = false;
const FNV: bool = true;
const COMPACT: bool = false;
const PROTECTED: bool = false;

use crate::inner;

// TODO: random state etc.

/// A [Hasher] inspired by [rapidhash::v3::rapidhash_v3] with a focus on speed and throughput.
///
/// This is an alias for [inner::RapidHasher] with the following settings:
/// - `AVALANCHE` is disabled.
/// - `FNV` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::quality::RapidHasher] for a higher quality hash output where necessary.
pub type RapidHasher = inner::RapidHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [RapidHasher] algorithm.
///
/// This is an alias for [inner::RapidBuildHasher] with the following settings:
/// - `AVALANCHE` is disabled.
/// - `FNV` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Note there that [crate::RapidRandomState] should be used
pub type RapidBuildHasher = inner::RapidBuildHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;

/// A rapidhash equivalent to [std::hash::RandomState] that uses a random seed and secrets.
pub type RandomState = inner::RandomState<AVALANCHE, FNV, COMPACT, PROTECTED>;

/// A [std::collections::HashMap] that uses the [RapidHasher] hash.
///
/// This is an alias for [inner::RapidHashMap] with the following settings:
/// - `AVALANCHE` is disabled.
/// - `FNV` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::quality::RapidHashMap] where higher hash collision resistance is required.
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V> = inner::RapidHashMap<K, V, AVALANCHE, FNV, COMPACT, PROTECTED>;

/// A [std::collections::HashSet] that uses the [RapidHasher] hash.
///
/// This is an alias for [inner::RapidHashSet] with the following settings:
/// - `AVALANCHE` is disabled.
/// - `FNV` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::quality::RapidHashSet] where higher hash collision resistance is required.
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K> = inner::RapidHashSet<K, AVALANCHE, FNV, COMPACT, PROTECTED>;

#[cfg(any(feature = "std", docsrs))]
pub use inner::HashMapExt;
#[cfg(any(feature = "std", docsrs))]
pub use inner::HashSetExt;

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_new() {
        let mut map = RapidHashMap::new();
        map.insert("key", "value");
    }

    #[test]
    fn test_hashset_new() {
        let mut set = RapidHashSet::new();
        set.insert("value");
    }
}
