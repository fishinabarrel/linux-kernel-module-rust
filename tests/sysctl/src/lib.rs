#![no_std]
#![feature(const_str_as_bytes)]

use core::sync::atomic::AtomicBool;

#[macro_use]
extern crate linux_kernel_module;

use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

struct SysctlTestModule {
    _sysctl_a: Sysctl<AtomicBool>,
    _sysctl_b: Sysctl<AtomicBool>,
}

impl linux_kernel_module::KernelModule for SysctlTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(SysctlTestModule {
            _sysctl_a: Sysctl::register(
                "rust/sysctl-tests\x00",
                "a\x00",
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
            _sysctl_b: Sysctl::register(
                "rust/sysctl-tests\x00",
                "b\x00",
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
        })
    }
}

kernel_module!(
    SysctlTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing sysctls",
    license: "GPL"
);
