# Changelog

## 2.0.0 (20250514)

**Rapidhash V2 algorithm change**. Going forwards this crate will expose old algorithms under `v1`, `v2` etc. modules and naming. **The non-versioned functions will use the latest algorithm.**

- **Breaking:** Updated to use the rapidhash V2 algorithm by default.
- **Breaking:** Rapidhash V1 algorithm changes:
  - Fixed the rapidhash V1 algorithm for 48 and 144 length inputs, where it would mismatch with the C implementation.
  - Since the algorithm implementation changed, I also took the opportunity to make performance improvements and to include seed pre-computation but this shouldn't change the hash output.
- **Breaking:** Removed the deprecated `RapidHashBuilder` type, it has been replaced with `RapidBuildHasher` to match the rust naming convention.
- **Breaking:** Removed `rapid_mix` and `rapid_mum` from the public API for cleanliness. These are now internal functions.
- **Breaking:** Random number generator algorithm has changed as the V2 secrets are different. The v2 `rapidng_time`, `rapidrng_fast`, and `RapidRng` will produce different values to the v1 versions.
- **Breaking:** Removed the `RapidInline*` variants in favour of making the default `Rapid` types inline by default. If this is a problem for your use case, please raise a github issue, thanks!
- Added `v1` and `v2` features to expose V1 and V2 algorithms with explicit versioning.
- Full tests and verification against the C implementations for both versions.
- Extensive benchmarking and optimisation, read more: https://github.com/hoxxep/rapidhash/issues/20

## 1.4.0 (20250219)

- Updated `rand` and `rand-core` to 0.9. [#18](https://github.com/hoxxep/rapidhash/pull/18)
- Fixed issue where using feature `unsafe` and without `std` would fail to compile. [#15](https://github.com/hoxxep/rapidhash/issues/15) and [#17](https://github.com/hoxxep/rapidhash/pull/17)

## 1.3.0 (20241208)

- Added `rapidhash_file` for streaming file hashing. [#10](https://github.com/hoxxep/rapidhash/pull/10)
- Added file streaming and `--help` to the CLI. [#10](https://github.com/hoxxep/rapidhash/pull/10)

## 1.2.0 (20241204)

- Added rapidhash CLI via `cargo install rapidhash`.
- Docs typo fix.

## 1.1.0 (20241003)

- Deprecated `RapidHashBuilder`.
- Added `RapidBuildHasher` to replace `RapidHashBuilder`.

## 1.0.0 (20241002)

Ownership kindly transferred by Justin Bradford to [Liam Gray](https://github.com/hoxxep) and this repository.

- **Breaking:** Removed the `hash` function that only hashes on `u128` types.
- Added `rapidhash` and `rapidhash_seeded` functions to hash byte streams.
- Added `RapidHasher` and `RapidHasherInline` for hashing via a `std::hash::Hasher` compatible interface.
- Added `RapidHashMap`, `RapidInlineHashMap`, `RapidHashSet`, and `RapidInlineHashSet` helper types.
- Added `RapidHashBuilder` and `RapidInlineHashBuilder` for `std::hash::BuildHasher` implementing types compatible with `HashMap` and `HashSet`.
- Added `RapidRandomState` for random seed initialisation.
- Added `RapidRng`, `rapidrng_fast`, and `rapidrng_time` for random number generation inspired by the [wyhash crate](https://docs.rs/wyhash/latest/wyhash/) but based on `rapid_mix`.
- Added `std`, `rand`, `rng`, and `unsafe` features.
- Extensive benchmarking and optimisation.

## 0.1.0

Initial release by [Justin Bradford](https://github.com/jabr) supporting rapidhash on `u128` inputs.

- Added `hash` for rapidhashing `u128` types.
