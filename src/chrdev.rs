use core::convert::TryInto;
use core::ops::Range;
use core::{mem, ptr};

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::bindings;
use crate::c_types;
use crate::error::{Error, KernelResult};
use crate::user_ptr::{UserSlicePtr, UserSlicePtrWriter};

pub fn builder(name: &'static str, minors: Range<u16>) -> KernelResult<Builder> {
    if !name.ends_with('\x00') {
        return Err(Error::EINVAL);
    }

    Ok(Builder {
        name,
        minors,
        file_ops: vec![],
    })
}

pub struct Builder {
    name: &'static str,
    minors: Range<u16>,
    file_ops: Vec<&'static FileOperationsVtable>,
}

impl Builder {
    pub fn register_device<T: FileOperations>(mut self) -> Builder {
        if self.file_ops.len() >= self.minors.len() {
            panic!("More devices registered than minor numbers allocated.")
        }
        self.file_ops.push(&T::VTABLE);
        self
    }

    pub fn build(self) -> KernelResult<Registration> {
        let mut dev: bindings::dev_t = 0;
        let res = unsafe {
            bindings::alloc_chrdev_region(
                &mut dev,
                self.minors.start.into(),
                self.minors.len().try_into()?,
                self.name.as_ptr() as *const c_types::c_char,
            )
        };
        if res != 0 {
            return Err(Error::from_kernel_errno(res));
        }

        // Turn this into a boxed slice immediately because the kernel stores pointers into it, and
        // so that data should never be moved.
        let mut cdevs = vec![unsafe { mem::zeroed() }; self.file_ops.len()].into_boxed_slice();
        for (i, file_op) in self.file_ops.iter().enumerate() {
            unsafe {
                bindings::cdev_init(&mut cdevs[i], &file_op.0);
                cdevs[i].owner = &mut bindings::__this_module;
                let rc = bindings::cdev_add(&mut cdevs[i], dev + i as bindings::dev_t, 1);
                if rc != 0 {
                    // Clean up the ones that were allocated.
                    for j in 0..=i {
                        bindings::cdev_del(&mut cdevs[j]);
                    }
                    bindings::unregister_chrdev_region(dev, self.minors.len() as _);
                    return Err(Error::from_kernel_errno(rc));
                }
            }
        }

        Ok(Registration {
            dev,
            count: self.minors.len(),
            cdevs,
        })
    }
}

pub struct Registration {
    dev: bindings::dev_t,
    count: usize,
    cdevs: Box<[bindings::cdev]>,
}

// This is safe because Registration doesn't actually expose any methods.
unsafe impl Sync for Registration {}

impl Drop for Registration {
    fn drop(&mut self) {
        unsafe {
            for dev in self.cdevs.iter_mut() {
                bindings::cdev_del(dev);
            }
            bindings::unregister_chrdev_region(self.dev, self.count as _);
        }
    }
}

pub struct FileOperationsVtable(bindings::file_operations);

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

unsafe extern "C" fn read_callback<T: FileOperations>(
    file: *mut bindings::file,
    buf: *mut c_types::c_char,
    len: c_types::c_size_t,
    offset: *mut bindings::loff_t,
) -> c_types::c_ssize_t {
    let mut data = match UserSlicePtr::new(buf as *mut c_types::c_void, len) {
        Ok(ptr) => ptr.writer(),
        Err(e) => return e.to_kernel_errno() as c_types::c_ssize_t,
    };
    let f = &*((*file).private_data as *const T);
    // TODO: Pass offset to read()?
    match f.read(&mut data) {
        Ok(()) => {
            let written = len - data.len();
            (*offset) += written as bindings::loff_t;
            written as c_types::c_ssize_t
        }
        Err(e) => e.to_kernel_errno() as c_types::c_ssize_t,
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

impl FileOperationsVtable {
    pub const fn new<T: FileOperations>() -> FileOperationsVtable {
        FileOperationsVtable(bindings::file_operations {
            open: Some(open_callback::<T>),
            read: Some(read_callback::<T>),
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
            read_iter: None,
            #[cfg(kernel_4_20_0_or_greater)]
            remap_file_range: None,
            sendpage: None,
            setfl: None,
            setlease: None,
            show_fdinfo: None,
            splice_read: None,
            splice_write: None,
            unlocked_ioctl: None,
            write: None,
            write_iter: None,
        })
    }
}

pub trait FileOperations: Sync + Sized {
    const VTABLE: FileOperationsVtable;

    fn open() -> KernelResult<Self>;
    fn read(&self, buf: &mut UserSlicePtrWriter) -> KernelResult<()>;
}
