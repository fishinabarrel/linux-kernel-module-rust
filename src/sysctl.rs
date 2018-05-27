use alloc::boxed::Box;
use alloc::vec;
use core::mem;
use core::ptr;

use bindings;
use c_types;
use error;
use types;

pub struct Sysctl<T: Sync> {
    inner: Box<T>,
    table: Box<[bindings::ctl_table]>,
    header: *mut bindings::ctl_table_header,
}

unsafe extern "C" fn proc_handler<T>(
    ctl: *mut bindings::ctl_table,
    write: c_types::c_int,
    buffer: *mut c_types::c_void,
    len: *mut usize,
    ppos: *mut bindings::loff_t,
) -> c_types::c_int {
    unimplemented!();
}

impl<T: Sync> Sysctl<T> {
    pub fn register(
        path: &'static str,
        name: &'static str,
        storage: T,
        mode: types::Mode,
    ) -> error::KernelResult<Sysctl<T>> {
        if !path.ends_with('\x00') || !name.ends_with('\x00') || name.contains('\x00') {
            return Err(error::Error::EINVAL);
        }

        let mut storage = Box::new(storage);
        let mut table = vec![
            bindings::ctl_table {
                procname: name.as_ptr() as *const i8,
                mode: mode.as_int(),
                data: &mut *storage as *mut T as *mut c_types::c_void,
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

impl<T: Sync> Drop for Sysctl<T> {
    fn drop(&mut self) {
        unsafe {
            bindings::unregister_sysctl_table(self.header);
        }
        self.header = ptr::null_mut();
    }
}
