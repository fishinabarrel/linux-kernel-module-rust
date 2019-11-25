#![no_std]
#![feature(const_str_as_bytes)]

extern crate alloc;

use linux_kernel_module::filesystem::{self, FileSystem, FileSystemFlags, SuperBlock};
use linux_kernel_module::{self, cstr, CStr, c_types};
use linux_kernel_module::println;
use linux_kernel_module::KernelResult;
use alloc::boxed::Box;

struct TestFSInfo {
    magic: u32,
}

struct TestFSSuperOperations;

impl SuperOperations<TestFSInfo> for TestFSSuperOperations {
    const VTABLE: SuperOperationsVtable<TestFSInfo> =
        SuperOperationsVtable::new<Self>();
    
    fn put_super(sb: &mut SuperBlock<TestFSInfo>) {
        assert!(sb.fs_info_ref().unwrap().magic == 0xbadf00d);

        // This returns the old value therefore dropping it if we don't take
        // ownership of it. This would normally happen in the put_super
        // callback.
        sb.set_fs_info(None);
    }
}

struct TestFS;

impl FileSystem for TestFS {
    const NAME: &'static CStr = cstr!("testfs");
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;

    // TODO: Enforce setting of sb.s_op. Make him return a InitializedSuperBlock?
    fn fill_super(
        sb: &mut SuperBlock<TestFSInfo>,
        _data: *mut c_types::c_void,
        _silent: c_types::c_int,
    ) -> KernelResult<()> {

        // The kernel initializes fs_info to NULL.
        assert!(sb.fs_info_ref().is_none());

        // Replace NULL with our data. SuperBlock takes ownership of it.
        sb.set_fs_info(Some(Box::new(TestFSInfo {
            magic: 42,
        })));

        // We can obtain references to it while SuperBlock owns it:
        assert!(sb.fs_info_ref().unwrap().magic == 42);

        // And also mutable references if we have a mutable reference to the
        // super block:
        let fs_info: &mut TestFSInfo = sb.fs_info_mut().unwrap();
        fs_info.magic = 0xbadf00d;

        sb.set_op(&TestFSSuperOperations::VTABLE);

        Ok(())
    }
}

struct TestFSModule {
    _fs_registration: filesystem::Registration<TestFS>,
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
