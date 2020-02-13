use core::convert::{TryFrom, TryInto};
use core::{marker, mem, ptr};

use alloc::boxed::Box;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::user_ptr::{UserSlicePtr, UserSlicePtrReader, UserSlicePtrWriter};

pub struct File {
    ptr: *const bindings::file,
}

impl File {
    unsafe fn from_ptr(ptr: *const bindings::file) -> File {
        File { ptr }
    }

    pub fn pos(&self) -> u64 {
        unsafe { (*self.ptr).f_pos as u64 }
    }
}

// Matches std::io::SeekFrom in the Rust stdlib
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub struct FileOperationsVtable(pub(crate) bindings::file_operations);

unsafe extern "C" fn open_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let f = match T::open() {
        Ok(f) => Box::new(f),
        Err(e) => return e.to_kernel_errno(),
    };
    (*file).private_data = Box::into_raw(f) as *mut c_types::c_void;
    0
}

unsafe extern "C" fn read_callback<T: Read>(
    file: *mut bindings::file,
    buf: *mut c_types::c_char,
    len: c_types::c_ulonglong,
    offset: *mut bindings::loff_t,
) -> c_types::c_longlong {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len.try_into().unwrap()) {
        Ok(ptr) => ptr.writer(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };
    let f = &*((*file).private_data as *const T);
    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };
    match f.read(&mut data, positive_offset) {
        Ok(()) => {
            let written = len - (data.len() as u64);
            (*offset) += bindings::loff_t::try_from(written).unwrap();
            written.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn write_callback<T: Write>(
    file: *mut bindings::file,
    buf: *const c_types::c_char,
    len: c_types::c_ulonglong,
    offset: *mut bindings::loff_t,
) -> c_types::c_longlong {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len.try_into().unwrap()) {
        Ok(ptr) => ptr.reader(),
        Err(e) => return e.to_kernel_errno().try_into().unwrap(),
    };
    let f = &*((*file).private_data as *const T);
    // No FMODE_UNSIGNED_OFFSET support, so offset must be in [0, 2^63).
    // See discussion in #113
    let positive_offset = match (*offset).try_into() {
        Ok(v) => v,
        Err(_) => return Error::EINVAL.to_kernel_errno().try_into().unwrap(),
    };
    match f.write(&mut data, positive_offset) {
        Ok(()) => {
            let read = len - (data.len() as u64);
            (*offset) += bindings::loff_t::try_from(read).unwrap();
            read.try_into().unwrap()
        }
        Err(e) => e.to_kernel_errno().try_into().unwrap(),
    }
}

unsafe extern "C" fn release_callback<T: FileOperations>(
    _inode: *mut bindings::inode,
    file: *mut bindings::file,
) -> c_types::c_int {
    let ptr = mem::replace(&mut (*file).private_data, ptr::null_mut());
    drop(Box::from_raw(ptr as *mut T));
    0
}

unsafe extern "C" fn llseek_callback<T: Seek>(
    file: *mut bindings::file,
    offset: bindings::loff_t,
    whence: c_types::c_int,
) -> bindings::loff_t {
    let off = match whence as u32 {
        bindings::SEEK_SET => match offset.try_into() {
            Ok(v) => SeekFrom::Start(v),
            Err(_) => return Error::EINVAL.to_kernel_errno().into(),
        },
        bindings::SEEK_CUR => SeekFrom::Current(offset),
        bindings::SEEK_END => SeekFrom::End(offset),
        _ => return Error::EINVAL.to_kernel_errno().into(),
    };
    let f = &*((*file).private_data as *const T);
    match f.seek(&File::from_ptr(file), off) {
        Ok(off) => off as bindings::loff_t,
        Err(e) => e.to_kernel_errno().into(),
    }
}

impl FileOperationsVtable {
    pub const fn builder<T: FileOperations>() -> FileOperationsVtableBuilder<T> {
        FileOperationsVtableBuilder(
            bindings::file_operations {
                open: Some(open_callback::<T>),
                release: Some(release_callback::<T>),

                check_flags: None,
                #[cfg(not(kernel_4_20_0_or_greater))]
                clone_file_range: None,
                compat_ioctl: None,
                copy_file_range: None,
                #[cfg(not(kernel_4_20_0_or_greater))]
                dedupe_file_range: None,
                fallocate: None,
                #[cfg(kernel_4_19_0_or_greater)]
                fadvise: None,
                fasync: None,
                flock: None,
                flush: None,
                fsync: None,
                get_unmapped_area: None,
                iterate: None,
                iterate_shared: None,
                #[cfg(kernel_5_1_0_or_greater)]
                iopoll: None,
                llseek: None,
                lock: None,
                mmap: None,
                #[cfg(kernel_4_15_0_or_greater)]
                mmap_supported_flags: 0,
                owner: ptr::null_mut(),
                poll: None,
                read: None,
                read_iter: None,
                #[cfg(kernel_4_20_0_or_greater)]
                remap_file_range: None,
                sendpage: None,
                #[cfg(kernel_aufs_setfl)]
                setfl: None,
                setlease: None,
                show_fdinfo: None,
                splice_read: None,
                splice_write: None,
                unlocked_ioctl: None,
                write: None,
                write_iter: None,
            },
            marker::PhantomData,
        )
    }
}

pub struct FileOperationsVtableBuilder<T>(bindings::file_operations, marker::PhantomData<T>);

impl<T> FileOperationsVtableBuilder<T> {
    pub const fn build(self) -> FileOperationsVtable {
        FileOperationsVtable(self.0)
    }
}

impl<T: Read> FileOperationsVtableBuilder<T> {
    pub const fn read(mut self) -> FileOperationsVtableBuilder<T> {
        self.0.read = Some(read_callback::<T>);
        self
    }
}

impl<T: Write> FileOperationsVtableBuilder<T> {
    pub const fn write(mut self) -> FileOperationsVtableBuilder<T> {
        self.0.write = Some(write_callback::<T>);
        self
    }
}

impl<T: Seek> FileOperationsVtableBuilder<T> {
    pub const fn seek(mut self) -> FileOperationsVtableBuilder<T> {
        self.0.llseek = Some(llseek_callback::<T>);
        self
    }
}

/// `FileOperations` corresponds to the kernel's `struct file_operations`. You
/// implement this trait whenever you'd create a `struct file_operations`, and
/// also an additional trait for each function pointer in the
/// `struct file_operations`. File descriptors may be used from multiple threads
/// (or processes) concurrently, so your type must be `Sync`.
pub trait FileOperations: Sync + Sized {
    /// A container for the actual `file_operations` value. This will always be:
    /// ```
    /// const VTABLE: linux_kernel_module::chrdev::FileOperationsVtable =
    ///     linux_kernel_module::chrdev::FileOperationsVtable::new::<Self>();
    /// ```
    const VTABLE: FileOperationsVtable;

    /// Creates a new instance of this file. Corresponds to the `open` function
    /// pointer in `struct file_operations`.
    fn open() -> KernelResult<Self>;
}

pub trait Read {
    /// Reads data from this file to userspace. Corresponds to the `read`
    /// function pointer in `struct file_operations`.
    fn read(&self, buf: &mut UserSlicePtrWriter, offset: u64) -> KernelResult<()>;
}

pub trait Write {
    /// Writes data from userspace o this file. Corresponds to the `write`
    /// function pointer in `struct file_operations`.
    fn write(&self, buf: &mut UserSlicePtrReader, offset: u64) -> KernelResult<()>;
}

pub trait Seek {
    /// Changes the position of the file. Corresponds to the `llseek` function
    /// pointer in `struct file_operations`.
    fn seek(&self, file: &File, offset: SeekFrom) -> KernelResult<u64>;
}
