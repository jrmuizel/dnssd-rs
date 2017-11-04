use ffi::DNSServiceErrorType;
use std::ffi::{CStr, CString};
use std::ptr::null;
use libc::{c_char, c_uchar, c_void};

pub unsafe fn const_c_to_string (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}

pub fn str_to_const_c (value : &str) -> CString {
    let new_string = value.clone();
    CString::new (new_string).unwrap ()
}

pub unsafe fn mut_c_to_string (ptr : *mut c_char) -> String {
    let mut result = Vec::<u8>::new();

    let i = 0;
    while *ptr.offset (i) != (b"\0"[0] as c_char) {
        result.push (*ptr.offset (i) as u8);
    }

    String::from_utf8 (result).unwrap ()
}

pub fn option_str_to_const_c (wrapper : Option<&str>) -> Option<CString> {
    match wrapper {
        None => None,
        Some (value) => Some(str_to_const_c (value)),
    }
}

pub fn option_cstr_to_const_c(wrapper : &Option<CString>) -> *const c_char {
    match wrapper {
        &None => null(),
        &Some(ref value) => value.as_ref().as_ptr()
    }
}

pub fn if_no_error <T> (value : T, result : DNSServiceErrorType) -> Result<T, DNSServiceErrorType> {
    match result {
        DNSServiceErrorType::NoError => Ok(value),
        error @ _ => Err (error),
    }
}

pub unsafe fn const_c_void_to_vec (data : *const c_void, len : usize) -> Vec<u8> {
    let mut result = Vec::<u8>::new ();
    let data = data as *const c_uchar;

    for i in 0..len {
        result.push (*data.offset (i as isize))
    }

    result
}
