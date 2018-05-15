use core::marker;
use core::mem;

use bindings;
use error;

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
}

pub fn register<T: FileSystem>() -> error::KernelResult<FileSystemRegistration<T>> {
    if !T::NAME.ends_with('\x00') {
        return Err(error::Error::EINVAL);
    }
    let mut fs_registration = FileSystemRegistration {
        ptr: bindings::file_system_type {
            name: T::NAME.as_ptr() as *const i8,
        },
        _phantom: marker::PhantomData,
    };
    let result = unsafe { bindings::register_filesystem(&mut fs_registration.ptr) };
    if result != 0 {
        return Err(error::Error::from_kernel_errno(result));
    }

    return Ok(fs_registration);
}
