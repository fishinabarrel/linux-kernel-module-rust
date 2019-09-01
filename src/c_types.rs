#![allow(non_camel_case_types)]

#[cfg(target_arch = "x86_64")]
mod c {
    use core::ffi;

    pub type c_int = i32;
    pub type c_char = i8;
    pub type c_long = i64;
    pub type c_longlong = i64;
    pub type c_short = i16;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ulong = u64;
    pub type c_ulonglong = u64;
    pub type c_ushort = u16;
    pub type c_schar = i8;
    pub type c_size_t = usize;
    pub type c_ssize_t = isize;
    pub type c_void = ffi::c_void;
}

pub use c::*;
