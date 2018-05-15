use alloc::heap::{AllocErr, GlobalAlloc, Layout, Opaque};

use bindings;
use types;

pub struct KernelAllocator;

// bindgen problems
const GFP_KERNEL: types::c_uint = 0xc000c0;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        return bindings::krealloc(0 as *const types::c_void, layout.size(), GFP_KERNEL)
            as *mut Opaque;
    }

    unsafe fn dealloc(&self, ptr: *mut Opaque, _layout: Layout) {
        bindings::kfree(ptr as *const types::c_void);
    }
}

#[lang = "oom"]
extern "C" fn oom(_err: AllocErr) -> ! {
    panic!("Out of memory!");
}
