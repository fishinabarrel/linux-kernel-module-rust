#![no_std]
#![feature(const_str_as_bytes)]

extern crate alloc;

use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags};
use linux_kernel_module::{self, cstr, CStr};

struct TestFSModule {
    _fs_registration: filesystem::Registration<TestFS>,
}

struct TestFS {}

struct TestFSInfo {
    magic: 0x5ac6e888,
}

impl FileSystem for TestFS {
    const NAME: &'static CStr = cstr!("testfs");
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;

    type SuperBlockInfo = TestFSInfo;

    fn fill_super(
        sb: &mut SuperBlock<Self::SuperBlockInfo>,
        data: *mut c_types::c_void,
        silent: c_types::c_int,
    ) -> KernelResult<()> {
        assert!(sb.get_fs_info() == None);
        sb.set_fs_info(Some(Box::new(TestFSInfo {})));
        assert!(sb.get_fs_info().unwrap());
    }
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
