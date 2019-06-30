#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module;

#[derive(Default)]
struct ChrdevRegionAllocationTestModule {
    _dev: Option<linux_kernel_module::chrdev::DeviceNumberRegion>,
}

impl linux_kernel_module::KernelModule for ChrdevRegionAllocationTestModule {
    fn init(&mut self) -> linux_kernel_module::KernelResult<()> {
        self._dev = Some(linux_kernel_module::chrdev::DeviceNumberRegion::allocate(
            0..1,
            "chrdev-region-allocation-tests\x00",
        )?);
        Ok(())
    }
}

linux_kernel_module::kernel_module!(
    ChrdevRegionAllocationTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character device region allocation",
    license: "GPL"
);
