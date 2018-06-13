use core::alloc::{AllocErr, GlobalAlloc, Layout};

use bindings;
use c_types;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // krealloc is used instead of kmalloc because kmalloc is an inline function and can't be
        // bound to as a result
        return bindings::krealloc(
            0 as *const c_types::c_void,
            layout.size(),
            bindings::GFP_KERNEL,
        ) as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        bindings::kfree(ptr as *const c_types::c_void);
    }
}

#[lang = "oom"]
extern "C" fn oom(_err: AllocErr) -> ! {
    panic!("Out of memory!");
}
