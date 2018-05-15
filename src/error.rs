use types;

struct Error(types::c_int);

impl Error {
    pub fn from_kernel_errno(errno: types::c_int) -> Error {
        return Error(errno);
    }

    pub fn to_kernel_errno(&self) -> types::c_int {
        return self.0;
    }
}

type KernelResult<T> = Result<T, Error>;
