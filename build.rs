extern crate bindgen;

use std::env;
use std::path::PathBuf;

const INCLUDED_TYPES: &[&str] = &[];
const INCLUDED_FUNCTIONS: &[&str] = &[];
const INCLUDED_VARS: &[&str] = &[];

fn main() {
    let mut builder = bindgen::Builder::default()
        // TODO: what header for linux!
        .header("XXX!");

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.whitelist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
