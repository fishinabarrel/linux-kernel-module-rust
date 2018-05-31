use alloc::vec;
use alloc::vec::Vec;
use core::u32;

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
        if unsafe { access_ok_helper(bindings::VERIFY_WRITE, ptr, length as c_types::c_ulong) } == 0
        {
            return Err(error::Error::EFAULT);
        }
        return Ok(UserSlicePtr(ptr, length));
    }

    pub fn read_all(self) -> error::KernelResult<Vec<u8>> {
        let mut data = vec![0; self.1];
        self.reader().read(&mut data)?;
        return Ok(data);
    }

    pub fn reader(self) -> UserSlicePtrReader {
        return UserSlicePtrReader(self.0, self.1);
    }

    pub fn write_all(self, data: &[u8]) -> error::KernelResult<()> {
        return self.writer().write(data);
    }

    pub fn writer(self) -> UserSlicePtrWriter {
        return UserSlicePtrWriter(self.0, self.1);
    }
}

pub struct UserSlicePtrReader(*mut c_types::c_void, usize);

impl UserSlicePtrReader {
    pub fn read(&mut self, data: &mut [u8]) -> error::KernelResult<()> {
        if data.len() > self.1 || data.len() > u32::MAX as usize {
            return Err(error::Error::EFAULT);
        }
        let res = unsafe {
            bindings::_copy_from_user(
                data.as_mut_ptr() as *mut c_types::c_void,
                self.0,
                data.len() as u32,
            )
        };
        if res != 0 {
            return Err(error::Error::EFAULT);
        }
        unsafe {
            self.0 = self.0.add(data.len());
        }
        self.1 -= data.len();
        return Ok(());
    }
}

pub struct UserSlicePtrWriter(*mut c_types::c_void, usize);

impl UserSlicePtrWriter {
    pub fn write(&mut self, data: &[u8]) -> error::KernelResult<()> {
        if data.len() > self.1 || data.len() > u32::MAX as usize {
            return Err(error::Error::EFAULT);
        }
        let res = unsafe {
            bindings::_copy_to_user(
                self.0,
                data.as_ptr() as *const c_types::c_void,
                data.len() as u32,
            )
        };
        if res != 0 {
            return Err(error::Error::EFAULT);
        }
        unsafe {
            self.0 = self.0.add(data.len());
        }
        self.1 -= data.len();
        Ok(())
    }
}
