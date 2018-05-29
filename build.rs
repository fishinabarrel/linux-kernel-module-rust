extern crate bindgen;
extern crate cc;
extern crate shlex;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .no_copy(".*")
        .derive_default(true)
        .rustfmt_bindings(true);

    let output = String::from_utf8(
        Command::new("make")
            .arg("-C")
            .arg("kernel-cflags-finder")
            .arg("-s")
            .output()
            .unwrap()
            .stdout,
    ).unwrap();

    for arg in shlex::split(&output).unwrap() {
        builder = builder.clang_arg(arg.to_string());
    }

    println!("cargo:rerun-if-changed=src/bindings_helper.h");
    builder = builder.header("src/bindings_helper.h");
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    println!("cargo:rerun-if-env-changed=CLANG");
    builder.compiler(env::var("CLANG").unwrap_or("clang".to_string()));
    builder.file("src/helpers.c");
    for arg in shlex::split(&output).unwrap() {
        builder.flag(&arg);
    }
    builder.compile("helpers");
}
