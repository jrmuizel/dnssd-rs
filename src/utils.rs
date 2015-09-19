use std::ffi::{CStr, CString};
use std::mem::transmute;
use libc::c_char;

pub unsafe fn const_c_to_string (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}

pub unsafe fn str_to_const_c (value : &str) -> *const c_char {
    let new_string = value.clone();
    CString::new (new_string).unwrap ().as_ptr ()
}

pub unsafe fn str_to_mut_c (value : &str) -> *mut c_char {
    let mut bytes = String::from (value).into_bytes ();
    bytes.push (b"\0"[0]);

    let mut result = Vec::<i8>::with_capacity(bytes.len());
    for &byte in bytes.iter () {
        result.push (transmute (byte));
    }

    result.as_mut_ptr()
}
