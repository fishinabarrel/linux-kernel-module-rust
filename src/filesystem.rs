use core::marker;
use core::mem;

pub struct FileSystemRegistration<T: FileSystem> {
    _phantom: marker::PhantomData<T>,
    ptr: bindings::file_system_type,
}

impl<T: FileSystem> Drop for FileSystemRegistration<T> {
    fn drop(&mut self) {
        bindings::unregister_filesystem(&self.ptr);
        self.ptr = unsafe { mem::zeroed() };
    }
}

pub trait FileSystem {}

pub fn register<T: FileSystem>() -> FileSystemRegistration<T> {
    let fs_registration = FileSystemRegistration {
        ptr: bindings::file_system_type {},
        _phantom: marker::PhantomData,
    };
    bindings::register_filesystem(&fs_registration.ptr);
    return fs_registration;
}
