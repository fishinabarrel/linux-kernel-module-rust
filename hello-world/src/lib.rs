#![no_std]
#![feature(alloc, const_str_as_bytes)]

extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;

#[macro_use]
extern crate linux_kernel_module;

struct HelloWorldModule {
    message: String,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");
        Ok(HelloWorldModule {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Goodbye kernel module!");
    }
}

kernel_module!(
    HelloWorldModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "An extremely simple kernel module",
    license: "GPL"
);
