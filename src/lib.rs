#![no_std]
#![feature(alloc, global_allocator, allocator_api, const_fn, lang_items)]

extern crate alloc;
#[macro_use]
extern crate bitflags;

mod allocator;
pub mod bindings;
mod c_types;
mod error;
pub mod filesystem;
#[macro_use]
pub mod printk;

pub use error::{Error, KernelResult};

pub type _InitResult = c_types::c_int;

#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),*) => {
        static mut __MOD: Option<$module> = None;
        #[no_mangle]
        pub extern "C" fn init_module() -> $crate::_InitResult {
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
        pub extern "C" fn cleanup_module() {
            unsafe {
                // Invokes drop() on __MOD, which should be used for cleanup.
                __MOD = None;
            }
        }

        $(
            kernel_module!(@attribute $name, $value);
        )*
    };

    (@attribute $name:ident, $value:expr) => {
        #[link_section = ".modinfo"]
        #[allow(non_upper_case_globals)]
        // TODO: Generate a name the same way the kernel's `__MODULE_INFO` does.
        // TODO: This needs to be a `&'static [u8]`, since the kernel defines this as a
        // `const char []`.
        pub static $name: &'static str = concat!(stringify!($name), "=", $value);
    };
}

pub trait KernelModule: Sized {
    fn init() -> KernelResult<Self>;
}

extern "C" {
    fn bug_helper() -> !;
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    unsafe {
        bug_helper();
    }
}

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;
