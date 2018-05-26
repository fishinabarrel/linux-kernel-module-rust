#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, improper_ctypes)]

use c_types;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const GFP_KERNEL: gfp_t = BINDINGS_GFP_KERNEL;
