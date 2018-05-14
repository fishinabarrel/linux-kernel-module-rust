extern crate bindgen;
extern crate nix;
extern crate shlex;

use std::env;
use std::path::PathBuf;
use std::process::Command;

const HEADERS: &[&str] = &["linux/fs.h"];
const INCLUDED_TYPES: &[&str] = &["file_system_type"];
const INCLUDED_FUNCTIONS: &[&str] = &["register_filesystem", "unregister_filesystem"];
const INCLUDED_VARS: &[&str] = &[];

fn main() {
    let kernel = nix::sys::utsname::uname();
    let mut builder = bindgen::Builder::default();

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

    for h in HEADERS {
        builder = builder.header(format!(
            "/lib/modules/{}/build/include/{}",
            kernel.release(),
            h.to_string()
        ));
    }

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.whitelist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
