#![no_std]

extern crate alloc;

use alloc::string::ToString;
use core::sync::atomic::{AtomicUsize, Ordering};

use linux_kernel_module::{self, cstr};

struct CycleFile;

impl linux_kernel_module::file_operations::FileOperations for CycleFile {
    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(CycleFile)
    }

    const READ: linux_kernel_module::file_operations::ReadFn<Self> = Some(
        |_this: &Self,
         _file: &linux_kernel_module::file_operations::File,
         buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
         offset: u64|
         -> linux_kernel_module::KernelResult<()> {
            for c in b"123456789"
                .iter()
                .cycle()
                .skip((offset % 9) as _)
                .take(buf.len())
            {
                buf.write(&[*c])?;
            }
            Ok(())
        },
    );
}

struct SeekFile;

impl linux_kernel_module::file_operations::FileOperations for SeekFile {
    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(SeekFile)
    }

    const SEEK: linux_kernel_module::file_operations::SeekFn<Self> = Some(
        |_this: &Self,
         _file: &linux_kernel_module::file_operations::File,
         _offset: linux_kernel_module::file_operations::SeekFrom|
         -> linux_kernel_module::KernelResult<u64> { Ok(1234) },
    );
}

struct WriteFile {
    written: AtomicUsize,
}

impl linux_kernel_module::file_operations::FileOperations for WriteFile {
    fn open() -> linux_kernel_module::KernelResult<Self> {
        Ok(WriteFile {
            written: AtomicUsize::new(0),
        })
    }

    const READ: linux_kernel_module::file_operations::ReadFn<Self> = Some(
        |this: &Self,
         _file: &linux_kernel_module::file_operations::File,
         buf: &mut linux_kernel_module::user_ptr::UserSlicePtrWriter,
         _offset: u64|
         -> linux_kernel_module::KernelResult<()> {
            let val = this.written.load(Ordering::SeqCst).to_string();
            buf.write(val.as_bytes())?;
            Ok(())
        },
    );

    const WRITE: linux_kernel_module::file_operations::WriteFn<Self> = Some(
        |this: &Self,
         buf: &mut linux_kernel_module::user_ptr::UserSlicePtrReader,
         _offset: u64|
         -> linux_kernel_module::KernelResult<()> {
            let data = buf.read_all()?;
            this.written.fetch_add(data.len(), Ordering::SeqCst);
            Ok(())
        },
    );
}

struct ChrdevTestModule {
    _chrdev_registration: linux_kernel_module::chrdev::Registration,
}

impl linux_kernel_module::KernelModule for ChrdevTestModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let chrdev_registration =
            linux_kernel_module::chrdev::builder(cstr!("chrdev-tests"), 0..3)?
                .register_device::<CycleFile>()
                .register_device::<SeekFile>()
                .register_device::<WriteFile>()
                .build()?;
        Ok(ChrdevTestModule {
            _chrdev_registration: chrdev_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    ChrdevTestModule,
    author: b"Fish in a Barrel Contributors",
    description: b"A module for testing character devices",
    license: b"GPL"
);
