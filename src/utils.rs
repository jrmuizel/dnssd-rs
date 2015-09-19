use std::ffi::{CStr, CString}
use libc::c_char;

pub unsafe fn const_c_to_string (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}

pub unsafe fn str_to_const_c (value : &str) -> *const c_char {
    let new_string = value.clone();
    CString::new (new_string).unwrap ().as_ptr ()
}

pub unsafe fn mut_c_to_str (ptr : *mut c_char) -> String {
    let mut result = Vec::<u8>::new();

    let i = 0;
    while *ptr.offset (i) != (b"\0"[0] as c_char) {
        result.push (*ptr.offset (i) as u8);
    }

    String::from_utf8 (result).unwrap ()
}
