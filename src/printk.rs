use core::fmt;

use types::c_int;

pub struct KernelConsole;

extern "C" {
    fn printk_helper(s: *const u8, len: c_int) -> c_int;
}

impl fmt::Write for KernelConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO: I believe printk never fails
        unsafe { printk_helper(s.as_ptr(), s.len() as c_int) };
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => ({
        use ::core::fmt::Write;
        let _ = $crate::printk::KernelConsole.write_str("");
    });
    ($fmt:expr) => ({
        use ::core::fmt::Write;
        let _ = $crate::printk::KernelConsole.write_str($fmt);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use ::core::fmt::Write;
        let _ = $crate::printk::KernelConsole.write_fmt(format_args!($fmt, $($arg)*));
    });
}
