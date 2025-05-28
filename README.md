# rapidhash - rust implementation

A rust implementation of the [rapidhash](https://github.com/Nicoshev/rapidhash) function, the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmark. Collision-based study showed a collision probability lower than wyhash, foldhash, and close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant throughput improvement over wyhash and foldhash. Fastest memory-safe hash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling default features.
- **Official successor to wyhash** with improved speed, quality, and compatibility.
- **Inline variants** that use `#[inline(always)]` on `RapidInlineHash` and `RapidInlineHashBuilder` to force compiler optimisations on specific input types (can double the hash performance depending on the hashed type).
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet` usage.
- **Non-cryptographic** hash function that's "minimally DoS resistant" in the same manner as foldhash.

## Usage
### Hashing
```rust
use std::hash::Hasher;
use rapidhash::fast::RapidHasher;
use rapidhash::v3::rapidhash_v3;

// direct const usage
assert_eq!(rapidhash_v3(b"hello world"), 1722744455612372674);

// a std::hash::Hasher compatible hasher
let mut hasher = RapidHasher::default();
hasher.write(b"hello world");
assert_eq!(hasher.finish(), 1722744455612372674);

// a const API similar to std::hash::Hasher
const HASH: u64 = RapidHasher::default_const()
    .write_const(b"hello world")
    .finish_const();
assert_eq!(HASH, 1722744455612372674);
```

### Helper Types
```rust
// also includes HashSet equivalents
use rapidhash::fast::RapidHashMap;

// std HashMap with the RapidHashBuilder hasher.
let mut map = RapidHashMap::default();
map.insert("hello", "world");
```

### CLI
Rapidhash can also be installed as a CLI tool to streaming hash files or stdin. This is not a cryptographic hash, but should be much faster than cryptographic hashes. This is fully compatible with C++ rapidhash V1, V2, and V3 algorithms.

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

Fixed versioning with C++ compatibility is presented in `rapidhash::v1`, `rapidhash::v2`, and `rapidhash::v3` modules.

Rust hasing traits (`RapidHasher`, `RapidBuildHasher`, etc.) are implemented in `rapidhash::fast`, `rapidhash::quality`, and `rapidhash::inner` modules. These are not guaranteed to give a consistent hash output between crate versions as the rust `Hasher` trait is not designed for this.

- Use `rapidhash::fast` for optimal hashing speed with a slightly lower hash quality. Best for most datastructures such as HashMap and HashSet usage.
- Use `rapidhash::quality` where statistical hash quality is the priority, such as HyperLogLog or MinHash algorithms.
- Use `rapidhash::inner` to configure advanced parameters to configure the hash function specifically to your use case. This allows enabling/disabling avalanching, FNV on integer types, compact mode to generate fewer instructions at compile time, and protected mode. Read more in the [rust documentation](https://docs.rs/rapidhash/latest/rapidhash/).

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables `RapidRandomState`, a `BuildHasher` that randomly initializes the seed. Includes the `rand` crate dependency.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic random number generator based on rapidhash. Includes the `rand_core` crate dependency.
- `unsafe`: Uses unsafe pointer arithmetic to skip some unnecessary bounds checks for a small 3-4% performance improvement.

## How to choose your hash function

Hash functions are not a one-size fits all. Benchmark your use case to find the best hash function for your needs, but here are some general guidelines on choosing a hash function:

- `default`: Use the std lib hasher when hashing is not in the critical path, or if you need strong DoS resistance.
- `rapidhash::fast`: You don't require any DoS resistance, and you want the fastest portable hash function available.
- `rapidhash::quality`: You require minimal DoS resistance, and you want a fast, general purpose, portable hash function.
- `foldhash`: You are hashing many tuples of small integers, whcih will be slightly faster with the foldhash sponge construction.
- `gxhash`: You are hashing long byte streams on platforms with the necessary instruction sets and only care about throughput. You don't need memory safety, HashDoS resistance, or platform independence.

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
- Major for breaking changes, such as hash output changes, breaking API changes, MSRV version bumps. When the RNG code is stabilised, major version bumps to `rand_core` will also trigger a major version bump of rapidhash due to the re-exported trait implementations.
- Minor for significant API additions/deprecations.
- Patch for bug fixes and performance improvements.

## License and Acknowledgements
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to [Nicolas De Carli](https://github.com/Nicoshev) for the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [BSD 2-Clause license](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).

With thanks to [Justin Bradford](https://github.com/jabr) for letting us use the rapidhash crate name 🍻
