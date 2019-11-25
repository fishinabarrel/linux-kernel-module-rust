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

unsafe extern "C" fn put_super_callback<T: SuperOperations<I>>(
    sb_raw: *mut bindings::super_block
) {
    let mut sb = SuperBlock {
        sb: sb_raw.as_mut().unwrap(),
        _phantom_fs_info: marker::PhantomData,
    };
    T::put_super(&mut sb);
}

pub struct SuperOperationsVtable<I> {
    op: bindings::super_operations,
    _phantom_sb_fs_info: marker::PhantomData<I>,
};

impl SuperOperationsVtable {
    pub const fn new<T: SuperOperations<I>>() -> SuperOperationsVtable {
        SuperOperationsVtable(bindings::super_operations {
            put_super: Some(put_super_callback::<T>),

            ..Default::default()
        })
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

    pub fn set_op(&mut self, op: &SuperOperationsVtable<I>) {

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

// TODO: Use generic or associate typ for Info.
pub trait FileSystem: Sync {
    const NAME: &'static CStr;
    const FLAGS: FileSystemFlags;

    fn fill_super(
        sb: &mut SuperBlock<>,
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
