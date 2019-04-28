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

/// A reference to an area in userspace memory, which can be either
/// read-only or read-write.
///
/// All methods on this struct are safe: invalid pointers return
/// `EFAULT`. Concurrent access, _including data races to/from userspace
/// memory_, is permitted, because fundamentally another userspace
/// thread / process could always be modifying memory at the same time
/// (in the same way that userspace Rust's std::io permits data races
/// with the contents of files on disk). In the presence of a race, the
/// exact byte values read/written are unspecified but the operation is
/// well-defined. Kernelspace code should validate its copy of data
/// after completing a read, and not expect that multiple reads of the
/// same address will return the same value.
///
/// Constructing a `UserSlicePtr` only checks that the range is in valid
/// userspace memory, and does not depend on the current process (and
/// can safely be constructed inside a kernel thread with no current
/// userspace process). Reads and writes wrap the kernel APIs
/// `copy_from_user` and `copy_to_user`, and check the memory map of the
/// current process.
pub struct UserSlicePtr(*mut c_types::c_void, usize);

impl UserSlicePtr {
    /// Construct a user slice from a raw pointer and a length in bytes.
    ///
    /// Checks that the provided range is within the legal area for
    /// userspace memory, using `access_ok` (e.g., on i386, the range
    /// must be within the first 3 gigabytes), but does not check that
    /// the actual pages are mapped in the current process with
    /// appropriate permissions. Those checks are handled in the read
    /// and write methods.
    pub fn new(ptr: *mut c_types::c_void, length: usize) -> error::KernelResult<UserSlicePtr> {
        // No current access_ok implementation actually distinguishes
        // between VERIFY_READ and VERIFY_WRITE, so passing VERIFY_WRITE
        // is fine in practice and fails safe if a future implementation
        // bothers.
        if unsafe { access_ok_helper(bindings::VERIFY_WRITE, ptr, length as c_types::c_ulong) } == 0
        {
            return Err(error::Error::EFAULT);
        }
        return Ok(UserSlicePtr(ptr, length));
    }

    /// Read the entirety of the user slice and return it in a `Vec`.
    ///
    /// Returns EFAULT if the address does not currently point to
    /// mapped, readable memory.
    pub fn read_all(self) -> error::KernelResult<Vec<u8>> {
        let mut data = vec![0; self.1];
        self.reader().read(&mut data)?;
        return Ok(data);
    }

    /// Construct a `UserSlicePtrReader` that can incrementally read
    /// from the user slice.
    pub fn reader(self) -> UserSlicePtrReader {
        return UserSlicePtrReader(self.0, self.1);
    }

    /// Write the provided slice into the user slice.
    ///
    /// Returns EFAULT if the address does not currently point to
    /// mapped, writable memory (in which case some data from before the
    /// fault may be written), or `data` is larger than the user slice
    /// (in which case no data is written).
    pub fn write_all(self, data: &[u8]) -> error::KernelResult<()> {
        return self.writer().write(data);
    }

    /// Construct a `UserSlicePtrWrite` that can incrementally write
    /// into the user slice.
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
                data.len() as _,
            )
        };
        if res != 0 {
            return Err(error::Error::EFAULT);
        }
        // Since this is not a pointer to a valid object in our program,
        // we cannot use `add`, which has C-style rules for defined
        // behavior.
        self.0 = self.0.wrapping_add(data.len());
        self.1 -= data.len();
        Ok(())
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
                data.len() as _,
            )
        };
        if res != 0 {
            return Err(error::Error::EFAULT);
        }
        // Since this is not a pointer to a valid object in our program,
        // we cannot use `add`, which has C-style rules for defined
        // behavior.
        self.0 = self.0.wrapping_add(data.len());
        self.1 -= data.len();
        Ok(())
    }
}
