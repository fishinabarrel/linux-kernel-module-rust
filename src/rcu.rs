//! Bindings to RCU (read-copy-update), a high-performance lockless
//! synchronization system used by many kernel data structures. At the
//! moment, only calling functions that perform RCU reads is supported.

extern "C" {
    fn rcu_read_lock_helper();
    fn rcu_read_unlock_helper();
}

/// A guard representing an RCU read-side critical section. Its
/// constructor calls `rcu_read_lock()` and its destructor calls
/// `rcu_read_unlock()`.
///
/// Within a read-side critical section (i.e., while at least one
/// RcuReadGuard object is instantiated), objects behind RCU-protected
/// pointers are guaranteed not to change, and so reading from them
/// (after gaining a pointer with `rcu_dereference()`) is safe.
///
/// It is an error (risk of deadlock, but not memory unsafety) to block
/// or schedule while holding an RcuReadGuard. It is also an error
/// (guaranteed deadlock) to call `synchronize_rcu()` while holding an
/// RcuReadGuard. Holding multiple guards (i.e., nesting read-side
/// critical sections) is safe.
pub struct RcuReadGuard(());

#[allow(clippy::new_without_default)]
impl RcuReadGuard {
    pub fn new() -> Self {
        unsafe {
            rcu_read_lock_helper();
        }
        RcuReadGuard(())
    }
}

impl Drop for RcuReadGuard {
    fn drop(&mut self) {
        unsafe {
            rcu_read_unlock_helper();
        }
    }
}
