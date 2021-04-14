//! APIs for interacting with the scheduler and with processes,
//! corresponding to <linux/sched.h> and related header files.
#![allow(improper_ctypes)]

use core::ptr;

use crate::bindings;
use crate::rcu;

extern "C" {
    fn task_lock_helper(p: *mut bindings::task_struct);
    fn task_unlock_helper(p: *mut bindings::task_struct);
    fn next_task_helper(p: *mut bindings::task_struct) -> *mut bindings::task_struct;
}

/// Represents a `struct task_struct *`.
pub struct TaskStruct<'a>(&'a mut bindings::task_struct);

impl TaskStruct<'_> {
    /// Returns the threadgroup ID (what userspace calls the process ID).
    pub fn tgid(&self) -> i32 {
        self.0.tgid
    }

    /// Returns the command name / process title. This is a short name,
    /// typically the base name of the command, and does not have the
    /// full path or arguments. It's a fixed-sized set of bytes, but by
    /// convention it's interpreted as NUL-terminated.
    pub fn comm(&mut self) -> [u8; bindings::TASK_COMM_LEN as usize] {
        let mut result = [0u8; bindings::TASK_COMM_LEN as usize];
        unsafe {
            task_lock_helper(self.0);
        }
        // if only char were unsigned char
        for (src, dst) in self.0.comm.iter().zip(result.iter_mut()) {
            if *src == 0 {
                break;
            }
            *dst = *src as _;
        }
        unsafe {
            task_unlock_helper(self.0);
        }
        result
    }
}

/// Iterate over every process on the system. Returns only processes,
/// i.e., thread group leaders.
///
/// ```
/// let g = rcu::RcuReadGuard::new();
/// for p in each_process(&g) {
///     println!("{:?}", p.comm());
/// }
/// ```
struct EachProcess<'g> {
    p: *mut bindings::task_struct,
    _g: &'g rcu::RcuReadGuard,
}

pub fn each_process(g: &rcu::RcuReadGuard) -> impl Iterator<Item = TaskStruct> {
    // unsafe is bogus here because we don't read it
    // https://github.com/rust-lang/rust/issues/74843
    EachProcess {
        p: unsafe { &mut bindings::init_task },
        _g: g,
    }
}

impl<'g> Iterator for EachProcess<'g> {
    type Item = TaskStruct<'g>;

    fn next(&mut self) -> Option<TaskStruct<'g>> {
        // Safety:
        // - oldp is valid if not null, because it is either &init_task
        //   (a static location) or updated by this function.
        // - next_task calls rcu_dereference internally, which is safe
        //   because we hold self._g.
        // - The returned reference has lifetime 'g, which is valid
        //   because self._g lives at least that long.
        let oldp = unsafe { self.p.as_mut()? };
        self.p = unsafe { next_task_helper(self.p) };
        if self.p == unsafe { &mut bindings::init_task } {
            self.p = ptr::null_mut();
        }
        Some(TaskStruct(oldp))
    }
}
