#![no_std]
#![allow(clippy::print_literal)]

use linux_kernel_module::{self, println};

struct PrintkTestModule;

impl linux_kernel_module::KernelModule for PrintkTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Single element printk");
        println!();
        println!("printk with {} parameters{}", 2, "!");

        Ok(PrintkTestModule)
    }
}

linux_kernel_module::kernel_module!(
    PrintkTestModule,
    author: b"Fish in a Barrel Contributors",
    description: b"A module for testing println!()",
    license: b"GPL"
);
