# rapidhash – portable rust hashing

A rust implementation of [rapidhash](https://github.com/Nicoshev/rapidhash), the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmarks. Collision-based study showed a collision probability that's close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant peak throughput improvement over wyhash and foldhash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling default features.
- **Official successor to wyhash** with improved speed, quality, and compatibility.
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet`.
- **Non-cryptographic** hash function that's "minimally DoS resistant" in the same manner as foldhash.
- **Streamable** hashing for large files and other streams.
- **CLI tool** for convenient hashing of files or stdin.

**Sponsored by [Upon](https://uponvault.com?utm_source=github&utm_campaign=rapidhash)**, inheritance vaults for your digital life. Ensure your family can access your devices, accounts, and assets when the unexpected happens.

## Usage
### In-Memory Hashing
Following rust's `std::hash` traits, the underlying hash function may change between minor versions, and is only suitable for in-memory hashing. These types are optimised for speed and minimal DoS resistance, available in the `rapidhash::fast` and `rapidhash::quality` flavours.

- `RapidHasher`: A `std::hash::Hasher` compatible hasher that uses the rapidhash algorithm.
- `RapidHashBuilder`: A `std::hash::BuildHasher` for initialising the hasher with the default seed and secrets.
- `RandomState`: A `std::hash::BuildHasher` for initialising the hasher with a random seed and secrets.
- `RapidHashMap` and `RapidHashSet`: Helper types for using `RapidHasher` with `HashMap` and `HashSet`.

```rust
use rapidhash::fast::RapidHashMap;

// A HashMap using RapidHasher for fast in-memory hashing.
let mut map = RapidHashMap::default();
map.insert("key", "value");
```

```rust
use std::hash::BuildHasher;
use rapidhash::quality::RapidBuildHasher;

// Using the RapidHasher directly for in-memory hashing.
let hasher = RapidBuildHasher::default();
assert_eq!(hasher.hash_one(b"hello world"), 9938606849760368330);
```

### Portable Hashing
Full compatibility with C++ rapidhash algorithms, methods are provided for all rapidhash V1, V2, and V3 (with micro/nano) variants. These are stable functions whose output will not change between crate versions.

```rust
use std::hash::{BuildHasher, Hasher};
use rapidhash::v3::{rapidhash_v3_seeded, rapidhash_v3_file_seeded, RapidSecrets};

/// Set your global hashing secrets.
/// - For HashDoS resistance, choose a randomised secret.
/// - For C++ compatibility, use the `seed_cpp` method or `DEFAULT_RAPID_SECRETS`.
const RAPID_SECRETS: RapidSecrets = RapidSecrets::seed(0x123456);

/// A helper function for your chosen rapidhash version and secrets.
#[inline]
pub fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_v3_seeded(data, &RAPID_SECRETS)
}

/// Hash streaming data with the rapidhash V3 algorithm.
pub fn rapidhash_stream<R: std::io::Read>(reader: R) -> std::io::Result<u64> {
    rapidhash_v3_file_seeded(reader, &RAPID_SECRETS)
}

assert_eq!(rapidhash(b"hello world"), 11653223729569656151);
assert_eq!(rapidhash_stream(std::io::Cursor::new(b"hello world")).unwrap(), 11653223729569656151);
```

Please see the [`portable-hash` crate](https://github.com/hoxxep/portable-hash?tab=readme-ov-file#whats-wrong-with-the-stdhash-traits) for why using the standard library hashing traits is not recommended for portable hashing. Rapidhash is planning to implement the `PortableHash` and `PortableHasher` traits in a future release.

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

## Features

- `default`: `std`
- `std`: Enables the `RapidHashMap` and `RapidHashSet` helper types.
- `rand`: Enables using the `rand` library to more securely initialise `RandomState`. Includes the `rand` crate dependency.
- `rng`: Enables `RapidRng`, a fast, non-cryptographic PRNG based on rapidrng. Includes the `rand_core` crate dependency.
- `unsafe`: Uses unsafe pointer arithmetic to skip some unnecessary bounds checks for a small 3-4% performance improvement.

## Benchmarks

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_aarch64_apple_m1_max_native.svg)

Raw throughput isn't the only measure to consider, as the interactions with Rust's `std::hash::Hasher` trait and the `HashMap` and `HashSet` types have a significant effect on inlining and performance across various different types.

Rapidhash uses its own benchmark charts (the charts) to measure raw throughput, and the [foldhash benchmark suite](https://github.com/orlp/foldhash?tab=readme-ov-file#performance) (the txt tables) for benchmarks that are closer to real-world usage. The foldhash suite benchmarks hashers by measuring raw hash throughput, hashmap lookup miss, hashmap lookup hit, and hashmap insertion performance on a wide variety of commonly hashed types.

We ran the benchmarks with and without `-C target-cpu=native` on a variety of platforms to demonstrate rapidhash's strong all-round performance. The full results are available in the [docs folder](https://github.com/hoxxep/rapidhash/tree/master/docs).

<details>
<summary><strong>aarch64 Apple M1 Max (target-cpu=native)</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_aarch64_apple_m1_max_native.svg)

```text
┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
│         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
│       avg_rank ┆        2.03 ┆        4.22 ┆       3.38 ┆       4.98 ┆   4.69 ┆   3.16 ┆  5.58 ┆    7.97 │
│ geometric_mean ┆        4.26 ┆        4.88 ┆       4.83 ┆       5.22 ┆   4.96 ┆   5.49 ┆  5.94 ┆   21.95 │
└────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘
```

</details>

<details>
<summary><strong>aarch64 AWS Graviton3 (target-cpu=native)</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_aarch64_aws_graviton3_native.svg)

```text
┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
│         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
│       avg_rank ┆        2.59 ┆        4.20 ┆       3.38 ┆       5.28 ┆   4.09 ┆   2.50 ┆  5.98 ┆    7.97 │
│ geometric_mean ┆        7.84 ┆        8.97 ┆       8.56 ┆       9.68 ┆   8.59 ┆   8.15 ┆ 11.16 ┆   32.59 │
└────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘
```

</details>

<details>
<summary><strong>x86_64 AMD EPYC 9R14 (target-cpu=native)</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_x86_64_amd_epyc_9R14_native.svg)

```text
┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
│         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
│       avg_rank ┆        2.56 ┆        4.36 ┆       3.45 ┆       5.38 ┆   4.31 ┆   3.36 ┆  4.61 ┆    7.97 │
│ geometric_mean ┆        4.68 ┆        5.34 ┆       5.24 ┆       5.91 ┆   5.01 ┆   5.98 ┆  5.63 ┆   25.75 │
└────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘
```

</details>

<details>
<summary><strong>x86_64 Intel Xeon Platinum 8488C (target-cpu=native)</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_x86_64_intel_xeon_8488c_native.svg)

```text
┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
│         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
│       avg_rank ┆        2.38 ┆        4.69 ┆       3.52 ┆       5.30 ┆   4.08 ┆   3.39 ┆  4.69 ┆    7.97 │
│ geometric_mean ┆        4.46 ┆        5.09 ┆       4.88 ┆       5.42 ┆   4.73 ┆   5.58 ┆  5.26 ┆   21.34 │
└────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘
```

</details>

<details>
<summary><strong>Benchmark notes</strong></summary>

- Hash throughput/latency does not measure hash "quality", and many of the benchmarked functions fail the [SMHasher3 hash benchmarks](https://gitlab.com/fwojcik/smhasher3). Hash quality affects hashmap performance, as well as algorithms that benefit from high quality hash functions such as HyperLogLog and MinHash.
- Most hash functions will be affected heavily by whether the compiler has inlined them. Rapidhash tries very hard to always be inlined by the compiler, but the larger a program, benchmark, or the hashed type gets, the less likely it is to be inlined due to Rust's `Hash::hash` method not being `#[inline(always)]`.
- `gxhash` achieves its high throughput by using AES instructions and consistently outperforms the other accelerated hashers (ahash, th1a, xxhash3_64). It's a great hash function, but is not a portable hash function (requiring `target-cpu=native` or specific feature flags to compile). Gxhash is a great choice for applications that can guarantee the availability of AES instructions and mostly hash strings.
- The default rust hasher (SipHasher) unexpectedly appears to run consistently faster _without_ `target-cpu=native` on various x86 and ARM chips.
- Benchmark your own use case, with your real world dataset! We suggest experimenting with different hash functions to see which one works best for your use case. Rapidhash is great for fast general-purpose hashing in libraries and applications, but certain hashers will outperform for specific use cases.

</details>

## Rapidhash Versioning

### Portable Hashing
C++ compatibility is presented in `rapidhash::v1`, `rapidhash::v2`, and `rapidhash::v3` modules. The output for these is guaranteed to be stable between major crate versions.

Rapidhash V3 is the recommended, fastest, and most recent version of the hash. Streaming is only possible with the rapidhash V3 algorithm. Others are provided for backwards compatibility.

### In-Memory Hashing
Rust hasing traits (`RapidHasher`, `RapidBuildHasher`, etc.) are implemented in `rapidhash::fast`, `rapidhash::quality`, and `rapidhash::inner` modules. These are not guaranteed to give a consistent hash output between platforms, compiler versions, or crate versions as the rust `Hasher` trait [is not suitable](https://github.com/hoxxep/portable-hash/?tab=readme-ov-file#whats-wrong-with-the-stdhash-traits) for portable hashing.

- Use `rapidhash::fast` for optimal hashing speed with a slightly lower hash quality. Best for most datastructures such as HashMap and HashSet usage.
- Use `rapidhash::quality` where statistical hash quality is the priority, such as HyperLogLog or MinHash algorithms.
- Use `rapidhash::inner` to set advanced parameters to configure the hash function specifically to your use case.

## Crate Versioning
The minimum supported Rust version (MSRV) is 1.77.0.

The rapidhash crate follows the following versioning scheme:
- Major for breaking API changes and MSRV version bumps or any changes to `rapidhash_v*` method output.
- Minor for significant API additions/deprecations or any changes to `RapidHasher` output.
- Patch for bug fixes and performance improvements.

Portable hash outputs (eg. `rapidhash_v3`) are guaranteed to be stable. In-memory hash outputs (eg. `RapidHasher`) may change between minor versions to allow us to freely improve performance.

## License and Acknowledgements
This project is licensed under both the MIT and Apache-2.0 licenses. You are free to choose either license.

With thanks to [Nicolas De Carli](https://github.com/Nicoshev) for the original [rapidhash](https://github.com/Nicoshev/rapidhash) C++ implementation, which is licensed under the [MIT License](https://github.com/Nicoshev/rapidhash/blob/master/LICENSE).

With thanks to [Justin Bradford](https://github.com/jabr) for letting us use the rapidhash crate name 🍻
