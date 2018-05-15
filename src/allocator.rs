use alloc::heap::{Alloc, Layout, AllocErr};

use bindings;
use types;

pub struct KernelAllocator;

unsafe impl<'a> Alloc for &'a KernelAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8 AllocErr> {
        let ptr = bindings::krealloc(0 as *const types::c_void, layout.size(), bindings::GFP_KERNEL) as *mut u8;
        if ptr.is_null() {
            return Err(AllocErr::Exhausted{request: layout});
        }
        return Ok(ptr);
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        bindings::kfree(ptr as *const types::c_void);
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;
