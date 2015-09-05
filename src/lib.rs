extern crate libc;

mod ffi;

use ffi::*;

pub struct DNSService {
    ptr : *mut ffi::DNSServiceRef,
}
