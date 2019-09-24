#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use linux_kernel_module::{self, cstr, println};

use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

static A_VAL: AtomicBool = AtomicBool::new(false);

struct SysctlGetTestModule {
    sysctl_a: Sysctl<&'static AtomicBool>,
}

impl linux_kernel_module::KernelModule for SysctlGetTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(SysctlGetTestModule {
            sysctl_a: Sysctl::register(
                cstr!("rust/sysctl-get-tests"),
                cstr!("a"),
                &A_VAL,
                Mode::from_int(0o666),
            )?,
        })
    }
}

impl Drop for SysctlGetTestModule {
    fn drop(&mut self) {
        println!("A_VAL: {:?}", A_VAL.load(Ordering::Relaxed));
        println!(
            "SYSCTL_A: {:?}",
            self.sysctl_a.get().load(Ordering::Relaxed)
        );
    }
}

linux_kernel_module::kernel_module!(
    SysctlGetTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing sysctls",
    license: "GPL"
);
