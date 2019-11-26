#![no_std]

extern crate alloc;

use linux_kernel_module::filesystem::*;
use linux_kernel_module::{self, cstr, CStr, c_types};
use linux_kernel_module::println;
use linux_kernel_module::KernelResult;
use linux_kernel_module::bindings;
use alloc::boxed::Box;

struct RamfsInfo {
    magic: u32,
}

struct RamfsSuperOperations;

impl SuperOperations for RamfsSuperOperations {
    type I = RamfsInfo;
    
    fn put_super(sb: &mut SuperBlock<Self::I>) {
        assert!(sb.fs_info_ref().unwrap().magic == 0xbadf00d);

        // This returns the old value therefore dropping it if we don't take
        // ownership of it. This would normally happen in the put_super
        // callback.
        sb.set_fs_info(None);

        println!("Ramfs put_super executed.");
    }
}

const TESTFS_SUPER_OPERATIONS_VTABLE: SuperOperationsVtable<RamfsInfo> =
    SuperOperationsVtable::<RamfsInfo>::new::<RamfsSuperOperations>();
const TESTFS_SB_MAGIC: c_types::c_ulong = 0xdeadc0de;

struct Ramfs;

impl FileSystem for Ramfs {
    type I = RamfsInfo;

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
        sb.set_fs_info(Some(Box::new(RamfsInfo {
            magic: 42,
        })));

        // We can obtain references to it while SuperBlock owns it:
        assert!(sb.fs_info_ref().unwrap().magic == 42);

        // And also mutable references if we have a mutable reference to the
        // super block:
        let fs_info: &mut RamfsInfo = sb.fs_info_mut().unwrap();
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

        println!("Ramfs fill_super executed.");

        Ok(())
    }
}

struct RamfsModule {
    _fs_registration: Registration<Ramfs>,
}

impl linux_kernel_module::KernelModule for RamfsModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        let fs_registration = register::<Ramfs>()?;
        Ok(RamfsModule {
            _fs_registration: fs_registration,
        })
    }
}

linux_kernel_module::kernel_module!(
    RamfsModule,
    author: "Fish in a Barrel Contributors",
    description: "A module for testing filesystem::register",
    license: "GPL"
);
