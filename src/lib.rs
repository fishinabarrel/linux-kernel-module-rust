#![no_std]
#![feature(allocator_api, const_fn, alloc_error_handler)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate byte_strings;

use core::panic::PanicInfo;

mod allocator;
pub mod bindings;
mod c_types;
pub mod chrdev;
mod error;
pub mod filesystem;
#[macro_use]
pub mod printk;
pub mod sysctl;
mod types;
pub mod user_ptr;

pub use error::{Error, KernelResult};
pub use types::Mode;

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
        kernel_module!(@modinfo $name, concat_bytes!(as_bytes!(stringify!($name), b"=", as_bytes!($value))));
    };

    (@modinfo $name:ident, $bytes:expr) => {
        #[link_section = ".modinfo"]
        #[allow(non_upper_case_globals)]
        // TODO: Generate a name the same way the kernel's `__MODULE_INFO` does.
        // TODO: Use of .len() here requires `#![feature(const_slice_len)]`.
        // We'd prefer [u8; ]. See https://github.com/rust-lang/rfcs/pull/2545
        pub static $name: [u8; ($bytes).len()] = *($bytes);
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
