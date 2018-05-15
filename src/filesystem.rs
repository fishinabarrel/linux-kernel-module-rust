use core::marker;
use core::mem;

use bindings;
use error;
use types;

pub struct FileSystemRegistration<T: FileSystem> {
    _phantom: marker::PhantomData<T>,
    ptr: bindings::file_system_type,
}

impl<T: FileSystem> Drop for FileSystemRegistration<T> {
    fn drop(&mut self) {
        unsafe { bindings::unregister_filesystem(&mut self.ptr) };
        self.ptr = unsafe { mem::zeroed() };
    }
}

pub trait FileSystem {
    const NAME: &'static str;
    const FLAGS: FileSystemFlags;
}

bitflags! {
    struct FileSystemFlags: types::c_int {
        const FS_REQUIRES_DEV = bindings::FS_REQUIRES_DEV;
        const FS_BINARY_MOUNTDATA = bindings::FS_BINARY_MOUNTDATA;
        const FS_HAS_SUBTYPE = bindings::FS_HAS_SUBTYPE;
        const FS_USERNS_MOUNT = bindings::FS_USERNS_MOUNT;
        const FS_RENAME_DOES_D_MOVE = bindings::FS_RENAME_DOES_D_MOVE;
    }
}

pub fn register<T: FileSystem>() -> error::KernelResult<FileSystemRegistration<T>> {
    if !T::NAME.ends_with('\x00') {
        return Err(error::Error::EINVAL);
    }
    let mut fs_registration = FileSystemRegistration {
        ptr: bindings::file_system_type {
            name: T::NAME.as_ptr() as *const i8,
            owner: &mut bindings::__this_module,
        },
        _phantom: marker::PhantomData,
    };
    let result = unsafe { bindings::register_filesystem(&mut fs_registration.ptr) };
    if result != 0 {
        return Err(error::Error::from_kernel_errno(result));
    }

    return Ok(fs_registration);
}
