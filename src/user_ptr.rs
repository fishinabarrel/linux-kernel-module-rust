use core::mem;

use bindings;
use error;
use types;

extern "C" {
    fn access_ok_helper(
        mode: types::c_uint,
        addr: *const types::c_void,
        len: types::c_ulong,
    ) -> types::c_int;
}

// Our goal is to protect against all possible memory unsafety specific to the
// kernel's interaction with userland pointers (the sort of stuff covered in
// the bochspwn work). As a result:
// 1) At the earliest possible moment, a ptr from userspace (__user in the
//    kernel's C parlance) should be converted to a UserPtr<T>. If the value
//    is not a valid userspace pointer EFAULT is returned.
// 2) UserPtr<T> permits a given address to be read from and then written to
//    exactly once to prevent TOCTOU vulnerabilities
// 3) Later we consider supporting various optimizations (e.g. multiple writes
//    with only a single user_access_begin/user_access_end pair), but safety
//    should be paramount.

// We will also need variants for handling length + slice and for
// nul-terminated strings. They will obey similar rules, but have slightly
// different APIs.
pub struct UserPtr<T: Copy + ?Sized>(*mut T);

impl<T: Copy + ?Sized> UserPtr<T> {
    pub fn new(ptr: *mut T) -> error::KernelResult<UserPtr<T>> {
        if unsafe {
            access_ok_helper(
                bindings::VERIFY_WRITE,
                ptr as *const types::c_void,
                mem::size_of::<T>() as types::c_ulong,
            )
        } != 0
        {
            return Err(error::Error::EFAULT);
        }
        Ok(UserPtr(ptr))
    }

    pub fn read(self) -> (T, WriteOnlyUserPtr<T>) {
        unsafe {
            let mut val = mem::zeroed();
            let res = bindings::_copy_from_user(
                &mut val as *mut T as *mut types::c_void,
                self.0 as *const types::c_void,
                mem::size_of::<T>() as u32,
            );
            // TODO: can an error happen here, given we already verified
            // access_ok?
            assert!(res == mem::size_of::<T>() as u64);
            return (val, WriteOnlyUserPtr(self.0));
        }
    }

    pub fn write(self, src: T) {
        WriteOnlyUserPtr(self.0).write(src);
    }
}

pub struct WriteOnlyUserPtr<T: Copy + ?Sized>(*mut T);

impl<T: Copy + ?Sized> WriteOnlyUserPtr<T> {
    pub fn write(self, src: T) {
        unsafe {
            let res = bindings::_copy_to_user(
                self.0 as *mut types::c_void,
                &src as *const T as *const types::c_void,
                mem::size_of::<T>() as u32,
            );
            // TODO: can an error happen here?
            assert!(res == mem::size_of::<T>() as u64);
        }
    }
}
