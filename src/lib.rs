#![no_std]
#![feature(lang_items)]

pub mod types;

#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),*) => {
        use $crate::KernelModule;
        static mut __MOD: Option<$module> = None;
        #[no_mangle]
        pub extern "C" fn init_module() -> $crate::types::c_int {
            match <$module as $crate::KernelModule>::init() {
                Ok(m) => {
                    unsafe {
                        __MOD = Some(m);
                    }
                    return 0;
                }
                Err(e) => {
                    return e.to_kernel_errno();
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn module_exit() {
            unsafe {
                __MOD.as_mut().unwrap().exit();
            }
        }
    };
}

pub enum Error {
}

impl Error {
    pub fn to_kernel_errno(&self) -> types::c_int {
        unimplemented!();
    }
}

pub trait KernelModule: Sized {
    fn init() -> Result<Self, Error>;
    fn exit(&mut self);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}
