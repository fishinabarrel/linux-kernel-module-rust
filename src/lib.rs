#![no_std]
#![feature(allocator_api, alloc_error_handler, const_fn)]

extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
pub mod bindings;
mod c_types;
pub mod chrdev;
mod error;
pub mod filesystem;
pub mod printk;
pub mod sysctl;
mod types;
pub mod user_ptr;

pub use crate::error::{Error, KernelResult};
pub use crate::types::{CStr, Mode};

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
            $crate::kernel_module!(@attribute $name, $value);
        )*
    };

    (@attribute $name:ident, $value:expr) => {
        #[link_section = ".modinfo"]
        #[allow(non_upper_case_globals)]
        // TODO: Generate a name the same way the kernel's `__MODULE_INFO` does.
        // TODO: This needs to be a `[u8; _]`, since the kernel defines this as a  `const char []`.
        // See https://github.com/rust-lang/rfcs/pull/2545
        pub static $name: &'static [u8] = concat!(stringify!($name), "=", $value, '\0').as_bytes();
    };
}

pub trait KernelModule: Sized + Sync {
    fn init() -> KernelResult<Self>;
}

extern "C" {
    fn bug_helper() -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        bug_helper();
    }
}

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;
