use ffi::*;
use error::*;
use context::*;
use callback::{SafeDomainEnumReplyCallback};
use std::option::Option;
use std::ops::Drop;
use std::mem::uninitialized;
use std::ptr::null;

pub struct DNSService {
    ptr : DNSServiceRef,
}

impl DNSService {
    pub fn new () -> DNSService {
        DNSService {
            ptr : unsafe { uninitialized () },
        }
    }

    pub fn socket_file_descriptor (&self) -> Option<isize> {
        let fd = unsafe { DNSServiceRefSockFD (self.ptr) };

        if fd == -1 {
            None
        } else {
            Some(fd as isize)
        }
    }

    pub fn process_result (&self) -> DNSServiceErrorType {
        unsafe { DNSServiceProcessResult (self.ptr) }
    }
}

impl Drop for DNSService {
    fn drop (&mut self) {
        unsafe { DNSServiceRefDeallocate(self.ptr) };
    }
}
