use ffi::*;
use std::mem::uninitialized;

pub struct DNSRecord {
    pub ptr : DNSRecordRef,
}

impl DNSRecord {
    pub fn new () -> DNSRecord {
        DNSRecord {
            ptr : unsafe { uninitialized () },
        }
    }
}
