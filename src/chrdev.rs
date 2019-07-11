use core::convert::TryInto;
use core::ops::Range;

use crate::bindings;
use crate::c_types;
use crate::error;

pub fn builder(name: &'static str, minors: Range<u16>) -> error::KernelResult<Builder> {
    if !name.ends_with('\x00') {
        return Err(error::Error::EINVAL);
    }

    return Ok(Builder { name, minors });
}

pub struct Builder {
    name: &'static str,
    minors: Range<u16>,
}

impl Builder {
    pub fn build(self) -> error::KernelResult<Registration> {
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
            return Err(error::Error::from_kernel_errno(res));
        }
        return Ok(Registration {
            dev,
            count: self.minors.len(),
        });
    }
}

pub struct Registration {
    dev: bindings::dev_t,
    count: usize,
}

impl Drop for Registration {
    fn drop(&mut self) {
        unsafe {
            bindings::unregister_chrdev_region(self.dev, self.count as _);
        }
    }
}
