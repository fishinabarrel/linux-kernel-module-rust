use error;
use types;

pub struct Sysctl<T: Sync> {
    inner: T,
}

impl<T: Sync> Sysctl<T> {
    pub fn register(path: &[u8], storage: T, mode: types::Mode) -> error::KernelResult<Sysctl<T>> {
        unimplemented!();
    }

    pub fn get(&self) -> &T {
        return &self.inner;
    }
}
