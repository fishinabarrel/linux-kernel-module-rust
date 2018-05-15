#![no_std]

#[macro_use]
extern crate linux_kernel_module;

struct HelloWorldModule {}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");
        Ok(HelloWorldModule {})
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("Goodbye kernel module!");
    }
}
kernel_module!(
    HelloWorldModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "An extremely simple kernel module",
    license: "GPL"
);
