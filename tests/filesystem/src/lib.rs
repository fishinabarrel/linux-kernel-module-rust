#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::convert::{TryFrom, TryInto};
use core::ptr;
use linux_kernel_module::bindings;
use linux_kernel_module::filesystem::*;
use linux_kernel_module::println;
use linux_kernel_module::{self, c_types, cstr, CStr};
use linux_kernel_module::{Error, KernelResult};

extern "C" {
    fn PTR_ERR_helper(ptr: *const c_types::c_void) -> c_types::c_long;
    fn IS_ERR_helper(ptr: *const c_types::c_void) -> c_types::c_bool;
}

struct TestfsInfo {
    dummy_data: u32,
}

struct TestfsSuperOperations;

impl SuperOperations for TestfsSuperOperations {
    type I = TestfsInfo;

    const VTABLE: SuperOperationsVtable<Self::I> = SuperOperationsVtable::<Self::I>::new::<Self>();

    fn put_super(sb: &mut SuperBlock<Self::I>) {
        assert!(sb.get_fs_info().unwrap().dummy_data == 0xbadf00d);

        // This returns the old value therefore dropping it if we don't take
        // ownership of it.
        sb.replace_fs_info(None);

        println!("testfs-put_super-marker");
    }
}

const TESTFS_SB_MAGIC: c_types::c_ulong = 0xdeadc0de;

struct Testfs;

impl FileSystem for Testfs {
    type I = TestfsInfo;

    const NAME: &'static CStr = cstr!("testfs");
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;

    fn fill_super(
        sb: &mut SuperBlock<Self::I>,
        _data: *mut c_types::c_void,
        silent: c_types::c_int,
    ) -> KernelResult<()> {
        // The kernel initializes fs_info to NULL.
        assert!(sb.get_fs_info().is_none());

        // Replace NULL with our data. SuperBlock takes ownership of it.
        sb.replace_fs_info(Some(Box::new(TestfsInfo { dummy_data: 42 })));

        // We can obtain references to it while SuperBlock owns it:
        assert!(sb.get_fs_info().unwrap().dummy_data == 42);

        // And also mutable references if we have a mutable reference to the
        // super block:
        let fs_info: &mut TestfsInfo = sb.get_mut_fs_info().unwrap();
        fs_info.dummy_data = 0xbadf00d;

        sb.set_op(&TestfsSuperOperations::VTABLE);
        sb.set_magic(TESTFS_SB_MAGIC);

        // TODO: Use safe API when available.
        unsafe {
            const TESTFS_ROOT_BNO: u64 = 1;
            let root = bindings::new_inode(sb.get_mut());
            if IS_ERR_helper(root as *const c_types::c_void) {
                if silent == 0 {
                    println!("Failed to create testfs root inode.");
                }
                let errno: i32 =
                    i32::try_from(PTR_ERR_helper(root as *const c_types::c_void)).unwrap();
                sb.replace_fs_info(None);
                return Err(Error::from_kernel_errno(errno));
            }

            (*root).i_sb = sb.get_mut();
            (*root).i_ino = TESTFS_ROOT_BNO;

            // The inode passed to d_make_root must have the S_IFDIR flag set,
            // otherwise mount(8) will fail with 'mount(2) system call failed:
            // Not a directory.' for the given mountpoint directory (even
            // thought the _supplied_ mountpoint is a directory).
            bindings::inode_init_owner(root, ptr::null(), bindings::S_IFDIR.try_into().unwrap());

            let now = bindings::current_time(root);
            (*root).i_atime = now;
            (*root).i_mtime = now;
            (*root).i_ctime = now;

            sb.get_mut().s_root = bindings::d_make_root(root);
            if sb.get_mut().s_root.is_null() {
                if silent == 0 {
                    println!("Testfs d_make_root failed.");
                }
                sb.replace_fs_info(None);
                return Err(Error::ENOMEM);
            }
        }

        println!("testfs-fill_super-marker");

        Ok(())
    }
}

struct TestfsModule {
    _fs_registration: Registration<Testfs>,
}

impl linux_kernel_module::KernelModule for TestfsModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let fs_registration = register::<Testfs>()?;
        Ok(TestfsModule {
            _fs_registration: fs_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    TestfsModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing filesystem::register",
    license: "GPL"
);
