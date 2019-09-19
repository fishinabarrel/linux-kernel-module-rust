#![no_std]
#![feature(const_str_as_bytes)]
#![feature(const_raw_ptr_deref)]

extern crate alloc;

use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags};
use linux_kernel_module::{self, CStr};

struct TestFSModule {
    _fs_registration: filesystem::Registration<TestFS>,
}

struct TestFS {}

impl FileSystem for TestFS {
    const NAME: &'static CStr = unsafe { &*("testfs\x00" as *const str as *const CStr) };
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;
}

impl linux_kernel_module::KernelModule for TestFSModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let fs_registration = filesystem::register::<TestFS>()?;
        Ok(TestFSModule {
            _fs_registration: fs_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    TestFSModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing filesystem::register",
    license: "GPL"
);
