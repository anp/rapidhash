fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/rapidhash_v1_wrapper.cpp")
        .include("cpp")
        .flag_if_supported("-std=c++20")
        .flag("-O3")
        .flag("-march=native")
        .flag("-flto")
        .warnings(false)
        .compile("rapidhash_v1");

    cc::Build::new()
        .cpp(true)
        .file("cpp/rapidhash_v2_wrapper.cpp")
        .include("cpp")
        .flag_if_supported("-std=c++20")
        .flag("-O3")
        .flag("-march=native")
        .flag("-flto")
        .warnings(false)
        .compile("rapidhash_v2");

    println!("cargo:rerun-if-changed=cpp/rapidhash_v1.h");
    println!("cargo:rerun-if-changed=cpp/rapidhash_v1_wrapper.cpp");
    println!("cargo:rerun-if-changed=cpp/rapidhash_v2.h");
    println!("cargo:rerun-if-changed=cpp/rapidhash_v2_wrapper.cpp");
}
