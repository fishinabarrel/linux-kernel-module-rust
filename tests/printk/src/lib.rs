#![no_std]

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
    author: "Fish in a Barrel Contributors",
    description: "A module for testing println!()",
    license: "GPL"
);
