#![no_std]

#[macro_use]
extern crate linux_kernel_module;

struct StaticFileSystem;

impl linux_kernel_module::KernelModule for StaticFileSystem {
    fn init() -> Result<Self, linux_kernel_module::Error> {
        Ok(StaticFileSystem)
    }

    fn exit(&mut self) {

    }
}

kernel_module!(StaticFileSystem, author: "Alex Gaynor and Geoffrey Thomas", description: "An example Rust kernel module that implements a static file system", license: "GPL");
