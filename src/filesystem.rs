use alloc::boxed::Box;
use core::default::Default;
use core::marker;
use core::ptr;

use bitflags;

use crate::bindings;
use crate::c_types::{c_void, c_int, c_char};
use crate::error;
use crate::types::CStr;
use crate::error::{KernelResult};

pub trait SuperOperations<I>: Sync + Sized {
    fn put_super(sb: &mut SuperBlock<I>);
}

unsafe extern "C" fn put_super_callback<I, T: SuperOperations<I>>(
    sb_raw: *mut bindings::super_block
) {
    let mut sb = SuperBlock {
        sb: sb_raw.as_mut().unwrap(),
        _phantom_fs_info: marker::PhantomData,
    };
    T::put_super(&mut sb);
}

unsafe extern "C" fn dirty_inode_callback<I, T: SuperOperations<I>>(
    inode: *mut bindings::inode,
    flags: c_int,
) {
    unimplemented!();
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
    pub const fn new<J, T: SuperOperations<J>>() -> SuperOperationsVtable<J> {
        SuperOperationsVtable {
            op: bindings::super_operations {
                alloc_inode: None,
                // destroy_inode is only required if alloc_inode was defined.
                destroy_inode: None,
                real_loop: None, // TODO: What's that?

		        dirty_inode: Some(dirty_inode_callback::<J, T>),
		        write_inode: Some(write_inode_callback::<J, T>),
		        drop_inode: None,
		        evict_inode: Some(evict_inode_callback::<J, T>),
                put_super: Some(put_super_callback::<J, T>),
		        sync_fs: None,
                freeze_super: Some(freeze_super_callback::<J, T>),
		        freeze_fs: Some(freeze_fs_callback::<J, T>),
                thaw_super: Some(thaw_super_callback::<J, T>),
		        unfreeze_fs: Some(unfreeze_fs_callback::<J, T>),
		        statfs: Some(statfs_callback::<J, T>),
		        remount_fs: Some(remount_fs_callback::<J, T>),
		        umount_begin: Some(umount_begin_callback::<J, T>),

		        show_options: Some(show_options_callback::<J, T>),
                show_devname: Some(show_devname_callback::<J, T>),
	            show_path: Some(show_path_callback::<J, T>),
	            show_stats: Some(show_stats_callback::<J, T>),
                // TODO #ifdef CONFIG_QUOTA
		        quota_read: Some(quota_read_callback::<J, T>),
		        quota_write: Some(quota_write_callback::<J, T>),
                get_dquots: Some(get_dquots_callback::<J, T>),
                // TODO #endif
                bdev_try_to_free_page: Some(bdev_try_to_free_page_callback::<J, T>),
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
    sb: &'a mut bindings::super_block,
    _phantom_fs_info: marker::PhantomData<Option<Box<I>>>,
}

impl<I> SuperBlock<'_, I> {

    // Ideally we should only require fs_info to be something than can be
    // converted to a raw pointer and back again safely (if we don't mess with
    // the value while we keep it). I don't think there exists such a
    // trait. Therefore just require that fs_info is on the heap (i.e. a Box).
    pub fn set_fs_info(&mut self, new_val: Option<Box<I>>) -> Option<Box<I>> {
        let old_val = ptr::NonNull::new(self.sb.s_fs_info as *mut I).map(
            |nn| unsafe { Box::from_raw(nn.as_ptr()) }
        );
        self.sb.s_fs_info = match new_val {
            None => ptr::null() as *const c_void as *mut c_void,
            Some(b) => Box::into_raw(b) as *mut c_void,
        };
        old_val
    }

    pub fn fs_info_ref(&self) -> Option<&I> {
        unsafe { (self.sb.s_fs_info as *mut I).as_ref()}
    }

    pub fn fs_info_mut(&mut self) -> Option<&mut I> {
        unsafe { (self.sb.s_fs_info as *mut I).as_mut() }
    }

    pub fn set_op(&mut self, op: &'static SuperOperationsVtable<I>) {
        self.sb.s_op = &op.op;
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
    const FLAGS: FileSystemFlags;

    fn fill_super(
        sb: &mut SuperBlock<Self::I>,
        data: *mut c_void,
        silent: c_int,
    ) -> KernelResult<()>;
}

fn _fill_super_callback<T: FileSystem>(
    sb_raw: *mut bindings::super_block,
    data: *mut c_void,
    silent: c_int,
) -> KernelResult<()> {
    let mut sb = SuperBlock {
        sb: unsafe { sb_raw.as_mut() }.unwrap(),
        _phantom_fs_info: marker::PhantomData,
    };
    T::fill_super(&mut sb, data, silent)
}

extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
    data: *mut c_void,
    silent: c_int,
) -> c_int {
    match _fill_super_callback::<T>(sb, data, silent) {
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

extern "C" fn kill_sb_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
) {
    unsafe { bindings::kill_litter_super(sb) }
}

extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut bindings::file_system_type,
    flags: c_int,
    dev_name: *const c_char,
    data: *mut c_void,
) -> *mut bindings::dentry {
    unsafe { bindings::mount_bdev(fs_type, flags, dev_name, data, Some(fill_super_callback::<T>)) }
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
