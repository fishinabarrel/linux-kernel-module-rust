#![no_std]
#![feature(const_str_as_bytes)]

extern crate alloc;

use linux_kernel_module::filesystem::*;
use linux_kernel_module::{self, cstr, CStr, c_types};
use linux_kernel_module::println;
use linux_kernel_module::KernelResult;
use linux_kernel_module::bindings;
use alloc::boxed::Box;
use core::ptr;
use core::convert::TryInto;

struct TestfsInfo {
    magic: u32,
}

struct TestfsSuperOperations;

impl SuperOperations for TestfsSuperOperations {
    type I = TestfsInfo;
    
    fn put_super(sb: &mut SuperBlock<Self::I>) {
        assert!(sb.fs_info_ref().unwrap().magic == 0xbadf00d);

        // This returns the old value therefore dropping it if we don't take
        // ownership of it. This would normally happen in the put_super
        // callback.
        sb.set_fs_info(None);

        println!("Testfs put_super executed.");
    }
}

const TESTFS_SUPER_OPERATIONS_VTABLE: SuperOperationsVtable<TestfsInfo> =
    SuperOperationsVtable::<TestfsInfo>::new::<TestfsSuperOperations>();
const TESTFS_SB_MAGIC: c_types::c_ulong = 0xdeadc0de;

struct Testfs;

impl FileSystem for Testfs {
    type I = TestfsInfo;

    const NAME: &'static CStr = cstr!("testfs");
    const FLAGS: FileSystemFlags = FileSystemFlags::FS_REQUIRES_DEV;

    // TODO: Enforce setting of sb.s_op. Make him return a InitializedSuperBlock?
    fn fill_super(
        sb: &mut SuperBlock<Self::I>,
        _data: *mut c_types::c_void,
        _silent: c_types::c_int,
    ) -> KernelResult<()> {

        // The kernel initializes fs_info to NULL.
        assert!(sb.fs_info_ref().is_none());

        // Replace NULL with our data. SuperBlock takes ownership of it.
        sb.set_fs_info(Some(Box::new(TestfsInfo {
            magic: 42,
        })));

        // We can obtain references to it while SuperBlock owns it:
        assert!(sb.fs_info_ref().unwrap().magic == 42);

        // And also mutable references if we have a mutable reference to the
        // super block:
        let fs_info: &mut TestfsInfo = sb.fs_info_mut().unwrap();
        fs_info.magic = 0xbadf00d;

        sb.set_op(&TESTFS_SUPER_OPERATIONS_VTABLE);
        sb.set_magic(TESTFS_SB_MAGIC);

        // TODO: Use safe API when available.
        unsafe {
            const TESTFS_ROOT_BNO: u64 = 1;
            let root = bindings::new_inode(sb.ptr);
	        // TODO:
            // if (IS_ERR(root)) {
		    //     if (!silent)
			//         pr_err("Root getting failed.\n");
		    //     err = PTR_ERR(root);
		    //     goto release_sbi;
	        // }

            (*root).i_sb = sb.ptr;
            (*root).i_ino = TESTFS_ROOT_BNO;
            bindings::inode_init_owner(root, ptr::null(),
                                       bindings::S_IFDIR.try_into().unwrap());
            // TODO: Handle error?
            let now = bindings::current_time(root);
            (*root).i_atime = now;
            (*root).i_mtime = now;
            (*root).i_ctime = now;

	        sb.ptr.s_root = bindings::d_make_root(root);
	        // TODO:
            // if (!sb->s_root) {
		    //     if (!silent)
			//         pr_err("Root creation failed.\n");
		    //     err = -ENOMEM;
		    //     goto release_root;
	        // }
            // release_root:
	        // if (err) {
		    //     destroy_root_inode(root);
	        // }
        }

        println!("Testfs fill_super executed.");

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
