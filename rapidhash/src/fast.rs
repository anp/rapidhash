//! RapidHasher with a focus on hashing and data structure speed.
//!
//! - `AVALANCHE` is disabled.
//! - `FNV` is enabled.
//! - `COMPACT` is disabled.
//! - `PROTECTED` is disabled.

const AVALANCHE: bool = false;
const FNV: bool = true;
const COMPACT: bool = false;
const PROTECTED: bool = false;

use crate::v3;

// TODO: random state etc.
pub type RapidHasher = v3::RapidHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;
pub type RapidBuildHasher = v3::RapidBuildHasher<AVALANCHE, FNV, COMPACT, PROTECTED>;
pub type RapidHashMap<K, V> = v3::RapidHashMap<K, V, AVALANCHE, FNV, COMPACT, PROTECTED>;
pub type RapidHashSet<K> = v3::RapidHashSet<K, AVALANCHE, FNV, COMPACT, PROTECTED>;
