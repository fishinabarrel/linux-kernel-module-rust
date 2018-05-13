#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),* ) => (

    );
}

pub enum Error {

}

pub trait KernelModule : Sized {
    fn init() -> Result<Self, Error>;
    fn exit(&mut self);
}
