#![allow(non_camel_case_types)]

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
// See explanation in rust/src/libstd/os/raw.rs
#[repr(u8)]
pub enum c_void {
    #[doc(hidden)]
    __nothing_to_see_here,
    #[doc(hidden)]
    __move_along,
}
