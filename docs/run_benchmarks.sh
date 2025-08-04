#!/bin/bash

set -euo pipefail

# Usage: ./docs/run_benchmarks.sh <chip>

CHIP=$1
FILTER=''

# The below filter should run only the graphed/tabled benchmarks if uncommented.
FILTER='hash/.*^(_raw|_rs|_cc_.*)/(str.*|u64)'

echo "Running on branch $(git rev-parse --abbrev-ref HEAD) with commit $(git rev-parse HEAD)"

# native benchmarks
echo "Starting native benchmarks for $CHIP..."
RUSTFLAGS="-C target-cpu=native" cargo criterion --bench realworld --all-features
RUSTFLAGS="-C target-cpu=native" cargo criterion --bench bench --all-features -- "$FILTER"

uv run docs/generate_table.py > "$HOME/bench_hash_${CHIP}_native.txt"
uv run docs/generate_charts.py
mv docs/bench_hash.svg "$HOME/bench_hash_${CHIP}_native.svg"

# portable benchmarks
echo "Starting portable benchmarks for $CHIP..."
cargo criterion --bench realworld --all-features
cargo criterion --bench bench --all-features -- "$FILTER"

uv run docs/generate_table.py --portable > "$HOME/bench_hash_${CHIP}.txt"
uv run docs/generate_charts.py --portable
mv docs/bench_hash.svg "$HOME/bench_hash_${CHIP}.svg"
