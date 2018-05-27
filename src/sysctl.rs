use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use core::ptr;

use bindings;
use error;
use types;

pub struct Sysctl<T: Sync> {
    inner: Box<T>,
    table: Box<[bindings::ctl_table]>,
    header: *mut bindings::ctl_table_header,
}

impl<T: Sync> Sysctl<T> {
    pub fn register(path: &str, storage: T, mode: types::Mode) -> error::KernelResult<Sysctl<T>> {
        let namespace_pos = path.rfind('/').ok_or(error::Error::EINVAL)?;
        let (namespace, name) = path.split_at(namespace_pos);
        let namespace = namespace.to_string() + "\x00";
        let name = name.to_string() + "\x00";

        let storage = Box::new(storage);

        let table = vec![bindings::ctl_table { name }, bindings::ctl_table {}].into_boxed_slice();

        let result = bindings::register_sysctl(namespace, table);
        if result.is_null() {
            return Err(error::Error::XXX);
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
        bindings::unregister_sysctl_table(self.header);
        self.header = ptr::null_mut();
    }
}
