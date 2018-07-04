#![no_std]

#[macro_use]
extern crate linux_kernel_module;

struct ChrdevTestModule {
    _dev: linux_kernel_module::chrdev::DeviceNumberRegion,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(ChrdevTestModule {
            _dev: linux_kernel_module::chrdev::DeviceNumberRegion::allocate(
                1,
                0,
                "chrdev-tests\x00",
            )?,
        })
    }
}

kernel_module!(
    ChrdevTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character devices",
    license: "GPL"
);
