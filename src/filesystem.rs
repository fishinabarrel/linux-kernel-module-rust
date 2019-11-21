use alloc::boxed::Box;
use core::default::Default;
use core::marker;
use core::ptr;

use bitflags;

use crate::bindings;
use crate::c_types;
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
    type SuperOperations;

    fn fill_super(sb: SuperBlock<Self::SuperBlockInfo>) -> KernelResult<()>;
}

struct SuperBlock<'a, I> {
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
    fn set_fs_info(&mut self, fs_info: Option<Box<I>>) {
        self.ptr.s_fs_info = match fs_info {
            Some(b) => unsafe { Box::into_raw(b) as *mut c_types::c_void },
            None => ptr::null_mut(),
        };
    }

    fn get_fs_info(&self) -> Option<Box<I>> {
        // TODO: Check whether ptr.s_fs_info is initilized to zero or
        // uninitialized by the kernel. If it is the latter use MaybeUninit?
        match self.ptr.s_fs_info.as_mut() {
            Some(nonnull_mutborrow) => Some(unsafe { Box::from_raw(
                nonnull_mutborrow
                    as *mut c_types::c_void
                    as *mut I
            )})
            None => None,
        }
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

fn _fill_super_callback<T: FileSystem>(
    raw_ptr: *mut bindings::super_block
) -> KernelResult<()> {
    // TODO: Panicing in a C callback is UB. Instead wrap this call in a closure
    // with unwind in fill_super_callback<T: FileSystem>. Then return error in
    // panic occurs.
    let ptr = raw_ptr.as_mut().unwrap();
    let sb = SuperBlock {
        ptr: ptr,
        _phantom: marker::PhantomData,
    };

    T::fill_super(sb)
}

extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
    _data: *mut c_types::c_void,
    _silent: c_types::c_int,
) -> c_types::c_int {
    match _fill_super_callback::<T>(sb) {
        Ok(()) => 0,
        Err(e) => e.to_kernel_errno(),
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

pub struct SuperOperationsVtable(bindings::super_operations);

impl SuperOperationsVtable {
    pub fn new<T: SuperOperations>() -> SuperOperationsVtable {
        SuperOperationsVtable(bindings::super_operations {
            put_super: Some(put_super_callback::<T>),
            ..Default::default()
        })
    }
}

unsafe extern "C" fn put_super_callback<T: SuperOperations>(
    _sb: *mut bindings::super_block,
) {
    // TODO: drop fs info?
    unimplemented!();
}

pub trait SuperOperations: Sync + Sized {
    /// A container for the actual `super_operations` value. This will always be:
    /// ```
    /// const VTABLE: linux_kernel_module::filesystem::SuperOperationsVtable =
    ///     linux_kernel_module::filesystem::SuperOperationsVtable::new::<Self>();
    /// ```
    const VTABLE: SuperOperationsVtable;

    // aka Drop?
    fn put_super(&self) -> KernelResult<()> {
        Err(Error::EINVAL)
    }
}
