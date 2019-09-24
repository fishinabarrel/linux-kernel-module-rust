#![no_std]

use core::sync::atomic::AtomicBool;

use linux_kernel_module::{self, cstr};

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
                cstr!("rust/sysctl-tests"),
                cstr!("a"),
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
            _sysctl_b: Sysctl::register(
                cstr!("rust/sysctl-tests"),
                cstr!("b"),
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
        })
    }
}

linux_kernel_module::kernel_module!(
    SysctlTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing sysctls",
    license: "GPL"
);
