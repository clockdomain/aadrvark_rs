extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let library_dir = "/path/to/your/library/directory";
    println!("cargo:rustc-link-search=native={}", library_dir);

    println!("cargo:rustc-link-lib=dylib=aardvark");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
