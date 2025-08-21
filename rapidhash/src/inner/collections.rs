use crate::inner::RandomState;

/// A [std::collections::HashMap] type that uses the [crate::fast::RandomState] hasher.
///
/// # Example
/// ```
/// use rapidhash::fast::{HashMapExt, RapidHashMap};
///
/// let mut map = RapidHashMap::default();
/// map.insert(42, "the answer");
///
/// // with capacity
/// let mut map = RapidHashMap::with_capacity(10);
/// map.insert(42, "the answer");
/// ```
pub type RapidHashMap<K, V, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> = std::collections::HashMap<K, V, RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED>>;

/// A [std::collections::HashSet] type that uses the [crate::fast::RandomState] hasher.
///
/// # Example
/// ```
/// use rapidhash::fast::{HashSetExt, RapidHashSet};
///
/// let mut set = RapidHashSet::default();
/// set.insert("the answer");
///
/// // with capacity
/// let mut set = RapidHashSet::with_capacity(10);
/// set.insert("the answer");
/// ```
pub type RapidHashSet<K, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> = std::collections::HashSet<K, RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED>>;

/// A trait for creating a `RapidHashMap` with a specified capacity and hasher.
pub trait HashMapExt<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> {
    /// Create a new `RapidHashMap` with the default capacity and hasher.
    fn new() -> Self;

    /// Create a new `RapidHashMap` with the given capacity and hasher.
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> HashMapExt<AVALANCHE, SPONGE, COMPACT, PROTECTED> for RapidHashMap<K, V, AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    fn new() -> Self {
        RapidHashMap::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        RapidHashMap::with_capacity_and_hasher(capacity, RandomState::<AVALANCHE, SPONGE, COMPACT, PROTECTED>::default())
    }
}

/// A trait for creating a `RapidHashSet` with a specified capacity and hasher.
pub trait HashSetExt<const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool = false, const PROTECTED: bool = false> {
    /// Create a new `RapidHashSet` with the default capacity and hasher.
    fn new() -> Self;

    /// Create a new `RapidHashSet` with the given capacity and hasher.
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, const AVALANCHE: bool, const SPONGE: bool, const COMPACT: bool, const PROTECTED: bool> HashSetExt<AVALANCHE, SPONGE, COMPACT, PROTECTED> for RapidHashSet<K, AVALANCHE, SPONGE, COMPACT, PROTECTED> {
    fn new() -> Self {
        RapidHashSet::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        RapidHashSet::with_capacity_and_hasher(capacity, RandomState::<AVALANCHE, SPONGE, COMPACT, PROTECTED>::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_size() {
        assert_eq!(core::mem::size_of::<RapidHashMap<u32, u32, true, true, false, false>>(), 40);
    }
}
