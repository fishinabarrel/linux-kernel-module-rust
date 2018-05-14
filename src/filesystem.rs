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

pub fn register<T: FileSystem>() -> FileSystemRegistration<T> {
    let mut fs_registration = FileSystemRegistration {
        ptr: /*bindings::file_system_type {},*/ unsafe {mem::zeroed() },
        _phantom: marker::PhantomData,
    };
    unsafe { bindings::register_filesystem(&mut fs_registration.ptr) };
    return fs_registration;
}
