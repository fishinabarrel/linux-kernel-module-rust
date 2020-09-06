#![no_std]

struct UtilsTestModule;

#[allow(dead_code)]
const TEST_CSTR: linux_kernel_module::CStr<'static> = linux_kernel_module::cstr!("abc");

impl linux_kernel_module::KernelModule for UtilsTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(UtilsTestModule)
    }
}

linux_kernel_module::kernel_module!(
    UtilsTestModule,
    author: b"Fish in a Barrel Contributors",
    description: b"A module for testing various utilities",
    license: b"GPL"
);
