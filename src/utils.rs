use std::ffi::CStr;
use libc::c_char;

pub unsafe fn c_to_str (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}
