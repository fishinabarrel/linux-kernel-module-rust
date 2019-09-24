#![no_std]

use linux_kernel_module::{self, cstr};

struct ChrdevRegionAllocationTestModule {
    _chrdev_reg: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevRegionAllocationTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_reg =
            linux_kernel_module::chrdev::builder(cstr!("chrdev-region-allocation-tests"), 0..1)?
                .build()?;

        Ok(ChrdevRegionAllocationTestModule {
            _chrdev_reg: chrdev_reg,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevRegionAllocationTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing character device region allocation",
    license: "GPL"
);
