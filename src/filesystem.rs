use core::marker;
use core::mem;

use bindings;

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

pub trait FileSystem {}

pub fn register<T: FileSystem>() -> Result<FileSystemRegistration<T>> {
    let mut fs_registration = FileSystemRegistration {
        ptr: /*bindings::file_system_type {},*/ unsafe {mem::zeroed() },
        _phantom: marker::PhantomData,
    };
    let result = unsafe { bindings::register_filesystem(&mut fs_registration.ptr) };
    if result != 0 {
        return Err(Error::from_kernel_errno(result));
    }

    return Ok(fs_registration);
}
