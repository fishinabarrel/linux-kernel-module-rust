#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module;

struct ChrdevTestModule {
    _dev: linux_kernel_module::chrdev::DeviceNumberRegion,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(ChrdevTestModule {
            _dev: linux_kernel_module::chrdev::DeviceNumberRegion::allocate(
                0..1,
                "chrdev-tests\x00",
            )?,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character devices",
    license: "GPL"
);
