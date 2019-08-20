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
    pub fn new(data: &str) -> &CStr {
        if data.bytes().position(|b| b == b'\x00') != Some(data.len() - 1) {
            panic!("CStr must contain a single NUL byte at the end");
        }
        unsafe { &*(data as *const str as *const CStr) }
    }
}

impl Deref for CStr {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {{
        $crate::CStr::new(concat!($str, "\x00"))
    }};
}
