//! An example of using declarative macros and unsafe Rust FFI from
//! chapters 19.1 and 19.5.
//!
//! Read from https://dev.to/dandyvica/how-to-call-rust-functions-from-c-on-linux-h37
//!
//! 
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::slice;

// A Rust struct mapping the C struct
#[repr(C)]
#[derive(Debug)]
pub struct RustStruct {
    pub c: char,
    pub ul: u64,
    pub c_string: *const c_char,
}

/// This macro takes an argument of designator `ident` and creates a function
/// named `$func_name`.
///
/// The `ident` designator is used for variable/function names.
macro_rules! create_function {
    ($func_name:ident, $ctype:ty) => {
        #[no_mangle]
        pub extern "C" fn $func_name(v: $ctype) {
            // The `stringify!` macro converts an `ident` into a string.
            println!(
                "{:?}() is called, value passed = <{:?}>",
                stringify!($func_name),
                v
            );
        }
    };
}

// create simple functions where C type is exactly mapping a Rust type
create_function!(rust_char, char);
create_function!(rust_wchar, char);
create_function!(rust_short, i16);
create_function!(rust_ushort, u16);
create_function!(rust_int, i32);
create_function!(rust_uint, u32);
create_function!(rust_long, i64);
create_function!(rust_ulong, u64);
create_function!(rust_void, *mut c_void);