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
pub type RapidHasher = inner::RapidHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;
pub type RapidBuildHasher = inner::RapidBuildHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashMap<K, V> = inner::RapidHashMap<K, V, AVALANCHE, FNV, COMPACT, PROTECTED>;
#[cfg(any(feature = "std", docsrs))]
pub type RapidHashSet<K> = inner::RapidHashSet<K, AVALANCHE, FNV, COMPACT, PROTECTED>;
