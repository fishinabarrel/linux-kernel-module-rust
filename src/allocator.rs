use alloc::heap::{AllocErr, GlobalAlloc, Layout, Opaque};

use bindings;
use types;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        return bindings::krealloc(
            0 as *const types::c_void,
            layout.size(),
            bindings::GFP_KERNEL,
        ) as *mut Opaque;
    }

    unsafe fn dealloc(&self, ptr: *mut Opaque, _layout: Layout) {
        bindings::kfree(ptr as *const types::c_void);
    }
}

#[lang = "oom"]
extern "C" fn oom(_err: AllocErr) -> ! {
    panic!("Out of memory!");
}
