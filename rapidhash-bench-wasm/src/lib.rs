use std::hash::BuildHasher;

use rapidhash::rng::rapidrng_fast;

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_rapidhash_q() -> u64 {
    profile_hash::<rapidhash::quality::RandomState>()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_rapidhash_f() -> u64 {
    profile_hash::<rapidhash::fast::RandomState>()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_foldhash_q() -> u64 {
    profile_hash::<foldhash::quality::RandomState>()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_foldhash_f() -> u64 {
    profile_hash::<foldhash::fast::RandomState>()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_default() -> u64 {
    profile_hash::<std::hash::RandomState>()
}

#[unsafe(no_mangle)]
pub extern "C" fn bench_wasm_fxhash() -> u64 {
    profile_hash::<fxhash::FxBuildHasher>()
}

/// Simulate hashing fictional (id, email) pairs, where email is len 6..30 bytes.
///
/// Lazy balanced test of what might get hashed on a frontend app.
///
/// TODO: 4kB hashing test.
fn profile_hash<B: BuildHasher + Default>() -> u64 {
    let builder = B::default();

    let mut seed = 0;
    let mut total = 0;

    for _ in 0..1_000 {
        let ratio = f64::from(rapidrng_fast(&mut seed) as u32) / f64::from(u32::MAX);
        let len = usize::max(6, (ratio * 30.0) as usize);
        let username = (0..len).map(|_| rapidrng_fast(&mut seed) as u8).collect::<Vec<u8>>();
        let num = rapidrng_fast(&mut seed);
        total ^= builder.hash_one((num, username));
    }

    total
}
