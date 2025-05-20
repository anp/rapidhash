fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = std::path::Path::new(&path).join("cpp");
    // println!("cargo:warning=CPP_PATH={}", dir.display());

    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // println!("cargo:warning=OUT_DIR={}", out_dir);

    // let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    // println!("cargo:warning=LATEST_TS={}", now.as_secs_f64());

    // Check if the directory exists
    if !dir.exists() {
        panic!("Failed to properly resolve cpp/ dir: {}", dir.display());
    }

    for i in ["1", "2", "2_1", "3"] {
        let header = format!("rapidhash_v{i}.hpp");
        let wrapper = format!("rapidhash_v{i}_wrapper.cpp");
        let library = format!("rapidhash_v{i}");

        cc::Build::new()
            .cpp(true)
            .file(dir.join(&wrapper))
            .include(&dir)
            .std("c++20")
            .opt_level(3)
            .flag_if_supported("-march=native")
            .compile(&library);

        println!("cargo:rerun-if-changed={}/{header}", dir.display());
        println!("cargo:rerun-if-changed={}/{wrapper}", dir.display());
    }
}
