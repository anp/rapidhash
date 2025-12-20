//! Benchmarking RandomState, GlobalState, etc.

use std::hash::{BuildHasher, Hash, Hasher};
use std::hint::black_box;
use criterion::Criterion;
use rapidhash::fast::{GlobalState, RandomState, RapidHasher, SeedableState};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("state");

    group.bench_function("RandomState", |b| b.iter(|| RandomState::new().hash_one(black_box("hello"))));
    group.bench_function("GlobalState", |b| b.iter(|| GlobalState::new().hash_one(black_box("hello"))));
    group.bench_function("SeedableState", |b| b.iter(|| SeedableState::fixed().hash_one(black_box("hello"))));
    group.bench_function("RapidHasher", |b| b.iter(|| {
        let mut hasher = RapidHasher::default_const();
        black_box("hello").hash(&mut hasher);
        hasher.finish()
    }));
}
