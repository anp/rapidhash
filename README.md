# rapidhash - rust implementation

A rust implementation of [rapidhash](https://github.com/Nicoshev/rapidhash), the official successor to [wyhash](https://github.com/wangyi-fudan/wyhash).

- **High quality**, the fastest hash passing all tests in the SMHasher and SMHasher3 benchmarks. Collision-based study showed a collision probability that's close to ideal.
- **Very fast**, the fastest passing hash in SMHasher3. Significant peak throughput improvement over wyhash and foldhash. Fastest platform-independent hash. Fastest const hash.
- **Platform independent**, works on all platforms, no dependency on machine-specific vectorized or cryptographic hardware instructions. Optimised for both AMD64 and AArch64.
- **Memory safe**, when the `unsafe` feature is disabled (default). This implementation has also been fuzz-tested with `cargo fuzz`.
- **No dependencies and no-std compatible** when disabling default features.
- **Official successor to wyhash** with improved speed, quality, and compatibility.
- **Run-time and compile-time hashing** as the hash implementation is fully `const`.
- **Idiomatic** `std::hash::Hasher` compatible hasher for `HashMap` and `HashSet` usage.
- **Non-cryptographic** hash function that's "minimally DoS resistant" in the same manner as foldhash.

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
assert_eq!(hasher.hash_one(b"hello world"), 1790036888308448300);
```

### Portable Hashing
Full compatibility with C++ rapidhash algorithms, methods are provided for all rapidhash V1, V2, and V3 (with micro/nano) variants. These are stable functions whose output will not change between crate versions.

```rust
use std::hash::{BuildHasher, Hasher};
use rapidhash::v3::{rapidhash_v3_seeded, RapidSecrets};

/// Set your global hashing secrets.
/// - For HashDoS resistance, choose a randomised secret.
/// - For C++ compatibility, use the `seed_cpp` method or `DEFAULT_RAPID_SECRETS`.
const RAPID_SECRETS: RapidSecrets = RapidSecrets::seed(0x123456);

/// Make a helper function that sets your rapidhash version and secrets.
#[inline]
pub fn rapidhash(data: &[u8]) -> u64 {
    rapidhash_v3_seeded(data, &RAPID_SECRETS)
}

assert_eq!(rapidhash(b"hello world"), 11653223729569656151);
```

Please see the [`portable-hash` crate](https://github.com/hoxxep/portable-hash) for why using the standard library hashing traits is not recommended for portable hashing. Rapidhash is planning to implement the `PortableHash` and `PortableHasher` traits in a future release.

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

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash.svg)

<details>
<summary><strong>Benchmark suite, M1 Max (aarch64)</strong></summary>

Pure byte hashing performance doesn't make a great hasher by itself. In rust, it has to contend with the `std::hash::Hash` and `std::hash::Hasher` traits which may disrupt the inlining and optimisations. Hash quality also matters for hashmap insertions and lookups, so we need to consider the overall performance of the hasher in real-world scenarios.

Rapidhash uses the [foldhash benchmark suite](https://github.com/orlp/foldhash?tab=readme-ov-file#performance), detailed heavily in their repo. It benchmarks hashers by measuring raw hash throughput, hashmap lookup miss, hashmap lookup hit, and hashmap insertion performance on a wide variety of commonly hashed types.

```text
              ┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
              │         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
              ╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
              │       avg_rank ┆        2.08 ┆        4.11 ┆       3.31 ┆       5.08 ┆   4.69 ┆   3.20 ┆  5.56 ┆    7.97 │
              │ geometric_mean ┆        4.26 ┆        4.86 ┆       4.82 ┆       5.22 ┆   4.96 ┆   5.49 ┆  5.94 ┆   21.95 │
              └────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘

┌────────────────┬────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬─────────┬─────────┬─────────┐
│          distr ┆      bench ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆  fxhash ┆   ahash ┆ siphash │
╞════════════════╪════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪═════════╪═════════╪═════════╡
│            u32 ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.63 ┆       0.76 ┆   0.93 ┆    0.43 ┆    0.85 ┆    5.86 │
│            u32 ┆ lookupmiss ┆        1.48 ┆        1.74 ┆       1.45 ┆       1.75 ┆   2.07 ┆    1.30 ┆    1.89 ┆    7.67 │
│            u32 ┆  lookuphit ┆        1.85 ┆        2.19 ┆       1.83 ┆       2.19 ┆   2.05 ┆    1.64 ┆    2.32 ┆    7.45 │
│            u32 ┆   setbuild ┆        4.06 ┆        4.49 ┆       4.09 ┆       4.55 ┆   5.07 ┆    2.77 ┆    4.69 ┆    9.06 │
│        u32pair ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.62 ┆       0.76 ┆   1.09 ┆    0.78 ┆    1.26 ┆   10.60 │
│        u32pair ┆ lookupmiss ┆        1.64 ┆        1.79 ┆       1.51 ┆       1.74 ┆   2.18 ┆    1.90 ┆    2.76 ┆   12.08 │
│        u32pair ┆  lookuphit ┆        1.91 ┆        2.26 ┆       1.89 ┆       2.22 ┆   2.15 ┆    2.10 ┆    3.03 ┆   11.91 │
│        u32pair ┆   setbuild ┆        4.24 ┆        4.71 ┆       4.29 ┆       4.74 ┆   5.63 ┆    3.20 ┆    5.44 ┆   12.41 │
│            u64 ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.62 ┆       0.75 ┆   0.90 ┆    0.43 ┆    0.85 ┆    7.36 │
│            u64 ┆ lookupmiss ┆        1.48 ┆        1.72 ┆       1.47 ┆       1.75 ┆   1.92 ┆    1.40 ┆    1.80 ┆    9.31 │
│            u64 ┆  lookuphit ┆        1.87 ┆        2.18 ┆       1.84 ┆       2.18 ┆   1.97 ┆    1.64 ┆    2.32 ┆    9.30 │
│            u64 ┆   setbuild ┆        4.09 ┆        4.55 ┆       4.10 ┆       4.57 ┆   5.05 ┆    2.79 ┆    4.67 ┆   10.62 │
│      u64lobits ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.63 ┆       0.75 ┆   0.93 ┆    0.43 ┆    0.85 ┆    7.36 │
│      u64lobits ┆ lookupmiss ┆        1.46 ┆        1.74 ┆       1.46 ┆       1.86 ┆   1.90 ┆    1.36 ┆    1.79 ┆    9.32 │
│      u64lobits ┆  lookuphit ┆        1.89 ┆        2.18 ┆       1.84 ┆       2.18 ┆   1.96 ┆    1.57 ┆    2.31 ┆    9.27 │
│      u64lobits ┆   setbuild ┆        4.13 ┆        4.51 ┆       4.15 ┆       4.54 ┆   5.06 ┆    2.66 ┆    4.64 ┆   10.62 │
│      u64hibits ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.62 ┆       0.76 ┆   0.89 ┆    0.43 ┆    0.85 ┆    7.40 │
│      u64hibits ┆ lookupmiss ┆        1.68 ┆        1.72 ┆       1.45 ┆       1.82 ┆   1.91 ┆    0.94 ┆    1.79 ┆    9.31 │
│      u64hibits ┆  lookuphit ┆        1.86 ┆        2.19 ┆       1.82 ┆       2.17 ┆   1.95 ┆   75.23 ┆    2.32 ┆    9.31 │
│      u64hibits ┆   setbuild ┆        4.09 ┆        4.51 ┆       4.08 ┆       4.55 ┆   5.06 ┆  122.38 ┆    4.66 ┆   10.59 │
│        u64pair ┆   hashonly ┆        0.78 ┆        1.01 ┆       0.93 ┆       0.92 ┆   1.17 ┆    0.78 ┆    1.27 ┆   13.13 │
│        u64pair ┆ lookupmiss ┆        1.71 ┆        2.00 ┆       1.86 ┆       2.08 ┆   2.34 ┆    1.75 ┆    2.42 ┆   13.72 │
│        u64pair ┆  lookuphit ┆        2.17 ┆        2.50 ┆       2.14 ┆       2.51 ┆   2.50 ┆    2.18 ┆    3.12 ┆   13.20 │
│        u64pair ┆   setbuild ┆        4.34 ┆        4.75 ┆       4.34 ┆       4.75 ┆   5.75 ┆    3.21 ┆    5.51 ┆   14.04 │
│           ipv4 ┆   hashonly ┆        0.66 ┆        0.81 ┆       0.62 ┆       0.76 ┆   0.93 ┆    0.43 ┆    0.85 ┆    5.84 │
│           ipv4 ┆ lookupmiss ┆        1.47 ┆        1.80 ┆       1.46 ┆       1.97 ┆   1.95 ┆    1.30 ┆    1.87 ┆    7.69 │
│           ipv4 ┆  lookuphit ┆        1.84 ┆        2.18 ┆       1.83 ┆       2.17 ┆   2.02 ┆    1.73 ┆    2.32 ┆    7.51 │
│           ipv4 ┆   setbuild ┆        4.03 ┆        4.49 ┆       4.07 ┆       4.48 ┆   5.05 ┆    2.94 ┆    4.64 ┆    9.04 │
│           ipv6 ┆   hashonly ┆        0.70 ┆        0.85 ┆       0.78 ┆       0.92 ┆   0.89 ┆    0.78 ┆    1.24 ┆    8.90 │
│           ipv6 ┆ lookupmiss ┆        1.70 ┆        1.94 ┆       1.74 ┆       2.00 ┆   1.92 ┆    1.76 ┆    2.30 ┆   11.07 │
│           ipv6 ┆  lookuphit ┆        2.34 ┆        2.70 ┆       2.39 ┆       2.75 ┆   2.47 ┆    2.40 ┆    3.14 ┆   12.19 │
│           ipv6 ┆   setbuild ┆        4.24 ┆        4.70 ┆       4.32 ┆       4.74 ┆   5.08 ┆    3.21 ┆    5.25 ┆   12.63 │
│           rgba ┆   hashonly ┆        0.66 ┆        0.82 ┆       0.62 ┆       0.75 ┆   1.72 ┆    1.14 ┆    1.94 ┆   20.51 │
│           rgba ┆ lookupmiss ┆        1.68 ┆        1.85 ┆       1.70 ┆       1.81 ┆   3.24 ┆    2.31 ┆    3.28 ┆   15.74 │
│           rgba ┆  lookuphit ┆        2.49 ┆        2.94 ┆       2.51 ┆       2.92 ┆   3.56 ┆    3.14 ┆    4.39 ┆   14.44 │
│           rgba ┆   setbuild ┆        4.72 ┆        5.23 ┆       4.76 ┆       5.26 ┆   7.21 ┆    4.23 ┆    6.92 ┆   12.62 │
│ strenglishword ┆   hashonly ┆        1.56 ┆        2.17 ┆       5.55 ┆       3.30 ┆   1.62 ┆    2.15 ┆    2.46 ┆   12.13 │
│ strenglishword ┆ lookupmiss ┆        4.02 ┆        4.32 ┆       6.36 ┆       6.82 ┆   5.32 ┆    3.01 ┆    3.92 ┆   11.71 │
│ strenglishword ┆  lookuphit ┆        7.53 ┆        8.37 ┆       9.32 ┆       9.65 ┆   9.91 ┆    6.64 ┆    8.37 ┆   13.75 │
│ strenglishword ┆   setbuild ┆       14.77 ┆       15.43 ┆      16.68 ┆      17.17 ┆  15.02 ┆   15.48 ┆   13.33 ┆   20.23 │
│        struuid ┆   hashonly ┆        2.48 ┆        3.26 ┆       5.51 ┆       4.93 ┆   2.18 ┆    2.93 ┆    3.83 ┆   14.03 │
│        struuid ┆ lookupmiss ┆        5.66 ┆        6.36 ┆       8.00 ┆       8.40 ┆   6.54 ┆    4.84 ┆    5.87 ┆   16.23 │
│        struuid ┆  lookuphit ┆        8.78 ┆        9.86 ┆      11.93 ┆      12.41 ┆   9.72 ┆    7.75 ┆    9.68 ┆   19.57 │
│        struuid ┆   setbuild ┆       12.72 ┆       14.15 ┆      16.25 ┆      16.97 ┆  14.17 ┆   12.06 ┆   13.07 ┆   23.42 │
│         strurl ┆   hashonly ┆        4.85 ┆        5.62 ┆       7.40 ┆       7.58 ┆   3.66 ┆    8.59 ┆    7.30 ┆   29.45 │
│         strurl ┆ lookupmiss ┆        8.05 ┆        9.00 ┆       9.60 ┆      10.05 ┆   7.23 ┆   10.06 ┆    9.46 ┆   31.82 │
│         strurl ┆  lookuphit ┆       13.50 ┆       14.51 ┆      16.24 ┆      17.09 ┆  14.18 ┆   17.71 ┆   15.16 ┆   35.56 │
│         strurl ┆   setbuild ┆       20.69 ┆       21.97 ┆      22.80 ┆      23.63 ┆  22.14 ┆   29.40 ┆   21.37 ┆   44.74 │
│        strdate ┆   hashonly ┆        1.40 ┆        2.13 ┆       5.41 ┆       3.07 ┆   1.63 ┆    2.02 ┆    2.22 ┆   16.28 │
│        strdate ┆ lookupmiss ┆        4.07 ┆        4.68 ┆       6.22 ┆       6.55 ┆   5.45 ┆    3.58 ┆    4.05 ┆   12.19 │
│        strdate ┆  lookuphit ┆        6.37 ┆        6.76 ┆       9.18 ┆       8.19 ┆   6.97 ┆    5.81 ┆    6.71 ┆   13.72 │
│        strdate ┆   setbuild ┆        9.85 ┆       11.13 ┆      13.01 ┆      12.70 ┆  11.18 ┆    9.86 ┆    9.66 ┆   17.13 │
│      accesslog ┆   hashonly ┆        1.14 ┆        1.34 ┆       1.16 ┆       1.39 ┆   1.65 ┆    1.35 ┆    2.30 ┆   19.46 │
│      accesslog ┆ lookupmiss ┆        2.33 ┆        2.59 ┆       2.33 ┆       2.58 ┆   3.36 ┆    2.64 ┆    4.65 ┆   16.64 │
│      accesslog ┆  lookuphit ┆        3.27 ┆        3.65 ┆       3.21 ┆       3.56 ┆   3.59 ┆    3.82 ┆    5.80 ┆   16.42 │
│      accesslog ┆   setbuild ┆        5.56 ┆        6.05 ┆       5.44 ┆       6.06 ┆   7.07 ┆    4.79 ┆    7.82 ┆   16.65 │
│       kilobyte ┆   hashonly ┆       27.61 ┆       29.22 ┆      30.86 ┆      30.67 ┆  15.89 ┆  136.30 ┆   60.58 ┆  302.68 │
│       kilobyte ┆ lookupmiss ┆       29.97 ┆       33.77 ┆      33.18 ┆      33.80 ┆  20.02 ┆  142.19 ┆   63.88 ┆  308.25 │
│       kilobyte ┆  lookuphit ┆       68.60 ┆       73.71 ┆      77.26 ┆      78.58 ┆  65.03 ┆  237.54 ┆  112.90 ┆  359.90 │
│       kilobyte ┆   setbuild ┆      101.91 ┆      107.06 ┆     109.02 ┆     108.10 ┆  99.72 ┆  272.53 ┆  145.53 ┆  421.91 │
│    tenkilobyte ┆   hashonly ┆      233.22 ┆      234.98 ┆     314.18 ┆     314.46 ┆ 147.59 ┆ 1929.59 ┆  687.88 ┆ 3044.50 │
│    tenkilobyte ┆ lookupmiss ┆      238.34 ┆      244.21 ┆     317.83 ┆     316.75 ┆ 155.75 ┆ 1935.68 ┆  692.82 ┆ 3027.24 │
│    tenkilobyte ┆  lookuphit ┆      615.21 ┆      620.82 ┆     691.39 ┆     692.27 ┆ 523.78 ┆ 2350.41 ┆ 1061.37 ┆ 3328.88 │
│    tenkilobyte ┆   setbuild ┆     1061.78 ┆     1066.55 ┆    1115.56 ┆    1118.10 ┆ 962.11 ┆ 2781.46 ┆ 1391.92 ┆ 3959.01 │
└────────────────┴────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴─────────┴─────────┴─────────┘
```

</details>

<details>
<summary><strong>Benchmark notes</strong></summary>

- Hash throughput/latency does not measure hash "quality", and many of the benchmarked functions fail SMHasher3 quality tests. Hash quality affects hashmap performance, as well as algorithms that benefit from high quality hash functions such as HyperLogLog and MinHash.
- Most hash functions will be affected heavily by whether the compiler has inlined them. Rapidhash tries very hard to always be inlined by the compiler, but the larger a program or benchmark gets, the less likely it is to be inlined due to Rust's `BuildHasher::hash_one` method not being `#[inline(always)]`.
- `gxhash` has high throughput by using AES instructions. It's a great hash function, but is not a portable hash function (often requires `target-cpu=native` to compile), uses unsafe code, and is not minimally DoS resistant.
- Benchmark your own use case, with your real world dataset! We suggest experimenting with different hash functions to see which one works best for your use case. Rapidhash is great for fast general-purpose hashing, but certain hashers will outperform for specific use cases.

</details>

## Rapidhash Versions

### Portable Hashing
Fixed versioning with C++ compatibility is presented in `rapidhash::v1`, `rapidhash::v2`, and `rapidhash::v3` modules.

Rapidhash V3 is the recommended, fastest, and most recent version of the hash. Others are provided for backwards compatibility.

### In-Memory Hashing
Rust hasing traits (`RapidHasher`, `RapidBuildHasher`, etc.) are implemented in `rapidhash::fast`, `rapidhash::quality`, and `rapidhash::inner` modules. These are not guaranteed to give a consistent hash output between platforms, compiler versions, or crate versions as the rust `Hasher` trait [is not suitable](https://github.com/hoxxep/portable-hash/?tab=readme-ov-file#whats-wrong-with-the-stdhash-traits) for portable hashing.

- Use `rapidhash::fast` for optimal hashing speed with a slightly lower hash quality. Best for most datastructures such as HashMap and HashSet usage.
- Use `rapidhash::quality` where statistical hash quality is the priority, such as HyperLogLog or MinHash algorithms.
- Use `rapidhash::inner` to configure advanced parameters to configure the hash function specifically to your use case. This allows tweaking the following compile time parameters, which all change the hash output:
    - `AVALANCHE`: Enables the final avalanche mixing step to improve hash quality. Enabled on quality, disabled on fast.
    - `SPONGE`: Hash integer types by collecting them into a 128-bit buffer and mixing them together, rather than hashing each integer individually. If this is disabled, it will perform a folded multiply for each integer. Enabled by default.
    - `COMPACT`: Generates fewer instructions at compile time by reducing the manual loop unrolling. This might improve the probability of rapidhash being inlined, but may be slower on some platforms. Disabled by default.
    - `PROTECTED`: Slightly stronger hash quality and DoS resistance by performing two extra XOR instructions on every mix step. Disabled by default.

## Versioning
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
