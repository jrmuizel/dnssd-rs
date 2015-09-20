use ffi::*;
use txtrecord::TXTRecord;
use utils::{option_str_to_const_c, str_to_const_c, if_no_error};
use callback::{SafeDomainEnumReply, SafeRegisterReply, SafeBrowseReply, SafeResolveReply,
    SafeRegisterRecordReply, SafeQueryRecordReply};
use std::option::Option;
use std::ops::Drop;
use std::mem::uninitialized;
use libc::{uint16_t, c_void};

pub struct DNSService {
    pub ptr : DNSServiceRef,
}

impl DNSService {
    fn new () -> DNSService {
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

    pub fn enumerate_domains <T> (flags           : DNSServiceFlags,
                                  interface_index : u32,
                                  callback_struct : Option<SafeDomainEnumReply<T>>)
                                  -> Result<DNSService, DNSServiceErrorType> {
        let mut service = DNSService::new ();
        let context = &callback_struct as *const _ as *mut c_void;

        unsafe {
            let result = DNSServiceEnumerateDomains (&mut service.ptr, flags, interface_index,
            Some (SafeDomainEnumReply::<T>::wrapper), context);

            if_no_error (service, result)
        }
    }

    pub fn register <T> (flags           : DNSServiceFlags,
                         interface_index : u32,
                         name            : Option<&str>,
                         regtype         : &str,
                         domain          : Option<&str>,
                         host            : Option<&str>,
                         port            : u16,
                         txt_record      : TXTRecord,
                         callback_struct : Option<SafeRegisterReply<T>>)
                         -> Result <DNSService, DNSServiceErrorType> {
        unsafe {
            let mut service = DNSService::new ();
            let unsafe_name = option_str_to_const_c (name);
            let unsafe_regtype = str_to_const_c (regtype);
            let unsafe_domain = option_str_to_const_c (domain);
            let unsafe_host = option_str_to_const_c (host);
            let context = &callback_struct as *const _ as *mut c_void;

            let result = DNSServiceRegister (&mut service.ptr, flags, interface_index, unsafe_name,
                unsafe_regtype, unsafe_domain, unsafe_host, port, txt_record.get_length (),
                &txt_record as *const _ as *const c_void, Some(SafeRegisterReply::<T>::wrapper), context);

            if_no_error (service, result)
        }
    }

    pub fn browse <T> (flags           : DNSServiceFlags,
                       interface_index : u32,
                       regtype         : &str,
                       domain          : &str,
                       callback_struct : Option<SafeBrowseReply<T>>)
                       -> Result<DNSService, DNSServiceErrorType> {
        unsafe {
            let mut service = DNSService::new ();
            let unsafe_regtype = str_to_const_c (regtype);
            let unsafe_domain = str_to_const_c (domain);
            let context = &callback_struct as *const _ as *mut c_void;
            let result = DNSServiceBrowse (&mut service.ptr, flags, interface_index, unsafe_regtype,
                unsafe_domain, Some(SafeBrowseReply::<T>::wrapper), context);

            if_no_error (service, result)
        }
    }

    pub fn resolve <T> (flags           : DNSServiceFlags,
                        interface_index : u32,
                        name            : &str,
                        regtype         : &str,
                        domain          : &str,
                        callback_struct : Option<SafeResolveReply<T>>)
                        -> Result<DNSService, DNSServiceErrorType> {
        unsafe {
            let mut service = DNSService::new ();
            let unsafe_name = str_to_const_c (name);
            let unsafe_regtype = str_to_const_c (regtype);
            let unsafe_domain = str_to_const_c (domain);
            let context = &callback_struct as *const _ as *mut c_void;
            let result = DNSServiceResolve (&mut service.ptr, flags, interface_index, unsafe_name,
                unsafe_regtype, unsafe_domain, Some(SafeResolveReply::<T>::wrapper), context);

            if_no_error (service, result)
        }
    }

    pub fn create_connection () -> Result<DNSService, DNSServiceErrorType> {
        unsafe  {
            let mut service = DNSService::new ();
            let result = DNSServiceCreateConnection (&mut service.ptr);

            if_no_error (service, result)
        }
    }

    pub fn add_record <T> (&self,
                           flags        : DNSServiceFlags,
                           service_type : DNSServiceType,
                           data         : T,
                           ttl          : u32)
                           -> Result<DNSRecord ,DNSServiceErrorType>
                           where T : Into<Vec<u8>> {
        unsafe {
            let mut record = DNSRecord::new();
            let bytes = &data.into ();
            let result = DNSServiceAddRecord (self.ptr, &mut record.ptr, flags, service_type,
            bytes.len () as u16, bytes.as_ptr () as *const c_void, ttl);

            if_no_error (record, result)
        }
    }

    pub fn update_record <T> (&self,
                              record : &DNSRecord,
                              flags  : DNSServiceFlags,
                              data   : T,
                              ttl    : u32) -> DNSServiceErrorType
                              where T : Into<Vec<u8>> {
        unsafe {
            let bytes = &data.into ();
            DNSServiceUpdateRecord (self.ptr, record.ptr, flags, bytes.len () as uint16_t,
            bytes.as_ptr () as *const c_void, ttl)
        }
    }

    pub fn remove_record <T> (&self,
                              record : &DNSRecord,
                              flags  : DNSServiceFlags) -> DNSServiceErrorType {
        unsafe { DNSServiceRemoveRecord (self.ptr, record.ptr, flags) }
    }

    pub fn register_record <S, T> (&self,
                                   flags           : DNSServiceFlags,
                                   interface_index : u32,
                                   fullname        : &str,
                                   rrtype          : DNSServiceType,
                                   rrclass         : DNSServiceClass,
                                   data           : T,
                                   ttl             : u32,
                                   callback_struct : Option<SafeRegisterRecordReply<S>>)
                                   -> Result<DNSRecord, DNSServiceErrorType>
                                   where T: Into<Vec<u8>> {
        let mut record = DNSRecord::new ();
        let context = &callback_struct as *const _ as *mut c_void;
        let unsafe_fullname = str_to_const_c (fullname);
        let bytes = &data.into ();
        unsafe {
            let result = DNSServiceRegisterRecord (self.ptr, &mut record.ptr, flags, interface_index,
                unsafe_fullname, rrtype, rrclass, bytes.len () as u16, bytes.as_ptr () as *const c_void,
                ttl, Some(SafeRegisterRecordReply::<T>::wrapper), context);

            if_no_error (record, result)
        }
    }

    pub fn query_record <T> (flags           : DNSServiceFlags,
                             interface_index : u32,
                             fullname        : &str,
                             rrtype          : DNSServiceType,
                             rrclass         : DNSServiceClass,
                             callback_struct : Option<SafeQueryRecordReply<T>>)
                             -> Result<DNSService, DNSServiceErrorType> {
        let mut service = DNSService::new ();
        let context = &callback_struct as *const _ as *mut c_void;
        let unsafe_fullname = str_to_const_c (fullname);
        unsafe {
            let result = DNSServiceQueryRecord (&mut service.ptr, flags, interface_index, unsafe_fullname,
                rrtype, rrclass, Some(SafeQueryRecordReply::<T>::wrapper), context);

            if_no_error (service, result)
        }
    }

    pub fn reconfirm_record <T> (flags           : DNSServiceFlags,
                                 interface_index : u32,
                                 fullname        : &str,
                                 rrtype          : DNSServiceType,
                                 rrclass         : DNSServiceClass,
                                 data            : T) -> DNSServiceErrorType
                                 where T : Into<Vec<u8>> {
        let unsafe_fullname = str_to_const_c (fullname);
        let bytes = &data.into ();
        unsafe {
            DNSServiceReconfirmRecord (flags, interface_index, unsafe_fullname, rrtype, rrclass,
                bytes.len () as u16, bytes.as_ptr () as *const c_void)
        }
    }
}

impl Drop for DNSService {
    fn drop (&mut self) {
        unsafe { DNSServiceRefDeallocate(self.ptr) };
    }
}

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
