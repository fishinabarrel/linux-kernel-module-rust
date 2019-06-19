#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module;

struct ChrdevRegionAllocationTestModule {
    _dev: linux_kernel_module::chrdev::DeviceNumberRegion,
}

impl linux_kernel_module::KernelModule for ChrdevRegionAllocationTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(ChrdevRegionAllocationTestModule {
            _dev: linux_kernel_module::chrdev::DeviceNumberRegion::allocate(
                0..1,
                "chrdev-region-allocation-tests\x00",
            )?,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevRegionAllocationTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character device region allocation",
    license: "GPL"
);
