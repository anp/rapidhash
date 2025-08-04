use criterion::{criterion_group, criterion_main};

mod basic;
mod emails;
mod rng;
mod compiled;

criterion_group!(
    benches,
    basic::bench,
    emails::bench,
    rng::bench,
    compiled::bench,
);
criterion_main!(benches);
