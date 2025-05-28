use crate::inner::RandomState;

/// A [std::collections::HashMap] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// use rapidhash::quality::RapidHashMap;
///
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
///
/// // with capacity
/// let mut map = RapidHashMap::with_capacity_and_hasher(10, Default::default());
/// map.insert(42, "the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V, const AVALANCHE: bool, const FNV: bool, const COMPACT: bool = false, const PROTECTED: bool = false> = std::collections::HashMap<K, V, RandomState<AVALANCHE, FNV, COMPACT, PROTECTED>>;

/// A [std::collections::HashSet] type that uses the [RapidBuildHasher] hasher.
///
/// # Example
/// ```
/// use rapidhash::quality::RapidHashSet;
///
/// let mut set = RapidHashSet::default();
/// set.insert("the answer");
///
/// // with capacity
/// let mut set = RapidHashSet::with_capacity_and_hasher(10, Default::default());
/// set.insert("the answer");
/// ```
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K, const AVALANCHE: bool, const FNV: bool, const COMPACT: bool = false, const PROTECTED: bool = false> = std::collections::HashSet<K, RandomState<AVALANCHE, FNV, COMPACT, PROTECTED>>;
