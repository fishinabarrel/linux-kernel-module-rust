use alloc::vec;
use alloc::vec::Vec;
use core::u32;

use crate::bindings;
use crate::c_types;
use crate::error;

extern "C" {
    fn access_ok_helper(addr: *const c_types::c_void, len: c_types::c_ulong) -> c_types::c_int;
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
/// All APIs enforce the invariant that a given byte of memory from userspace
/// may only be read once. By pretenting double-fetches we avoid TOCTOU
/// vulnerabilities. This is accomplished by taking `self` by value to prevent
/// obtaining multiple readers on a given UserSlicePtr, and the readers only
/// permitting forward reads.
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
    ///
    /// This is `unsafe` because if it is called within `set_fs(KERNEL_DS)` context then
    /// `access_ok` will not do anything. As a result the only place you can safely use this is
    /// with an `__user` pointer that was provided by the kernel.
    pub(crate) unsafe fn new(
        ptr: *mut c_types::c_void,
        length: usize,
    ) -> error::KernelResult<UserSlicePtr> {
        if access_ok_helper(ptr, length as c_types::c_ulong) == 0 {
            return Err(error::Error::EFAULT);
        }
        Ok(UserSlicePtr(ptr, length))
    }

    /// Read the entirety of the user slice and return it in a `Vec`.
    ///
    /// Returns EFAULT if the address does not currently point to
    /// mapped, readable memory.
    pub fn read_all(self) -> error::KernelResult<Vec<u8>> {
        self.reader().read_all()
    }

    /// Construct a `UserSlicePtrReader` that can incrementally read
    /// from the user slice.
    pub fn reader(self) -> UserSlicePtrReader {
        UserSlicePtrReader(self.0, self.1)
    }

    /// Write the provided slice into the user slice.
    ///
    /// Returns EFAULT if the address does not currently point to
    /// mapped, writable memory (in which case some data from before the
    /// fault may be written), or `data` is larger than the user slice
    /// (in which case no data is written).
    pub fn write_all(self, data: &[u8]) -> error::KernelResult<()> {
        self.writer().write(data)
    }

    /// Construct a `UserSlicePtrWrite` that can incrementally write
    /// into the user slice.
    pub fn writer(self) -> UserSlicePtrWriter {
        UserSlicePtrWriter(self.0, self.1)
    }
}

pub struct UserSlicePtrReader(*mut c_types::c_void, usize);

impl UserSlicePtrReader {
    /// Returns the number of bytes left to be read from this. Note that even
    /// reading less than this number of bytes may return an Error().
    pub fn len(&self) -> usize {
        self.1
    }

    /// Returns `true` if `self.len()` is 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Read all data remaining in the user slice and return it in a `Vec`.
    ///
    /// Returns EFAULT if the address does not currently point to
    /// mapped, readable memory.
    pub fn read_all(&mut self) -> error::KernelResult<Vec<u8>> {
        let mut data = vec![0; self.1];
        self.read(&mut data)?;
        Ok(data)
    }

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
    pub fn len(&self) -> usize {
        self.1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
