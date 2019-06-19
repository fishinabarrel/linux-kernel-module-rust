#![no_std]
#![feature(const_str_as_bytes)]

use linux_kernel_module;

struct CycleFile;

impl linux_kernel_module::chrdev::FileOperations for CycleFile {
    const VTABLE: linux_kernel_module::chrdev::FileOperationsVtable =
        linux_kernel_module::chrdev::FileOperationsVtable::new::<Self>();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(CycleFile);
    }

    fn read(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
    ) -> linux_kernel_module::KernelResult<()> {
        for c in b"123456789".iter().cycle().take(buf.len()) {
            buf.write(&[*c])?;
        }
        return Ok(());
    }
}

struct ChrdevTestModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration = linux_kernel_module::chrdev::builder("chrdev-tests\x00", 0..1)?
            .register_device::<CycleFile>()
            .build()?;
        Ok(ChrdevTestModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "A module for testing character devices",
    license: "GPL"
);
