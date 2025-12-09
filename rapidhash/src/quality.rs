//! In-memory hashing: RapidHasher with a focus on hash quality.
//!
//! This is a specific instantiation of the [crate::inner] module with the following settings:
//! - `AVALANCHE` is enabled.
//! - `SPONGE` is enabled.
//! - `COMPACT` is disabled, unless building for WASM targets.
//! - `PROTECTED` is disabled.

const AVALANCHE: bool = true;
const SPONGE: bool = true;
const COMPACT: bool = cfg!(target_family = "wasm");
const PROTECTED: bool = false;

use crate::inner;

/// A [std::hash::Hasher] inspired by [crate::v3::rapidhash_v3] with a focus on speed and throughput.
///
/// This is an alias for [inner::RapidHasher] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RapidHasher] for a lower quality but faster hash output where desirable.
pub type RapidHasher<'s> = inner::RapidHasher<'s, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A rapidhash equivalent to [std::hash::RandomState] that uses a random seed and secrets for
/// minimal DoS resistance.
///
/// This initialises a [RapidHasher] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::RandomState] for a lower quality but faster hash output where desirable.
pub type RandomState = inner::RandomState<AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [crate::fast::RapidHasher] algorithm.
///
/// This initialises a [RapidHasher] with the following settings:
/// - `AVALANCHE` is enabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::SeedableState] for a lower quality but faster hash output where desirable.
pub type SeedableState<'secrets> = inner::SeedableState<'secrets, AVALANCHE, SPONGE, COMPACT, PROTECTED>;

/// A [std::hash::BuildHasher] trait compatible hasher that uses the [crate::fast::RapidHasher] algorithm.
///
/// This initialises a [crate::quality::RapidHasher] with the following settings:
/// - `AVALANCHE` is disabled.
/// - `SPONGE` is enabled.
/// - `COMPACT` is disabled.
/// - `PROTECTED` is disabled.
///
/// Use [crate::fast::GlobalState] for a lower quality but faster hash output where desirable.
pub type GlobalState = inner::GlobalState<AVALANCHE, SPONGE, COMPACT, PROTECTED>;
