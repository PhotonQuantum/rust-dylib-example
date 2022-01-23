fn main() {
    println!("cargo:rustc-link-lib=dylib=is_odd_lib");
    println!("cargo:rustc-link-search=native={}", std::env::current_dir().unwrap().display());
}