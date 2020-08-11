#![no_std]

struct ModinfoTestModule;

impl linux_kernel_module::KernelModule for ModinfoTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(ModinfoTestModule)
    }
}

linux_kernel_module::kernel_module!(
    ModinfoTestModule,
    author: b"Fish in a Barrel Contributors",
    description: b"Empty module for testing modinfo",
    license: b"GPL"
);
