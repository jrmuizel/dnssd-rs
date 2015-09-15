use std::ffi::{CStr, CString};
use libc::c_char;

pub unsafe fn c_to_str (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}

pub unsafe fn str_to_c (value : &str) -> *const c_char {
    let new_string = value.clone();
    CString::new (new_string).unwrap ().as_ptr ()
}
