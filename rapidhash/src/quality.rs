//! RapidHasher with a focus on hash quality.
//!
//! This is a specific instantiation of the [rapidhash::inner] module with the following settings:
//! - `AVALANCHE` is enabled.
//! - `FNV` is disabled.
//! - `COMPACT` is disabled.
//! - `PROTECTED` is disabled.

const AVALANCHE: bool = true;
const FNV: bool = false;
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

// TODO: use params
#[cfg(any(feature = "std", feature = "rand", docsrs))]
pub type RapidRandomState = inner::RapidRandomState;
