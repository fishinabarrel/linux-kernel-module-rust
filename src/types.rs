use core::ops::Deref;

use crate::bindings;

pub struct Mode(bindings::umode_t);

impl Mode {
    pub fn from_int(m: u16) -> Mode {
        Mode(m)
    }

    pub fn as_int(&self) -> u16 {
        self.0
    }
}

#[repr(transparent)]
pub struct CStr(str);

impl CStr {
    /// Creates a new CStr from a str without performing any additional checks. `data` _must_ end
    /// with a NUL byte, and should only have only a single NUL byte, or the string will be
    /// truncated.
    pub const unsafe fn new_unchecked(data: &str) -> &CStr {
        &*(data as *const str as *const CStr)
    }
}

impl Deref for CStr {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Creates a new `CStr` from a string literal. The string literal should not contain any NUL
/// bytes. Example usage:
/// ```
/// const MY_CSTR: &CStr = cstr!("My awesome CStr!");
/// ```
#[macro_export]
macro_rules! cstr {
    ($str:expr) => {{
        let s = concat!($str, "\x00");
        unsafe { $crate::CStr::new_unchecked(s) }
    }};
}
