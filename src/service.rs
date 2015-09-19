use ffi::*;
use record::*;
use callback::{SafeDomainEnumReply};
use std::option::Option;
use std::ops::Drop;
use std::mem::uninitialized;
use libc::{uint16_t, c_void};

pub struct DNSService {
    pub ptr : DNSServiceRef,
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

    pub fn enumerate_domains <T> (&mut self,
                                  flags           : DNSServiceFlags,
                                  interface_index : u32,
                                  callback_struct : Option<SafeDomainEnumReply<T>>) -> DNSServiceErrorType {
        let context = &callback_struct as *const _ as *mut c_void;

        unsafe { DNSServiceEnumerateDomains (&mut self.ptr, flags, interface_index, Some(SafeDomainEnumReply::<T>::wrapper), context) }
    }

    /*pub fn register (&mut self,
                     flags           : DNSServiceFlags,
                     interface_index : u32,
                     name            : &str,
                     regtype         : &str,
                     domain          : &str,
                     port            : u16,
                     txtLen          : u16,
                     )*/

    pub fn add_record <T> (&self,
                           record       : &mut DNSRecord,
                           flags        : DNSServiceFlags,
                           service_type : DNSServiceType,
                           length       : usize,
                           data         : &T,
                           ttl          : u32) -> DNSServiceErrorType {
        unsafe { DNSServiceAddRecord (self.ptr, &mut record.ptr, flags, service_type as uint16_t, length as uint16_t, data as *const _ as*const c_void, ttl) }
    }

    pub fn update_record <T> (&self,
                              record : &DNSRecord,
                              flags  : DNSServiceFlags,
                              length : usize,
                              data   : &T,
                              ttl    : u32) -> DNSServiceErrorType {
        unsafe { DNSServiceUpdateRecord (self.ptr, record.ptr, flags, length as uint16_t, data as *const _ as *const c_void, ttl) }
    }
}

impl Drop for DNSService {
    fn drop (&mut self) {
        unsafe { DNSServiceRefDeallocate(self.ptr) };
    }
}
