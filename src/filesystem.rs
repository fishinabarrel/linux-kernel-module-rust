use alloc::boxed::Box;
use core::default::Default;
use core::marker;
use core::ptr;

use bitflags;

use crate::bindings;
use crate::c_types::NonZeroCInt;
use crate::c_types::{c_char, c_int, c_ulong, c_void};
use crate::error;
use crate::error::KernelResult;
use crate::types::CStr;

pub trait SuperOperations: Sync + Sized {
    type I;

    /// A container for the actual `super_operations` value. This will always be:
    /// ```
    /// const VTABLE: linux_kernel_module::filesystem::SuperOperationsVtable<Self::I> =
    ///     linux_kernel_module::filesystem::SuperOperationsVtable::<Self::I>::new::<Self>();
    /// ```
    const VTABLE: SuperOperationsVtable<Self::I>;

    fn put_super(ptr: &mut SuperBlock<Self::I>);
    // TODO: How can we cause SuperOperationsVtable::new to insert a None for a
    // optional method (thereby causing the kernel to choose some default
    // implementation at runtime) when we don't want to define it?
}

unsafe extern "C" fn put_super_callback<T: SuperOperations>(sb_raw: *mut bindings::super_block) {
    let mut ptr = SuperBlock {
        ptr: sb_raw.as_mut().unwrap(),
        _phantom_fs_info: marker::PhantomData,
    };
    T::put_super(&mut ptr);
}

pub struct SuperOperationsVtable<I> {
    op: bindings::super_operations,
    // TODO: If we allow dropping of vtables we should include the whole
    // virtually owned type here (e.g. functions that takes a super block
    // containing I as argument) to communicate to the compiler that this
    // type will be dropped if the vtable is dropped. For now only include I to
    // silence the unused parameter warning.
    _phantom_sb_fs_info: marker::PhantomData<I>,
}

impl<I> SuperOperationsVtable<I> {
    pub const fn new<T: SuperOperations>() -> Self {
        SuperOperationsVtable {
            op: bindings::super_operations {
                alloc_inode: None,
                // destroy_inode is only required if alloc_inode is defined.
                destroy_inode: None,
                real_loop: None,

                dirty_inode: None,
                write_inode: None,
                drop_inode: None,
                evict_inode: None,
                put_super: Some(put_super_callback::<T>),
                sync_fs: None,
                freeze_super: None,
                freeze_fs: None,
                thaw_super: None,
                unfreeze_fs: None,
                statfs: None,
                remount_fs: None,
                umount_begin: None,

                show_options: None,
                show_devname: None,
                show_path: None,
                show_stats: None,
                // TODO: #ifdef CONFIG_QUOTA
                quota_read: None,
                quota_write: None,
                get_dquots: None,
                // TODO: #endif
                bdev_try_to_free_page: None,
                nr_cached_objects: None,
                // free_cached_objects is optional, but any filesystem
                // implementing this method needs to
                // also implement nr_cached_objects for it to be called
                // correctly.
                free_cached_objects: None,
            },
            _phantom_sb_fs_info: marker::PhantomData,
        }
    }
}

pub struct SuperBlock<'a, I> {
    pub ptr: &'a mut bindings::super_block,
    _phantom_fs_info: marker::PhantomData<Option<Box<I>>>,
}

impl<I> SuperBlock<'_, I> {
    // Ideally we should only require fs_info to be something than can be
    // converted to a raw pointer and back again safely (if we don't mess with
    // the value while we keep it). I don't think there exists such a
    // trait. Therefore just require that fs_info is on the heap (i.e. a Box).
    pub fn set_fs_info(&mut self, new_val: Option<Box<I>>) -> Option<Box<I>> {
        let old_val = ptr::NonNull::new(self.ptr.s_fs_info as *mut I)
            .map(|nn| unsafe { Box::from_raw(nn.as_ptr()) });
        self.ptr.s_fs_info = match new_val {
            None => ptr::null() as *const c_void as *mut c_void,
            Some(b) => Box::into_raw(b) as *mut c_void,
        };
        old_val
    }

    pub fn fs_info_ref(&self) -> Option<&I> {
        unsafe { (self.ptr.s_fs_info as *mut I).as_ref() }
    }

    pub fn fs_info_mut(&mut self) -> Option<&mut I> {
        unsafe { (self.ptr.s_fs_info as *mut I).as_mut() }
    }

    pub fn set_op(&mut self, op: &'static SuperOperationsVtable<I>) {
        self.ptr.s_op = &op.op;
    }

    pub fn set_magic(&mut self, magic: c_ulong) {
        self.ptr.s_magic = magic;
    }

    /// Size must be a power of two, between 512 and PAGE_SIZE, and cannot be
    /// smaller than the size supported by the device.
    pub fn set_blocksize(&mut self, size: c_int) -> Result<NonZeroCInt, ()> {
        let success = unsafe { bindings::sb_set_blocksize(self.ptr, size) };
        NonZeroCInt::new(success).ok_or(())
    }
}

pub struct Registration<T: FileSystem> {
    _phantom: marker::PhantomData<T>,
    ptr: Box<bindings::file_system_type>,
}

// This is safe because Registration doesn't actually expose any methods.
unsafe impl<T> Sync for Registration<T> where T: FileSystem {}

impl<T: FileSystem> Drop for Registration<T> {
    fn drop(&mut self) {
        unsafe { bindings::unregister_filesystem(&mut *self.ptr) };
    }
}

pub trait FileSystem: Sync {
    type I;

    const NAME: &'static CStr;

    // TODO from linux/Documentation/filesystems/porting.rst:
    //
    // new file_system_type method - kill_sb(superblock).  If you are converting
    // an existing filesystem, set it according to ->fs_flags::
    //
    // FS_REQUIRES_DEV		-	kill_block_super
    // FS_LITTER		-	kill_litter_super
    // neither			-	kill_anon_super
    const FLAGS: FileSystemFlags;

    fn fill_super(
        ptr: &mut SuperBlock<Self::I>,
        data: *mut c_void,
        silent: c_int,
    ) -> KernelResult<()>;
}

fn _fill_super_callback<T: FileSystem>(
    sb_raw: *mut bindings::super_block,
    data: *mut c_void,
    silent: c_int,
) -> KernelResult<()> {
    let mut ptr = SuperBlock {
        ptr: unsafe { sb_raw.as_mut() }.unwrap(),
        _phantom_fs_info: marker::PhantomData,
    };
    T::fill_super(&mut ptr, data, silent)
}

extern "C" fn fill_super_callback<T: FileSystem>(
    ptr: *mut bindings::super_block,
    data: *mut c_void,
    silent: c_int,
) -> c_int {
    match _fill_super_callback::<T>(ptr, data, silent) {
        Ok(()) => 0,
        Err(e) => e.to_kernel_errno(),
    }
}

bitflags::bitflags! {
    pub struct FileSystemFlags: c_int {
        const FS_REQUIRES_DEV = bindings::FS_REQUIRES_DEV as c_int;
        const FS_BINARY_MOUNTDATA = bindings::FS_BINARY_MOUNTDATA as c_int;
        const FS_HAS_SUBTYPE = bindings::FS_HAS_SUBTYPE as c_int;
        const FS_USERNS_MOUNT = bindings::FS_USERNS_MOUNT as c_int;
        const FS_RENAME_DOES_D_MOVE = bindings::FS_RENAME_DOES_D_MOVE as c_int;
    }
}

extern "C" fn kill_sb_callback<T: FileSystem>(ptr: *mut bindings::super_block) {
    unsafe { bindings::kill_block_super(ptr) }
}

extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut bindings::file_system_type,
    flags: c_int,
    dev_name: *const c_char,
    data: *mut c_void,
) -> *mut bindings::dentry {
    unsafe {
        bindings::mount_bdev(
            fs_type,
            flags,
            dev_name,
            data,
            Some(fill_super_callback::<T>),
        )
    }
}

pub fn register<T: FileSystem>() -> error::KernelResult<Registration<T>> {
    let mut fs_registration = Registration {
        ptr: Box::new(bindings::file_system_type {
            name: T::NAME.as_ptr() as *const i8,
            owner: unsafe { &mut bindings::__this_module },
            fs_flags: T::FLAGS.bits(),
            mount: Some(mount_callback::<T>),
            kill_sb: Some(kill_sb_callback::<T>),

            ..Default::default()
        }),
        _phantom: marker::PhantomData,
    };
    let result = unsafe { bindings::register_filesystem(&mut *fs_registration.ptr) };
    if result != 0 {
        return Err(error::Error::from_kernel_errno(result));
    }

    Ok(fs_registration)
}
