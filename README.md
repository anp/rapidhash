# rapidhash - rust implementation

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
assert_eq!(hasher.hash_one(b"hello world"), 1790036888308448300);
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
pub fn rapidhash_stream<R: std::io::Read>(reader: R) -> u64 {
    rapidhash_v3_file_seeded(reader, &RAPID_SECRETS)
}

assert_eq!(rapidhash(b"hello world"), 11653223729569656151);
assert_eq!(rapidhash_stream(std::io::Cursor::new(b"hello world")), 11653223729569656151);
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

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_aarch64_m1_max.svg)

<details>
<summary><strong>Benchmark suite, M1 Max (aarch64)</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_aarch64_m1_max.svg)

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
<summary><strong>Benchmark suite, Intel Xeon Platinum 8488C (x86_64)</strong></summary>

With the `target-cpu = native` option, hashers like gxhash and ahash perform well. Without adding compiler flags for certain features, ahash is slower, and gxhash fails to compile entirely.

Rapidhash and foldhash should be almost identical for integer types and integer tuples, but for some reason rapidhash isn't being inlined for `rgba` or `accesslog` in this benchmark suite. This is potentially a quirk of LLVM inlining in this benchmarking suite, for example, the default SipHasher was unexpectedly faster when not compiling with gxhash for the no CPU feature benchmarks. More testing will be done to address this in a future release.

<details>
<summary><strong>Intel Xeon, with target-cpu = native</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_x86_64_intel_xeon_8488c.svg)

```text
             ┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬────────┬───────┬─────────┐
             │         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ gxhash ┆ fxhash ┆ ahash ┆ siphash │
             ╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪════════╪═══════╪═════════╡
             │       avg_rank ┆        2.84 ┆        5.08 ┆       3.27 ┆       5.22 ┆   3.86 ┆   3.23 ┆  4.53 ┆    7.97 │
             │ geometric_mean ┆        5.18 ┆        5.98 ┆       5.10 ┆       5.69 ┆   4.98 ┆   5.87 ┆  5.54 ┆   22.38 │
             └────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴────────┴───────┴─────────┘

┌────────────────┬────────────┬─────────────┬─────────────┬────────────┬────────────┬─────────┬─────────┬─────────┬─────────┐
│          distr ┆      bench ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆  gxhash ┆  fxhash ┆   ahash ┆ siphash │
╞════════════════╪════════════╪═════════════╪═════════════╪════════════╪════════════╪═════════╪═════════╪═════════╪═════════╡
│            u32 ┆   hashonly ┆        0.69 ┆        0.83 ┆       0.70 ┆       0.84 ┆    0.74 ┆    0.56 ┆    1.11 ┆    6.44 │
│            u32 ┆ lookupmiss ┆        1.72 ┆        1.86 ┆       1.50 ┆       1.85 ┆    1.86 ┆    1.26 ┆    2.02 ┆    8.90 │
│            u32 ┆  lookuphit ┆        2.24 ┆        2.65 ┆       2.23 ┆       2.63 ┆    2.74 ┆    1.91 ┆    2.70 ┆    8.72 │
│            u32 ┆   setbuild ┆        4.04 ┆        4.45 ┆       4.01 ┆       4.42 ┆    4.70 ┆    2.86 ┆    5.27 ┆    9.80 │
│        u32pair ┆   hashonly ┆        0.69 ┆        0.83 ┆       0.69 ┆       0.84 ┆    0.97 ┆    0.70 ┆    1.39 ┆   10.90 │
│        u32pair ┆ lookupmiss ┆        1.94 ┆        2.18 ┆       1.93 ┆       2.21 ┆    2.36 ┆    2.00 ┆    2.68 ┆   13.16 │
│        u32pair ┆  lookuphit ┆        2.16 ┆        2.59 ┆       2.15 ┆       2.57 ┆    3.28 ┆    2.17 ┆    2.93 ┆   13.38 │
│        u32pair ┆   setbuild ┆        4.18 ┆        4.51 ┆       4.17 ┆       4.54 ┆    5.41 ┆    3.27 ┆    5.76 ┆   12.35 │
│            u64 ┆   hashonly ┆        0.83 ┆        0.84 ┆       0.83 ┆       0.83 ┆    0.74 ┆    0.56 ┆    1.12 ┆    7.88 │
│            u64 ┆ lookupmiss ┆        1.40 ┆        1.87 ┆       1.43 ┆       1.90 ┆    1.86 ┆    1.26 ┆    1.70 ┆   10.43 │
│            u64 ┆  lookuphit ┆        2.24 ┆        2.65 ┆       2.24 ┆       2.62 ┆    2.74 ┆    1.91 ┆    2.58 ┆   10.82 │
│            u64 ┆   setbuild ┆        4.01 ┆        4.47 ┆       4.00 ┆       4.43 ┆    4.67 ┆    2.86 ┆    5.50 ┆   11.41 │
│      u64lobits ┆   hashonly ┆        0.69 ┆        0.84 ┆       0.70 ┆       0.84 ┆    0.75 ┆    0.56 ┆    1.11 ┆    7.89 │
│      u64lobits ┆ lookupmiss ┆        1.55 ┆        1.94 ┆       1.54 ┆       1.86 ┆    1.89 ┆    1.25 ┆    1.78 ┆   10.46 │
│      u64lobits ┆  lookuphit ┆        2.20 ┆        2.63 ┆       2.20 ┆       2.62 ┆    2.74 ┆    1.85 ┆    2.59 ┆   10.86 │
│      u64lobits ┆   setbuild ┆        3.99 ┆        4.45 ┆       3.98 ┆       4.53 ┆    4.71 ┆    2.79 ┆    5.44 ┆   11.50 │
│      u64hibits ┆   hashonly ┆        0.70 ┆        0.84 ┆       0.69 ┆       0.83 ┆    0.74 ┆    0.56 ┆    1.16 ┆    7.88 │
│      u64hibits ┆ lookupmiss ┆        1.41 ┆        1.88 ┆       1.49 ┆       1.85 ┆    1.96 ┆    1.41 ┆    1.72 ┆   10.44 │
│      u64hibits ┆  lookuphit ┆        2.25 ┆        2.66 ┆       2.24 ┆       2.63 ┆    2.73 ┆   39.65 ┆    2.60 ┆   10.85 │
│      u64hibits ┆   setbuild ┆        3.99 ┆        4.39 ┆       3.96 ┆       4.46 ┆    4.61 ┆   93.27 ┆    5.36 ┆   11.40 │
│        u64pair ┆   hashonly ┆        0.83 ┆        1.00 ┆       0.83 ┆       1.11 ┆    0.97 ┆    0.65 ┆    1.40 ┆   11.01 │
│        u64pair ┆ lookupmiss ┆        2.18 ┆        2.48 ┆       2.13 ┆       2.45 ┆    2.13 ┆    2.34 ┆    2.42 ┆   13.57 │
│        u64pair ┆  lookuphit ┆        2.55 ┆        3.00 ┆       2.56 ┆       3.01 ┆    3.29 ┆    2.57 ┆    3.43 ┆   14.64 │
│        u64pair ┆   setbuild ┆        4.41 ┆        4.82 ┆       4.38 ┆       4.81 ┆    5.52 ┆    3.38 ┆    6.17 ┆   14.17 │
│           ipv4 ┆   hashonly ┆        0.69 ┆        0.83 ┆       0.69 ┆       0.83 ┆    0.74 ┆    0.56 ┆    1.12 ┆    6.45 │
│           ipv4 ┆ lookupmiss ┆        1.48 ┆        1.84 ┆       1.51 ┆       1.84 ┆    1.88 ┆    1.31 ┆    1.97 ┆    8.91 │
│           ipv4 ┆  lookuphit ┆        2.25 ┆        2.65 ┆       2.23 ┆       2.63 ┆    2.74 ┆    1.97 ┆    2.70 ┆    8.76 │
│           ipv4 ┆   setbuild ┆        3.93 ┆        4.41 ┆       3.96 ┆       4.42 ┆    4.67 ┆    2.92 ┆    5.24 ┆    9.83 │
│           ipv6 ┆   hashonly ┆        0.83 ┆        0.98 ┆       0.75 ┆       0.99 ┆    0.75 ┆    0.69 ┆    1.22 ┆    9.61 │
│           ipv6 ┆ lookupmiss ┆        1.61 ┆        2.00 ┆       1.76 ┆       2.09 ┆    1.96 ┆    1.77 ┆    1.93 ┆   12.36 │
│           ipv6 ┆  lookuphit ┆        2.69 ┆        3.14 ┆       2.79 ┆       3.31 ┆    3.00 ┆    2.79 ┆    3.03 ┆   13.57 │
│           ipv6 ┆   setbuild ┆        4.53 ┆        5.01 ┆       4.63 ┆       5.13 ┆    5.12 ┆    3.63 ┆    5.78 ┆   14.26 │
│           rgba ┆   hashonly ┆        4.98 ┆        5.15 ┆       0.83 ┆       0.83 ┆    1.69 ┆    1.20 ┆    2.35 ┆   12.54 │
│           rgba ┆ lookupmiss ┆        6.72 ┆        7.94 ┆       2.14 ┆       2.51 ┆    3.59 ┆    2.80 ┆    3.62 ┆   14.40 │
│           rgba ┆  lookuphit ┆        6.89 ┆        8.00 ┆       2.57 ┆       2.94 ┆    4.29 ┆    3.55 ┆    4.08 ┆   15.86 │
│           rgba ┆   setbuild ┆        4.98 ┆        5.39 ┆       4.56 ┆       5.00 ┆    6.59 ┆    4.50 ┆    7.13 ┆   12.32 │
│ strenglishword ┆   hashonly ┆        1.79 ┆        2.54 ┆       3.67 ┆       3.84 ┆    1.50 ┆    2.45 ┆    2.24 ┆    9.84 │
│ strenglishword ┆ lookupmiss ┆        3.75 ┆        4.58 ┆       6.18 ┆       6.62 ┆    4.55 ┆    3.92 ┆    3.07 ┆   11.71 │
│ strenglishword ┆  lookuphit ┆        6.45 ┆        7.86 ┆       8.38 ┆       8.98 ┆    6.71 ┆    6.27 ┆    5.44 ┆   16.31 │
│ strenglishword ┆   setbuild ┆       11.15 ┆       12.35 ┆      13.51 ┆      14.23 ┆    9.28 ┆   16.84 ┆   12.14 ┆   24.07 │
│        struuid ┆   hashonly ┆        3.60 ┆        3.91 ┆       5.20 ┆       5.63 ┆    1.98 ┆    3.28 ┆    2.90 ┆   14.23 │
│        struuid ┆ lookupmiss ┆        6.40 ┆        7.43 ┆       8.60 ┆       9.15 ┆    5.60 ┆    5.78 ┆    4.57 ┆   16.77 │
│        struuid ┆  lookuphit ┆        8.45 ┆       10.27 ┆      11.38 ┆      12.04 ┆    8.02 ┆    8.77 ┆    6.76 ┆   21.02 │
│        struuid ┆   setbuild ┆       12.80 ┆       14.92 ┆      15.97 ┆      16.74 ┆   11.73 ┆   12.31 ┆   12.71 ┆   26.44 │
│         strurl ┆   hashonly ┆        5.14 ┆        6.30 ┆       7.29 ┆       7.69 ┆    3.39 ┆    8.44 ┆    5.29 ┆   26.44 │
│         strurl ┆ lookupmiss ┆        7.58 ┆        8.83 ┆       9.76 ┆      10.22 ┆    6.06 ┆   10.04 ┆    6.57 ┆   27.91 │
│         strurl ┆  lookuphit ┆       12.91 ┆       14.60 ┆      14.88 ┆      15.66 ┆   11.01 ┆   15.86 ┆   11.17 ┆   36.93 │
│         strurl ┆   setbuild ┆       22.75 ┆       24.37 ┆      23.77 ┆      24.48 ┆   21.11 ┆   33.38 ┆   19.87 ┆   54.16 │
│        strdate ┆   hashonly ┆        1.72 ┆        2.57 ┆       3.61 ┆       3.64 ┆    1.43 ┆    3.03 ┆    2.24 ┆    9.54 │
│        strdate ┆ lookupmiss ┆        4.31 ┆        5.20 ┆       6.61 ┆       6.99 ┆    4.78 ┆    5.08 ┆    3.58 ┆   12.23 │
│        strdate ┆  lookuphit ┆        6.24 ┆        7.66 ┆       7.88 ┆       8.39 ┆    6.46 ┆    7.64 ┆    5.27 ┆   15.92 │
│        strdate ┆   setbuild ┆        9.18 ┆       10.49 ┆      11.36 ┆      11.88 ┆    9.22 ┆   10.23 ┆   10.57 ┆   19.07 │
│      accesslog ┆   hashonly ┆        4.92 ┆        5.08 ┆       1.39 ┆       1.69 ┆    1.71 ┆    1.49 ┆    1.90 ┆   19.79 │
│      accesslog ┆ lookupmiss ┆        6.67 ┆        8.47 ┆       2.88 ┆       3.32 ┆    3.50 ┆    3.40 ┆    3.47 ┆   24.82 │
│      accesslog ┆  lookuphit ┆        8.13 ┆        9.60 ┆       4.58 ┆       5.08 ┆    5.47 ┆    5.21 ┆    5.12 ┆   31.22 │
│      accesslog ┆   setbuild ┆        6.60 ┆        7.17 ┆       6.15 ┆       6.64 ┆    7.62 ┆    5.70 ┆    7.72 ┆   19.42 │
│       kilobyte ┆   hashonly ┆       29.62 ┆       32.53 ┆      31.80 ┆      32.37 ┆   16.32 ┆  137.14 ┆   25.32 ┆  222.72 │
│       kilobyte ┆ lookupmiss ┆       34.92 ┆       39.31 ┆      36.01 ┆      36.91 ┆   19.98 ┆  144.41 ┆   28.21 ┆  225.93 │
│       kilobyte ┆  lookuphit ┆       74.76 ┆       79.83 ┆      74.36 ┆      75.49 ┆   56.41 ┆  184.88 ┆   68.85 ┆  256.80 │
│       kilobyte ┆   setbuild ┆      135.40 ┆      141.35 ┆     140.09 ┆     140.96 ┆  107.36 ┆  255.51 ┆  134.82 ┆  351.00 │
│    tenkilobyte ┆   hashonly ┆      345.12 ┆      346.54 ┆     388.45 ┆     387.31 ┆  305.13 ┆ 1742.88 ┆  356.14 ┆ 2154.61 │
│    tenkilobyte ┆ lookupmiss ┆      351.15 ┆      355.04 ┆     397.39 ┆     397.91 ┆  314.16 ┆ 1750.23 ┆  363.86 ┆ 2158.98 │
│    tenkilobyte ┆  lookuphit ┆      721.05 ┆      721.82 ┆     762.95 ┆     765.26 ┆  691.95 ┆ 2146.77 ┆  733.43 ┆ 2537.38 │
│    tenkilobyte ┆   setbuild ┆     1443.45 ┆     1415.95 ┆    1532.52 ┆    1577.09 ┆ 1286.80 ┆ 2711.33 ┆ 1433.48 ┆ 3217.17 │
└────────────────┴────────────┴─────────────┴─────────────┴────────────┴────────────┴─────────┴─────────┴─────────┴─────────┘
```

</details>

<details>
<summary><strong>Intel Xeon, without CPU feature flags</strong></summary>

![Hashing Benchmarks](https://github.com/hoxxep/rapidhash/raw/master/docs/bench_hash_x86_64_intel_xeon_8488c_portable.svg)

```text
             ┌────────────────┬─────────────┬─────────────┬────────────┬────────────┬────────┬───────┬─────────┐
             │         metric ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆ fxhash ┆ ahash ┆ siphash │
             ╞════════════════╪═════════════╪═════════════╪════════════╪════════════╪════════╪═══════╪═════════╡
             │       avg_rank ┆        2.12 ┆        4.02 ┆       2.89 ┆       4.47 ┆   2.66 ┆  4.88 ┆    6.97 │
             │ geometric_mean ┆        5.30 ┆        6.09 ┆       5.23 ┆       5.82 ┆   5.96 ┆  6.65 ┆   21.24 │
             └────────────────┴─────────────┴─────────────┴────────────┴────────────┴────────┴───────┴─────────┘

┌────────────────┬────────────┬─────────────┬─────────────┬────────────┬────────────┬─────────┬─────────┬─────────┐
│          distr ┆      bench ┆ rapidhash-f ┆ rapidhash-q ┆ foldhash-f ┆ foldhash-q ┆  fxhash ┆   ahash ┆ siphash │
╞════════════════╪════════════╪═════════════╪═════════════╪════════════╪════════════╪═════════╪═════════╪═════════╡
│            u32 ┆   hashonly ┆        0.83 ┆        0.87 ┆       0.70 ┆       0.87 ┆    0.56 ┆    0.99 ┆    6.18 │
│            u32 ┆ lookupmiss ┆        1.71 ┆        2.03 ┆       1.68 ┆       2.05 ┆    1.49 ┆    2.24 ┆    8.72 │
│            u32 ┆  lookuphit ┆        2.28 ┆        2.71 ┆       2.29 ┆       2.68 ┆    1.99 ┆    2.91 ┆    8.35 │
│            u32 ┆   setbuild ┆        4.00 ┆        4.39 ┆       4.01 ┆       4.40 ┆    2.90 ┆    4.56 ┆    9.53 │
│        u32pair ┆   hashonly ┆        0.69 ┆        0.87 ┆       0.70 ┆       0.87 ┆    0.69 ┆    1.36 ┆    9.89 │
│        u32pair ┆ lookupmiss ┆        2.17 ┆        2.50 ┆       2.16 ┆       2.49 ┆    2.16 ┆    3.17 ┆   12.11 │
│        u32pair ┆  lookuphit ┆        2.23 ┆        2.62 ┆       2.24 ┆       2.60 ┆    2.28 ┆    3.33 ┆   12.14 │
│        u32pair ┆   setbuild ┆        4.25 ┆        4.58 ┆       4.25 ┆       4.59 ┆    3.29 ┆    5.27 ┆   11.90 │
│            u64 ┆   hashonly ┆        0.69 ┆        0.87 ┆       0.70 ┆       0.87 ┆    0.56 ┆    0.99 ┆    7.72 │
│            u64 ┆ lookupmiss ┆        1.75 ┆        2.10 ┆       1.70 ┆       2.04 ┆    1.51 ┆    2.24 ┆    9.68 │
│            u64 ┆  lookuphit ┆        2.29 ┆        2.71 ┆       2.29 ┆       2.69 ┆    2.02 ┆    2.92 ┆    9.67 │
│            u64 ┆   setbuild ┆        4.05 ┆        4.43 ┆       4.02 ┆       4.45 ┆    2.94 ┆    4.58 ┆   11.14 │
│      u64lobits ┆   hashonly ┆        0.83 ┆        0.87 ┆       0.70 ┆       0.89 ┆    0.56 ┆    0.99 ┆    7.65 │
│      u64lobits ┆ lookupmiss ┆        1.69 ┆        2.05 ┆       1.73 ┆       2.04 ┆    1.56 ┆    2.50 ┆    9.70 │
│      u64lobits ┆  lookuphit ┆        2.29 ┆        2.71 ┆       2.29 ┆       2.70 ┆    1.94 ┆    2.91 ┆    9.68 │
│      u64lobits ┆   setbuild ┆        4.02 ┆        4.48 ┆       4.07 ┆       4.48 ┆    2.85 ┆    4.62 ┆   11.19 │
│      u64hibits ┆   hashonly ┆        0.83 ┆        0.87 ┆       0.84 ┆       0.87 ┆    0.56 ┆    0.99 ┆    7.64 │
│      u64hibits ┆ lookupmiss ┆        1.69 ┆        2.13 ┆       1.70 ┆       2.12 ┆    1.23 ┆    2.27 ┆    9.70 │
│      u64hibits ┆  lookuphit ┆        2.28 ┆        2.69 ┆       2.30 ┆       2.70 ┆   50.78 ┆    2.92 ┆    9.67 │
│      u64hibits ┆   setbuild ┆        3.98 ┆        4.46 ┆       4.05 ┆       4.44 ┆   88.86 ┆    4.59 ┆   11.18 │
│        u64pair ┆   hashonly ┆        0.78 ┆        0.98 ┆       0.83 ┆       1.03 ┆    0.65 ┆    1.38 ┆   10.57 │
│        u64pair ┆ lookupmiss ┆        2.34 ┆        2.67 ┆       2.37 ┆       2.88 ┆    2.37 ┆    3.39 ┆   13.06 │
│        u64pair ┆  lookuphit ┆        2.60 ┆        3.00 ┆       2.60 ┆       2.96 ┆    2.63 ┆    3.84 ┆   13.34 │
│        u64pair ┆   setbuild ┆        4.36 ┆        4.82 ┆       4.40 ┆       4.86 ┆    3.41 ┆    5.42 ┆   13.80 │
│           ipv4 ┆   hashonly ┆        0.70 ┆        0.87 ┆       0.83 ┆       0.88 ┆    0.56 ┆    0.99 ┆    6.17 │
│           ipv4 ┆ lookupmiss ┆        1.74 ┆        2.03 ┆       1.68 ┆       2.04 ┆    1.44 ┆    2.23 ┆    8.70 │
│           ipv4 ┆  lookuphit ┆        2.29 ┆        2.69 ┆       2.32 ┆       2.69 ┆    2.06 ┆    2.91 ┆    8.36 │
│           ipv4 ┆   setbuild ┆        4.03 ┆        4.45 ┆       4.03 ┆       4.43 ┆    3.07 ┆    4.57 ┆    9.51 │
│           ipv6 ┆   hashonly ┆        0.83 ┆        0.99 ┆       0.78 ┆       0.98 ┆    0.65 ┆    1.41 ┆    8.72 │
│           ipv6 ┆ lookupmiss ┆        1.93 ┆        2.22 ┆       1.98 ┆       2.35 ┆    1.96 ┆    2.84 ┆   11.39 │
│           ipv6 ┆  lookuphit ┆        2.69 ┆        3.11 ┆       2.79 ┆       3.21 ┆    2.75 ┆    3.70 ┆   11.99 │
│           ipv6 ┆   setbuild ┆        4.48 ┆        4.94 ┆       4.58 ┆       5.03 ┆    3.61 ┆    5.62 ┆   13.82 │
│           rgba ┆   hashonly ┆        5.16 ┆        5.38 ┆       0.69 ┆       0.87 ┆    1.22 ┆    2.22 ┆   12.64 │
│           rgba ┆ lookupmiss ┆        6.99 ┆        8.43 ┆       2.31 ┆       2.69 ┆    3.04 ┆    4.38 ┆   15.15 │
│           rgba ┆  lookuphit ┆        6.90 ┆        8.35 ┆       2.61 ┆       3.03 ┆    3.58 ┆    4.95 ┆   15.97 │
│           rgba ┆   setbuild ┆        5.00 ┆        5.38 ┆       4.60 ┆       4.92 ┆    4.52 ┆    6.93 ┆   12.19 │
│ strenglishword ┆   hashonly ┆        1.85 ┆        2.54 ┆       3.66 ┆       3.88 ┆    2.62 ┆    2.75 ┆    9.84 │
│ strenglishword ┆ lookupmiss ┆        3.95 ┆        4.76 ┆       6.57 ┆       6.69 ┆    3.72 ┆    4.22 ┆   11.83 │
│ strenglishword ┆  lookuphit ┆        6.02 ┆        7.30 ┆       8.24 ┆       8.85 ┆    6.24 ┆    7.02 ┆   16.48 │
│ strenglishword ┆   setbuild ┆       11.03 ┆       12.34 ┆      14.06 ┆      14.57 ┆   16.52 ┆   11.06 ┆   23.88 │
│        struuid ┆   hashonly ┆        3.08 ┆        3.91 ┆       5.31 ┆       5.46 ┆    2.80 ┆    4.29 ┆   12.56 │
│        struuid ┆ lookupmiss ┆        6.62 ┆        7.43 ┆       8.79 ┆       9.11 ┆    5.39 ┆    6.78 ┆   15.60 │
│        struuid ┆  lookuphit ┆        8.35 ┆       10.07 ┆      11.16 ┆      11.83 ┆    7.90 ┆   10.11 ┆   20.52 │
│        struuid ┆   setbuild ┆       12.80 ┆       14.82 ┆      15.74 ┆      16.53 ┆   11.41 ┆   14.43 ┆   25.65 │
│         strurl ┆   hashonly ┆        5.01 ┆        6.10 ┆       7.42 ┆       7.66 ┆    8.10 ┆    7.28 ┆   24.48 │
│         strurl ┆ lookupmiss ┆        7.40 ┆        8.68 ┆       9.93 ┆      10.35 ┆   10.02 ┆    8.97 ┆   26.67 │
│         strurl ┆  lookuphit ┆       12.34 ┆       13.97 ┆      14.71 ┆      15.51 ┆   15.49 ┆   14.73 ┆   34.72 │
│         strurl ┆   setbuild ┆       23.25 ┆       24.91 ┆      24.33 ┆      25.19 ┆   33.41 ┆   22.97 ┆   53.53 │
│        strdate ┆   hashonly ┆        1.76 ┆        2.49 ┆       3.42 ┆       3.62 ┆    3.47 ┆    2.59 ┆    9.68 │
│        strdate ┆ lookupmiss ┆        4.49 ┆        5.29 ┆       6.77 ┆       6.99 ┆    5.14 ┆    4.68 ┆   12.05 │
│        strdate ┆  lookuphit ┆        6.06 ┆        7.44 ┆       7.74 ┆       8.59 ┆    7.16 ┆    6.97 ┆   15.38 │
│        strdate ┆   setbuild ┆        8.87 ┆       10.30 ┆      11.21 ┆      11.79 ┆    9.97 ┆    9.58 ┆   18.18 │
│      accesslog ┆   hashonly ┆        5.04 ┆        5.18 ┆       1.38 ┆       1.65 ┆    1.48 ┆    2.74 ┆   19.56 │
│      accesslog ┆ lookupmiss ┆        6.95 ┆        8.79 ┆       3.17 ┆       3.50 ┆    3.69 ┆    5.53 ┆   20.66 │
│      accesslog ┆  lookuphit ┆        8.06 ┆        9.57 ┆       4.53 ┆       4.86 ┆    5.13 ┆    7.43 ┆   19.70 │
│      accesslog ┆   setbuild ┆        6.57 ┆        7.20 ┆       6.08 ┆       6.61 ┆    5.76 ┆    8.73 ┆   17.76 │
│       kilobyte ┆   hashonly ┆       32.38 ┆       35.49 ┆      34.94 ┆      35.19 ┆  137.90 ┆   69.94 ┆  214.41 │
│       kilobyte ┆ lookupmiss ┆       33.80 ┆       37.71 ┆      36.16 ┆      36.88 ┆  145.30 ┆   71.41 ┆  216.04 │
│       kilobyte ┆  lookuphit ┆       73.56 ┆       78.33 ┆      73.71 ┆      75.19 ┆  187.49 ┆  110.28 ┆  249.47 │
│       kilobyte ┆   setbuild ┆      134.36 ┆      139.41 ┆     140.28 ┆     141.00 ┆  254.08 ┆  183.97 ┆  339.18 │
│    tenkilobyte ┆   hashonly ┆      340.48 ┆      349.72 ┆     390.77 ┆     390.46 ┆ 1746.86 ┆  775.93 ┆ 2069.57 │
│    tenkilobyte ┆ lookupmiss ┆      352.01 ┆      357.61 ┆     401.82 ┆     403.12 ┆ 1751.79 ┆  784.04 ┆ 2077.96 │
│    tenkilobyte ┆  lookuphit ┆      718.53 ┆      724.68 ┆     764.16 ┆     769.16 ┆ 2144.67 ┆ 1161.51 ┆ 2452.90 │
│    tenkilobyte ┆   setbuild ┆     1450.45 ┆     1423.77 ┆    1587.08 ┆    1594.35 ┆ 2722.63 ┆ 1836.10 ┆ 3089.42 │
└────────────────┴────────────┴─────────────┴─────────────┴────────────┴────────────┴─────────┴─────────┴─────────┘
```

</details>

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
