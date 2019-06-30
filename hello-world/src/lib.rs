#![no_std]
#![feature(const_str_as_bytes)]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module;
use linux_kernel_module::println;

#[derive(Default)]
struct HelloWorldModule {
    message: String,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init(&mut self) -> linux_kernel_module::KernelResult<()> {
        println!("Hello kernel module!");
        self.message = "on the heap!".to_owned();
        Ok(())
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Goodbye kernel module!");
    }
}

linux_kernel_module::kernel_module!(
    HelloWorldModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "An extremely simple kernel module",
    license: "GPL"
);
