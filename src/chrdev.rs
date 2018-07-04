use bindings;
use c_types;
use error;

pub struct DeviceNumberRegion {
    dev: bindings::dev_t,
    count: usize,
}

impl DeviceNumberRegion {
    pub fn allocate(
        count: usize,
        first_minor: usize,
        name: &'static str,
    ) -> error::KernelResult<DeviceNumberRegion> {
        if !name.ends_with('\x00') {
            return Err(error::Error::EINVAL);
        }

        let mut dev: bindings::dev_t = 0;
        let res = unsafe {
            bindings::alloc_chrdev_region(
                &mut dev,
                first_minor as bindings::dev_t,
                count as bindings::dev_t,
                name.as_ptr() as *const c_types::c_char,
            )
        };
        if res != 0 {
            return Err(error::Error::from_kernel_errno(res));
        }
        return Ok(DeviceNumberRegion { dev, count });
    }
}

impl Drop for DeviceNumberRegion {
    fn drop(&mut self) {
        unsafe {
            bindings::unregister_chrdev_region(self.dev, self.count as bindings::dev_t);
        }
    }
}
