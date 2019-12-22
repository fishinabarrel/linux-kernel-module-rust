#![no_std]

use linux_kernel_module::{self, cstr};

struct CycleFile;

impl linux_kernel_module::file_operations::FileOperations for CycleFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .read()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(CycleFile);
    }
}
impl linux_kernel_module::file_operations::Read for CycleFile {
    fn read(
        &self,
        buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
        offset: u64,
    ) -> linux_kernel_module::KernelResult<()> {
        for c in b"123456789"
            .iter()
            .cycle()
            .skip((offset % 9) as _)
            .take(buf.len())
        {
            buf.write(&[*c])?;
        }
        return Ok(());
    }
}

struct SeekFile;

impl linux_kernel_module::file_operations::FileOperations for SeekFile {
    const VTABLE: linux_kernel_module::file_operations::FileOperationsVtable =
        linux_kernel_module::file_operations::FileOperationsVtable::builder::<Self>()
            .seek()
            .build();

    fn open() -> linux_kernel_module::KernelResult<Self> {
        return Ok(SeekFile);
    }
}

impl linux_kernel_module::file_operations::Seek for SeekFile {
    fn seek(
        &self,
        _file: &linux_kernel_module::file_operations::File,
        _offset: linux_kernel_module::file_operations::SeekFrom,
    ) -> linux_kernel_module::KernelResult<u64> {
        return Ok(1234);
    }
}

struct ChrdevTestModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("chrdev-tests"), 0..2)?
                .register_device::<CycleFile>()
                .register_device::<SeekFile>()
                .build()?;
        Ok(ChrdevTestModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing character devices",
    license: "GPL"
);
