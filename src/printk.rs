use core::fmt;

use types::c_int;

pub struct KernelConsole;

extern "C" {
    fn printk_info_helper(s: *const u8, len: c_int) -> c_int;
    fn printk_cont_helper(s: *const u8, len: c_int) -> c_int;
}

fn printk_info(s: &str) {
    // TODO: I believe printk never fails
    unsafe { printk_info_helper(s.as_ptr(), s.len() as c_int) };
}

fn printk_cont(s: &str) {
    // TODO: I believe printk never fails
    unsafe { printk_cont_helper(s.as_ptr(), s.len() as c_int) };
}

impl fmt::Write for KernelConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        printk_cont(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::printk::printk_info("\n");
    });
    ($fmt:expr) => ({
        $crate::printk::printk_info(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use ::core::fmt::Write;
        $crate::printk::printk_info("");
        let _ = $crate::printk::KernelConsole.write_fmt(format_args!(concat!($fmt, "\n"), $($args)*);
    });
}
