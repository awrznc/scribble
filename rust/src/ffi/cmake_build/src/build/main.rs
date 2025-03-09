// `target/(debug|release)/build/cmake_build-0000000000000000/out` に出力される

fn main() {
    // cmake
    let cmake = cmake::Config::new("deps")
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .build();
    println!("cargo:rustc-link-search=native={}", cmake.display());
    println!("cargo:rustc-link-lib=static=piyo");

    // bindgen
    let bindgen = bindgen::Builder::default()
        .header("deps/include/piyo.h")
        .generate()
        .unwrap();
    let export_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindgen
        .write_to_file(export_path.join("bindgen.rs"))
        .unwrap()
}
