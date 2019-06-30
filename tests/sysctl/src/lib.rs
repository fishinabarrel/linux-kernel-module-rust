#![no_std]
#![feature(const_str_as_bytes)]

use core::sync::atomic::AtomicBool;

use linux_kernel_module;
use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

#[derive(Default)]
struct SysctlTestModule {
    _sysctl_a: Option<Sysctl<AtomicBool>>,
    _sysctl_b: Option<Sysctl<AtomicBool>>,
}

impl linux_kernel_module::KernelModule for SysctlTestModule {
    fn init(&mut self) -> linux_kernel_module::KernelResult<()> {
        self._sysctl_a = Some(Sysctl::register(
            "rust/sysctl-tests\x00",
            "a\x00",
            AtomicBool::new(false),
            Mode::from_int(0o666),
        )?);
        self._sysctl_b = Some(Sysctl::register(
            "rust/sysctl-tests\x00",
            "b\x00",
            AtomicBool::new(false),
            Mode::from_int(0o666),
        )?);

        return Ok(());
    }
}

linux_kernel_module::kernel_module!(
    SysctlTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing sysctls",
    license: "GPL"
);
