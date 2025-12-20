use criterion::{criterion_group, criterion_main};

mod basic;
mod emails;
mod rng;
mod compiled;
mod state;

criterion_group!(
    benches,
    basic::bench,
    emails::bench,
    rng::bench,
    compiled::bench,
    state::bench,
);
criterion_main!(benches);
