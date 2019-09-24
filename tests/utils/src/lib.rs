#![no_std]

use linux_kernel_module;

struct UtilsTestModule;

#[allow(dead_code)]
const TEST_CSTR: &linux_kernel_module::CStr = linux_kernel_module::cstr!("abc");

impl linux_kernel_module::KernelModule for UtilsTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(UtilsTestModule)
    }
}

linux_kernel_module::kernel_module!(
    UtilsTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing various utilities",
    license: "GPL"
);
