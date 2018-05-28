use alloc::Vec;

use bindings;
use c_types;
use error;

extern "C" {
    fn access_ok_helper(
        mode: c_types::c_uint,
        addr: *const c_types::c_void,
        len: c_types::c_ulong,
    ) -> c_types::c_int;
}

pub struct UserSlicePtr(*mut c_types::c_void, usize);

impl UserSlicePtr {
    pub fn new(ptr: *mut c_types::c_void, length: usize) -> error::KernelResult<UserSlicePtr> {
        if unsafe { access_ok_helper(bindings::VERIFY_WRITE, ptr, length as c_types::c_ulong) } != 0
        {
            return Err(error::Error::EFAULT);
        }
        return Ok(UserSlicePtr(ptr, length));
    }

    pub fn read_all(self) -> error::KernelResult<Vec<u8>> {
        unimplemented!();
    }
}
