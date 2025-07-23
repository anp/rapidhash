# rapidhash - rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmark. Collision-based study showed a collision probability lower than wyhash, foldhash, and close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant peak throughput improvement over wyhash and foldhash. Fastest memory-safe hash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling default features.
- **Official successor to wyhash** with improved speed, quality, and compatibility.
- **Inline variants** that use `#[inline(always)]` on `RapidInlineHash` and `RapidInlineHashBuilder` to force compiler optimisations on specific input types (can double the hash performance depending on the hashed type).
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet` usage.
- **Non-cryptographic** hash function that's "minimally DoS resistant" in the same manner as foldhash.

## Usage
### Persistent Hashing
Full compatibility with C++ rapidhash algorithms, methods are provided for all rapidhash V1, V2, and V3 (with micro/nano) variants. These are stable functions whose output will not change between crate versions.

```rust
use std::hash::{BuildHasher, Hasher};

// rapidhash V3 algorithm, for fast, high-quality, persistent hashing
use rapidhash::v3::rapidhash_v3;
assert_eq!(rapidhash_v3(b"hello world"), 3397907815814400320);
```

### In-Memory Hashing
Following rust's `std::hash` traits, the underlying hash function may change between minor versions, and is only suitable for in-memory hashing. These types are optimised for speed and minimal DoS resistance.

- `RapidHasher`: A `std::hash::Hasher` compatible hasher that uses the rapidhash algorithm.
- `RapidHashBuilder`: A `std::hash::BuildHasher` for initialising the hasher with the default seed and secrets.
- `RandomState`: A `std::hash::BuildHasher` for initialising the hasher with a random seed and secrets.
- `RapidHashMap` and `RapidHashSet`: Helper types for using `RapidHasher` with `HashMap` and `HashSet`.

```rust
use rapidhash::fast::RapidHashMap;

// std HashMap with the RapidHashBuilder hasher.
let mut map = RapidHashMap::default();
map.insert("key", "value");
```

### CLI
Rapidhash can also be installed as a CLI tool to hash files or stdin. This is not a cryptographic hash, but should be much faster than cryptographic hashes. This is fully compatible with the C++ rapidhash V1, V2, and V3 algorithms.

Output is the decimal string of the `u64` hash value.

```shell
# install
cargo install rapidhash

# hash a file (output: 8543579700415218186)
rapidhash --v3 example.txt

# hash stdin (output: 8543579700415218186)
echo "example" | rapidhash --v3
```

## Rapidhash Versions

Rapidhash has multiple versions of the algorithm.

### Persistent Hashing
Fixed versioning with C++ compatibility is presented in `rapidhash::v1`, `rapidhash::v2`, and `rapidhash::v3` modules.

Rapidhash V3 is the recommended, fastest, and most recent version of the hash. Others are provided for backwards compatibility.

### In-Memory Hashing
Rust hasing traits (`RapidHasher`, `RapidBuildHasher`, etc.) are implemented in `rapidhash::fast`, `rapidhash::quality`, and `rapidhash::inner` modules. These are not guaranteed to give a consistent hash output between crate versions as the rust `Hasher` trait is not designed for this.

- Use `rapidhash::fast` for optimal hashing speed with a slightly lower hash quality. Best for most datastructures such as HashMap and HashSet usage.
- Use `rapidhash::quality` where statistical hash quality is the priority, such as HyperLogLog or MinHash algorithms.
- Use `rapidhash::inner` to configure advanced parameters to configure the hash function specifically to your use case. This allows tweaking the following compile time parameters, which all change the hash output:
  - `AVALANCHE`: Enables the final avalanche mixing step to improve hash quality. Enabled on quality, disabled on fast.
  - `FNV`: Hash integer types using FNV instead of the rapidhash algorithm. Reduces DoS resistance on integer types. Disabled on quality, enabled on fast.
  - `COMPACT`: Generates fewer instructions at compile time with less manual loop unrolling, but may be slower on some platforms. Disabled by default.
  - `PROTECTED`: Slightly stronger hash quality and DoS resistance by performing two extra XOR instructions on every mix step. Disabled by default.

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables using the `rand` library to more securely initialise `RandomState`. Includes the `rand` crate dependency.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic PRNG based on rapidrng. Includes the `rand_core` crate dependency.
- `unsafe`: Uses unsafe pointer arithmetic to skip some unnecessary bounds checks for a small 3-4% performance improvement.

## Benchmarks

Initial benchmarks on M1 Max (aarch64) for various input sizes.

### Hashing Benchmarks
There are two types of benchmarks over the different algorithms to cover various forms of compiler optimisation that Rust can achieve:
- `str_len`: hashing bytes (a string) of the given length, where the length is not known at compile time.
- `u64`: hashing a u64, 8 bytes of known size, where the compiler can optimise the path.

Note on wyhash: hashing throughput doesn't translate to hashmap insertion throughput, see the hashmap insertion benchmarks below.

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash.svg)

### HashMap Insertion Benchmarks

Hash speed and throughput can be a poor measure in isolation, as it doesn't take into account hash quality. More hash collisions can cause slower hashmap insertion, and so hashmap insertion benchmarks can be a better measure of hash performance. As always, benchmark your use case.

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_insert.svg)

## Versioning
The minimum supported Rust version (MSRV) is 1.77.0.

The rapidhash crate follows the following versioning scheme:
- Major for breaking API changes and MSRV version bumps.
- Minor for significant API additions/deprecations or any changes to `RapidHasher` output.
- Patch for bug fixes and performance improvements.

Persistent hash outputs (eg. `rapidhash_v3`) are guaranteed to be stable. In-memory hash outputs (eg. `RapidHasher`) may change between minor versions to allow us to freely improve performance.

## License and Acknowledgements
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to [Nicolas De Carli](https://github.com/Nicoshev) for the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [MIT License](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).

With thanks to [Justin Bradford](https://github.com/jabr) for letting us use the rapidhash crate name 🍻
