use crate::bindings;
use crate::c_types;

pub struct Error(c_types::c_int);

impl Error {
    pub const EINVAL: Self = Error(-(bindings::EINVAL as i32));
    pub const ENOMEM: Self = Error(-(bindings::ENOMEM as i32));
    pub const EFAULT: Self = Error(-(bindings::EFAULT as i32));

    pub fn from_kernel_errno(errno: c_types::c_int) -> Error {
        return Error(errno);
    }

    pub fn to_kernel_errno(&self) -> c_types::c_int {
        return self.0;
    }
}

pub type KernelResult<T> = Result<T, Error>;
