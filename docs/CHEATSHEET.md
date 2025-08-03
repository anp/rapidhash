# Repository Command Cheatsheet

Repositories are split into multiple crates:
- `rapidhash`: The main library crate.
- `rapidhash-c`: The original rapidhash C code, used for checking correctness.
- `rapidhash-bench`: A benchmark crate for running benchmarks, separates the benchmarking deps from MSRV tests.

## C++ Dependencies
We need a C++ compiler so that we can test and benchmark the C implementations of rapidhash at the same time. We do this to verify correctness and performance. Testing without `rapidhash-c` can be done by targeting the `-p rapidhash` crate directly, but the benchmarks currently require the C code to be built.

AWS helper script to set up a fresh Amazon Linux instance for benchmarking.
```shell
# install git and a C++ compiler
sudo yum install -y git gcc gcc-c++

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"

# install uv
curl -LsSf https://astral.sh/uv/install.sh | sh

# install cargo-criterion for benchmarks
cargo install cargo-criterion

# clone rapidhash
git clone https://github.com/hoxxep/rapidhash.git
cd rapidhash

# run tests
cargo test --all-features
```

## Running Tests
```shell
## Tests

# Run tests
cargo test --all-features

# Run tests, for no_std with std = off and unsafe = off
cargo test --no-default-features --lib

# Check MSRV
cargo +1.77.0 test --all-features
```

## Running benchmarks
Benchmarks are run using `cargo-criterion` in the `rapidhash-bench` crate to separate them from the library.

```shell
# Run in the bench crate
cd rapidhash-bench

# Run all benchmarks (assumes cargo-criterion is installed)
RUSTFLAGS="-C target-cpu=native" cargo criterion --bench bench --all-features

# Run all benchmarks, but unsafe=disabled
RUSTFLAGS="-C target-cpu=native" cargo criterion --bench bench --features bench

# Run the realworld benchmark, which is a modification of the foldhash benchmarks
RUSTFLAGS="-C target-cpu=native" cargo criterion --profile bench --bench realworld --all-features

# Run quality tests across various hash functions
RUSTFLAGS="-C target-cpu=native" cargo bench --bench quality --all-features

# Run iai-callgrind to compare instruction counts and L1 cache misses
# Requires: valgrind
RUSTFLAGS="-C target-cpu=native" cargo bench --bench iai-callgrind --all-features

# Use cargo-instruments to diagnose performance
# Requires: cargo-instruments and MacOS
RUSTFLAGS="-C target-cpu=native" cargo instruments -t time --profile=bench --bench realworld --features bench,unsafe -- --bench hashonly-struuid-rapidhash-v2

# Benchmark WASM targets, which will automatically build the WASM target
RUSTFLAGS="-C target-cpu=native" cargo criterion --bench wasm --all-features
```

## Fuzzing
```shell
# fuzz the raw rapidhash method. (assumes cargo-fuzz is installed)
cargo +nightly fuzz run --features unsafe rapidhash

# fuzz the RapidHasher struct with std::hash::Hasher write and finish calls.
cargo +nightly fuzz run --features unsafe rapidhasher

# use AFL fuzzing. (assumes cargo-afl is installed)
cargo afl fuzz -i in -o out target/debug/afl_rapidhash
```

## Documentation
```shell
# Install cargo-docs
cargo install cargo-docs

# Preview the documentation
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly docs -- --all-features
```

## CLI
```shell
# From stdin
echo "example" | cargo run --example cli

# From file
cargo run --example cli -- example.txt
```

## Report Benchmark Data
Our python scripts have the python dependencies listed at the top of the file in the uv script format to make running them easy without needing to manually set up a virtual environment.

```shell
# generate the --bench bench charts
uv run generate_charts.py

# generate the --bench realworld table
uv run generate_table.py
```
