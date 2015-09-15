use ffi::{TXTRecordRef, TXTRecordCreate, TXTRecordDeallocate, TXTRecordSetValue, DNSServiceErrorType};
use std::mem::uninitialized;
use libc::c_void;
use std::ops::Drop;

pub struct TXTRecord {
    pub ptr : TXTRecordRef,
}

impl TXTRecord {
    pub fn new () -> TXTRecord {
        TXTRecord {
            ptr : unsafe { uninitialized () },
        }
    }

    pub fn create (&mut self,
                   buffer_len : u16) {
        //unsafe { TXTRecordCreate (&mut self.ptr, buffer_len, null ())};
    }

    pub fn set_value (&mut self,
                      key        : &str,
                      value_size : u8,
                      value      : Option<T>) -> DNSServiceErrorType
                      where T: <Into<Vec<u8>>> {
        unsafe { TXTRecordSetValue (&mut self.ptr, key, value_size) }
    }
}

impl Drop for TXTRecord {
    fn drop (&mut self) {
        unsafe { TXTRecordDeallocate (&mut self.ptr) };
    }
}
