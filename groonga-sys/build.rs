use std::env;
use std::path::PathBuf;

fn main() {
    let libgroonga = pkg_config::probe_library("groonga").unwrap();
    let include_path = libgroonga.include_paths.first().unwrap();

    println!("cargo:rerun-if-changed=src/macro.h");
    println!("cargo:rerun-if-changed=src/macro.c");

    cc::Build::new()
        .file("src/macro.c")
        .include(include_path)
        .compile("macro");

    let bindings = bindgen::Builder::default()
        .header(include_path.join("groonga.h").to_str().unwrap())
        .header("src/macro.h")
        .clang_arg(format!("-I{}", include_path.display()))
        .rustified_enum(".*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unbale to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
