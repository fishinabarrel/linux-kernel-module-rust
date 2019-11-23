use alloc::boxed::Box;
use core::default::Default;
use core::marker;
use core::ptr;

use bitflags;

use crate::bindings;
use crate::c_types::{self, c_void};
use crate::error;
use crate::types::CStr;
use crate::error::{Error, KernelResult};

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
    const NAME: &'static CStr;
    const FLAGS: FileSystemFlags;

    type SuperBlockInfo;

    fn fill_super(
        sb: &mut SuperBlock<Self::SuperBlockInfo>,
        data: *mut c_types::c_void,
        silent: c_types::c_int,
    ) -> KernelResult<()>;
}

fn _fill_super_callback<T: FileSystem>(
    ptr: *mut bindings::super_block,
    data: *mut c_types::c_void,
    silent: c_types::c_int,
) -> KernelResult<()> {
    let ptr = unsafe { ptr.as_mut() }.unwrap();
    let mut sb = SuperBlock {
        ptr: ptr,
        _phantom: marker::PhantomData,
    };
    T::fill_super(&mut sb, data, silent)
}

extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
    data: *mut c_types::c_void,
    silent: c_types::c_int,
) -> c_types::c_int {
    match _fill_super_callback::<T>(sb, data, silent) {
        Ok(()) => 0,
        Err(e) => e.to_kernel_errno(),
    }
}

pub struct SuperBlock<'a, I> {
    _phantom: marker::PhantomData<I>,
    ptr: &'a mut bindings::super_block,
}

impl<'a, I> SuperBlock<'a, I> {

    // Ideally we should only require fs_info to be something than can be
    // converted to a raw pointer and back again safely (if we don't mess with
    // the value while we keep it). I don't think there exists such a trait
    // (maybe it is BorrowMut in the former case but what's the
    // reverse?). Therefore just require that fs_info is on the heap (i.e. a
    // Box).
    //
    // Assumptions:
    // - The interface should not allow the creation of multiple owned boxes.
    // - The superblock is refcounted by the kernel, it is never
    //   NULL and never an invalid pointer.
    // - It is not safe to mutate sb.s_fs_info outside of put/fill_super.
    // - When fs_info is deallocated, sb.s_fs_info should be set to NULL.

    // To be called in fill_super.
    pub fn into_fs_info(&mut self, val: Box<I>) -> StoredInfoHandle {
        assert!(self.ptr.s_fs_info.is_null());
        self.ptr.s_fs_info = Box::into_raw(val) as *mut c_types::c_void;
    }

    // We still need a way to obtain refs to fs_info in the callbacks between
    // put/fill_super. These refs must become invalid when someone calls
    // from_fs_info and drops the box. They should take '&self' so they are
    // useable in intermediate callbacks.
    //
    // A (mut) ref obtained by these two must not outlive the Box<I> obtained
    // using from_fs_info.
    pub fn fs_info_as_ref(&self, _h: &'b StoredInfoHandle) -> &'b I {
        let ptr = self.ptr.s_fs_info;
        assert!(!ptr.is_null());
        unsafe { & *(ptr as *mut I) }
    }

    // It should be possible to mutate I even if SuperBlock is not mutable
    // (i.e. between fill/put_super).
    pub fn fs_info_as_mut(&self, _h: &'b mut StoredInfoHandle) -> &'b mut I {
        let ptr = self.ptr.s_fs_info;
        assert!(!ptr.is_null());
        unsafe { &mut *(ptr as *mut I) }
    }

    // To be called in put_super.
    //
    // TODO: Maybe create some wrapper object, e.g. StoredSBI (with the lifetime 'i) from which
    // re can retrieve references with lifetime 'i. Getting the box consumes the
    // wrapper object and therefor invalidates all references created by it.
    pub fn from_fs_info(&mut self, _h: StoredInfoHandle) -> Box<I> {
        let ptr = self.ptr.s_fs_info;
        assert!(!ptr.is_null());
        self.ptr.s_fs_info = ptr::null() as *const c_void as *mut c_void;
        unsafe { Box::from_raw(ptr as *mut I) }
    }

}

bitflags::bitflags! {
    pub struct FileSystemFlags: c_types::c_int {
        const FS_REQUIRES_DEV = bindings::FS_REQUIRES_DEV as c_types::c_int;
        const FS_BINARY_MOUNTDATA = bindings::FS_BINARY_MOUNTDATA as c_types::c_int;
        const FS_HAS_SUBTYPE = bindings::FS_HAS_SUBTYPE as c_types::c_int;
        const FS_USERNS_MOUNT = bindings::FS_USERNS_MOUNT as c_types::c_int;
        const FS_RENAME_DOES_D_MOVE = bindings::FS_RENAME_DOES_D_MOVE as c_types::c_int;
    }
}

extern "C" fn kill_sb_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
) {
    unsafe { bindings::kill_litter_super(sb) }
}

extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut bindings::file_system_type,
    flags: c_types::c_int,
    _dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut bindings::dentry {
    unsafe { bindings::mount_nodev(fs_type, flags, data, Some(fill_super_callback::<T>)) }
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
