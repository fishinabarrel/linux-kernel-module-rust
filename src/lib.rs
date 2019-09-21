#![no_std]
#![feature(allocator_api, alloc_error_handler, const_fn, const_raw_ptr_deref)]

extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
pub mod bindings;
pub mod c_types;
pub mod chrdev;
mod error;
pub mod filesystem;
pub mod printk;
pub mod sysctl;
mod types;
pub mod user_ptr;

pub use crate::error::{Error, KernelResult};
pub use crate::types::{CStr, Mode};

/// Declares the entrypoint for a kernel module. The first argument should be a type which
/// implements the [`KernelModule`] trait. Also accepts various forms of kernel metadata.
///
/// Example:
/// ```rust,no_run
/// use linux_kernel_module;
/// struct MyKernelModule;
/// impl linux_kernel_module::KernelModule for MyKernelModule {
///     fn init() -> linux_kernel_module::KernelResult<Self> {
///         Ok(MyKernelModule)
///     }
/// }
///
/// linux_kernel_module::kernel_module!(
///     MyKernelModule,
///     author: "Fish in a Barrel Contributors",
///     description: "My very own kernel module!",
///     license: "GPL"
/// );
#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),*) => {
        static mut __MOD: Option<$module> = None;
        #[no_mangle]
        pub extern "C" fn init_module() -> $crate::c_types::c_int {
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

/// KernelModule is the top level entrypoint to implementing a kernel module. Your kernel module
/// should implement the `init` method on it, which maps to the `module_init` macro in Linux C API.
/// You can use this method to do whatever setup or registration your module should do. For any
/// teardown or cleanup operations, your type may implement [`Drop`].
///
/// [`Drop`]: https://doc.rust-lang.org/stable/core/ops/trait.Drop.html
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
