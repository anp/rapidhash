use std::collections::HashMap;
use std::hint::black_box;
use std::path::Path;
use std::process::Command;
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use wasmtime::*;

const WASM_PATH: &str = "target/wasm32-unknown-unknown/release/rapidhash_bench_wasm.wasm";

const HASHES: &[&str] = &[
    "rapidhash_f",
    "rapidhash_q",
    "foldhash_f",
    "foldhash_q",
    "default",
    "fxhash",
];

pub fn wasm_bench(c: &mut Criterion) {
    compile_wasm();

    let mut group = c.benchmark_group("wasmtime");
    group.sampling_mode(criterion::SamplingMode::Flat);

    for &hash in HASHES.iter() {
        group.bench_function(hash, profile_hash(hash));
    }
}

fn profile_hash(hash: &str) -> impl Fn(&mut Bencher) + use<'_> {
    |b| {
        let mut env = WasmEnv::new();
        let hash = hash.to_string();
        b.iter(move || {
            black_box(env.profile(&hash))
        });
    }
}

struct WasmEnv {
    store: Store<()>,
    hashes: HashMap<&'static str, TypedFunc<(), u64>>,
}

impl WasmEnv {
    fn new() -> Self {
        let engine = Engine::default();
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        let module_path = cwd.join(WASM_PATH);
        let module = Module::from_file(&engine, module_path)
            .expect("Failed to load WASM module, run: cargo build -p rapidhash-bench-wasm --release --target wasm32-unknown-unknown");
        let linker = Linker::new(&engine);
        let mut store = Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &module).unwrap();

        let mut hashes = HashMap::new();
        for &hash in HASHES.iter() {
            let func = instance.get_typed_func::<(), u64>(&mut store, &format!("bench_wasm_{hash}")).unwrap();
            hashes.insert(hash, func);
        }

        Self {
            store,
            hashes,
        }
    }

    fn profile(&mut self, hash: &str) -> u64 {
        let hash_fn = self.hashes.get(hash).unwrap();
        hash_fn.call(&mut self.store, ()).unwrap()
    }
}

/// Ensure we've compiled the WASM binary before running benchmarks
fn compile_wasm() {
    let out_path = Path::new(WASM_PATH);

    // Run cargo build --target
    let status = Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "--package", "rapidhash-bench-wasm",
            "--target", "wasm32-unknown-unknown",
        ])
        .env("RUSTFLAGS", "")  // clear any bench flags
        .status()
        .expect("Failed to run cargo build for wasm target");

    if !status.success() {
        panic!("Failed to compile to wasm32-unknown-unknown target");
    }

    if !out_path.exists() {
        let cwd = std::env::current_dir().unwrap();
        panic!("Expected output wasm file not found at {}, cwd: {}", out_path.display(), cwd.display());
    }
}

criterion_group!(
    benches,
    wasm_bench,
);
criterion_main!(benches);
