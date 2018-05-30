use alloc::boxed::Box;
use alloc::vec;
use core::mem;
use core::ptr;
use core::sync::atomic;

use bindings;
use c_types;
use error;
use types;
use user_ptr::UserSlicePtr;

pub struct Sysctl<T: SysctlStorage> {
    inner: Box<T>,
    table: Box<[bindings::ctl_table]>,
    header: *mut bindings::ctl_table_header,
}

pub trait SysctlStorage: Sync {
    fn store_value(&self, data: &[u8]) -> (usize, error::KernelResult<()>);
    fn read_value(&self, data: &mut UserSlicePtr) -> (usize, error::KernelResult<()>);
}

impl SysctlStorage for atomic::AtomicBool {
    fn store_value(&self, data: &[u8]) -> (usize, error::KernelResult<()>) {
        unimplemented!();
    }

    fn read_value(&self, data: &mut UserSlicePtr) -> (usize, error::KernelResult<()>) {
        unimplemented!();
    }
}

unsafe extern "C" fn proc_handler<T: SysctlStorage>(
    ctl: *mut bindings::ctl_table,
    write: c_types::c_int,
    buffer: *mut c_types::c_void,
    len: *mut usize,
    ppos: *mut bindings::loff_t,
) -> c_types::c_int {
    let mut data = match UserSlicePtr::new(buffer, *len) {
        Ok(ptr) => ptr,
        Err(e) => return e.to_kernel_errno(),
    };
    let storage = &*((*ctl).data as *const T);
    let (bytes_processed, result) = if write != 0 {
        let data = match data.read_all() {
            Ok(r) => r,
            Err(e) => return e.to_kernel_errno(),
        };
        storage.store_value(&data)
    } else {
        storage.read_value(&mut data)
    };
    *len -= bytes_processed;
    *ppos += bytes_processed as bindings::loff_t;
    match result {
        Ok(()) => 0,
        Err(e) => e.to_kernel_errno(),
    }
}

impl<T: SysctlStorage> Sysctl<T> {
    pub fn register(
        path: &'static str,
        name: &'static str,
        storage: T,
        mode: types::Mode,
    ) -> error::KernelResult<Sysctl<T>> {
        if !path.ends_with('\x00') || !name.ends_with('\x00') || name.contains('/') {
            return Err(error::Error::EINVAL);
        }

        let storage = Box::new(storage);
        let mut table = vec![
            bindings::ctl_table {
                procname: name.as_ptr() as *const i8,
                mode: mode.as_int(),
                data: &*storage as *const T as *mut c_types::c_void,
                proc_handler: Some(proc_handler::<T>),

                maxlen: 0,
                child: ptr::null_mut(),
                poll: ptr::null_mut(),
                extra1: ptr::null_mut(),
                extra2: ptr::null_mut(),
            },
            unsafe { mem::zeroed() },
        ].into_boxed_slice();

        let result =
            unsafe { bindings::register_sysctl(path.as_ptr() as *const i8, table.as_mut_ptr()) };
        if result.is_null() {
            return Err(error::Error::ENOMEM);
        }

        return Ok(Sysctl {
            inner: storage,
            table: table,
            header: result,
        });
    }

    pub fn get(&self) -> &T {
        return &self.inner;
    }
}

impl<T: SysctlStorage> Drop for Sysctl<T> {
    fn drop(&mut self) {
        unsafe {
            bindings::unregister_sysctl_table(self.header);
        }
        self.header = ptr::null_mut();
    }
}
