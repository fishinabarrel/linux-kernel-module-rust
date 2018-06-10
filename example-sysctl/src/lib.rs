#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

#[macro_use]
extern crate linux_kernel_module;

use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

struct ExampleSysctlModule {
    a: Sysctl<AtomicBool>,
}

impl linux_kernel_module::KernelModule for ExampleSysctlModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let a = Sysctl::register(
            "rust/example\x00",
            "a\x00",
            AtomicBool::new(false),
            Mode::from_int(0o644),
        )?;

        Ok(ExampleSysctlModule { a })
    }
}

impl Drop for ExampleSysctlModule {
    fn drop(&mut self) {
        println!(
            "Current sysctl value: {}",
            self.a.get().load(Ordering::Relaxed)
        );
    }
}
kernel_module!(
    ExampleSysctlModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A kernel module that offers a sysctl",
    license: "GPL"
);
