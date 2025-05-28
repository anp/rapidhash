# Changelog

## 2.0.0 (20250514)

**Rapidhash algorithm changes**. Rapidhash released V2.0, V2.1, V2.2, and V3 in quick succession. We have decided to support all versions of rapidhash under `rapidhash::v3` etc modules. These expose `rapidhash_v3` and similar methods to avoid confusion.

- **Breaking:** `RapidHasher` overhaul:
  - `RapidHasher` deviates from the main rapidhash algorithm to improve performance hashing rust objects while maintaining similar hash quality.
  - `RapidHasher` may change the underlying hash between minor versions. The rust `Hasher` trait is not to be used for persistent hashing, and we will follow this mantra to allow easily improving hashing performance. Persistent hashing should be done though the `rapidhash::v3::rapidhash_v3(bytes: &[u8])` functions.
  - `RapidHasher`, `RandomState`, `RapidHashMap`, and `RapidHashSet` now move behind the following three modules:
    - `rapidhash::fast` when hashing speed is the priority. This sacrifices some hash quality for speed, and uses FNV when hashing integer types.
    - `rapidhash::quality` when hash quality is the ultimate priority. This closely resembles the rapidhash algorithm and hash quality.
    - `rapidhash::inner` when you want to configure the settings for `AVALANCHE`, `FNV`, `COMPACT`, and `PROTECTED` modes as necessary.
- **Breaking:** `rapidhash` function moved and renamed:
  - Fixed the rapidhash V1 algorithm for 48 and 144 length inputs, where it would mismatch with the C implementation.
  - Moved and renamed `rapidhash::rapidhash()` to `rapidhash::v1::rapidhash_v1()` to allow us to include other rapidhash versions.
- **Breaking:** Removed the deprecated `RapidHashBuilder` type, it has been replaced with `RapidBuildHasher` to match the rust naming convention.
- **Breaking:** Removed `rapid_mix` and `rapid_mum` from the public API for cleanliness. These are now internal functions.
- **Breaking:** Random number generation has been moved behind the `rng` module, but otherwise works the same.
- **Breaking:** Removed the `RapidInline*` variants in favour of making the default `Rapid` types inline by default. If this is a problem for your use case, please raise a github issue, thanks!
- **Breaking:** `RapidRandomState` has been renamed to `RandomState`.
- New: Added support for rapidhash V2.0, V2.1, V2.2, and V3 algorithms.
- New: rapidhash CLI now supports a proper streaming V3 version.
- Fix: Full tests and verification against the C implementations for both versions.
- Perf: Extensive benchmarking and optimisation, see more: https://github.com/hoxxep/rapidhash/issues/20

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
