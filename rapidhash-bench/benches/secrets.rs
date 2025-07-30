//! Secrets need to be accessed and passed around by hashers, this benchmark
//! tests the performance of various ways to clone them.
//!
//! This makes a strong argument that we should pass `RapidSecrets` by reference,
//! even if it makes the `RapidHasher` type need an annoying lifetime parameter.

use std::hint::black_box;
use std::rc::Rc;
use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Bencher, Criterion, Throughput};

fn bench_secrets(c: &mut Criterion) {
    let mut group = c.benchmark_group("secrets");

    group.throughput(Throughput::Elements(1));
    group.bench_function("copy/array", profile_copy_array);
    group.bench_function("copy/array_ref", profile_copy_array_ref);
    group.bench_function("copy/ref", profile_copy_ref);
    group.bench_function("copy/ref_all", profile_copy_ref_all);
    group.bench_function("copy/rc", profile_copy_rc);
    group.bench_function("copy/arc", profile_copy_arc);
}

#[derive(Copy, Clone)]
struct RapidSecrets {
    seed: u64,
    secret: [u64; 7],
}

#[derive(Copy, Clone)]
struct RapidSecretsRef<'a> {
    seed: u64,
    secret: &'a [u64; 7],
}

#[derive(Copy, Clone)]
struct RapidSecretsRefAll<'a> {
    secret: &'a [u64; 8],
}

#[derive(Clone)]
struct RapidSecretsRc {
    seed: u64,
    secret: Rc<[u64; 7]>,
}

#[derive(Clone)]
struct RapidSecretsArc {
    seed: u64,
    secret: Arc<[u64; 7]>,
}

fn profile_copy_array(b: &mut Bencher) {
    let secrets = RapidSecrets {
        seed: 0x123456789abcdef,
        secret: [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7],
    };

    b.iter(|| {
        let secrets: RapidSecrets = black_box(secrets);  // copy the result
        (secrets.seed ^ secrets.secret[0], secrets)
    });
}

fn profile_copy_array_ref(b: &mut Bencher) {
    let secrets = RapidSecrets {
        seed: 0x123456789abcdef,
        secret: [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7],
    };

    b.iter(|| {
        let secrets: &RapidSecrets = black_box(&secrets);  // ref the result
        (secrets.seed ^ secrets.secret[0], secrets)
    });
}

fn profile_copy_ref(b: &mut Bencher) {
    let array = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7];
    let secrets = RapidSecretsRef {
        seed: 0x123456789abcdef,
        secret: &array,
    };

    b.iter(|| {
        let secrets: RapidSecretsRef = black_box(secrets);  // ref the result
        (secrets.seed ^ secrets.secret[0], secrets)
    });
}

fn profile_copy_ref_all(b: &mut Bencher) {
    let array = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];
    let secrets = RapidSecretsRefAll {
        secret: &array,
    };

    b.iter(|| {
        let secrets: RapidSecretsRefAll = black_box(secrets);  // ref the result
        (secrets.secret[1] ^ secrets.secret[0], secrets)
    });
}

fn profile_copy_rc(b: &mut Bencher) {
    let array = Rc::new([0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]);
    let secrets = RapidSecretsRc {
        seed: 0x123456789abcdef,
        secret: array,
    };

    b.iter(|| {
        let secrets: RapidSecretsRc = black_box(secrets.clone());  // ref the result
        (secrets.seed ^ secrets.secret[0], secrets)
    });
}

fn profile_copy_arc(b: &mut Bencher) {
    let array = Arc::new([0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]);
    let secrets = RapidSecretsArc {
        seed: 0x123456789abcdef,
        secret: array,
    };

    b.iter(|| {
        let secrets: RapidSecretsArc = black_box(secrets.clone());  // ref the result
        (secrets.seed ^ secrets.secret[0], secrets)
    });
}

criterion_group!(
    benches,
    bench_secrets
);
criterion_main!(benches);
