#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module;

struct ChrdevRegionAllocationTestModule {
    _chrdev_reg: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevRegionAllocationTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_reg =
            linux_kernel_module::chrdev::builder("chrdev-region-allocation-tests\x00", 0..1)?
                .build()?;

        Ok(ChrdevRegionAllocationTestModule {
            _chrdev_reg: chrdev_reg,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevRegionAllocationTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character device region allocation",
    license: "GPL"
);
