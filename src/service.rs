use ffi::*;
use txtrecord::TXTRecord;
use utils::{option_str_to_const_c, str_to_const_c};
use dnsrecord::DNSRecord;
use callback::{SafeDomainEnumReply, SafeRegisterReply, SafeBrowseReply, SafeResolveReply};
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
            match result {
                DNSServiceErrorType::NoError => Ok (service),
                error @ _ => Err (error),
            }
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

            match result {
                DNSServiceErrorType::NoError => Ok (service),
                error @ _ => Err (error),
            }
        }
    }

    pub fn add_record <T> (&self,
                           record       : &mut DNSRecord,
                           flags        : DNSServiceFlags,
                           service_type : DNSServiceType,
                           data         : T,
                           ttl          : u32) -> DNSServiceErrorType
                           where T : Into<Vec<u8>> {
        unsafe {
            let bytes = &data.into ();
            DNSServiceAddRecord (self.ptr, &mut record.ptr, flags, service_type as uint16_t,
            bytes.len () as u16, bytes as *const _ as*const c_void, ttl)
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
            bytes as *const _ as *const c_void, ttl)
        }
    }

    pub fn remove_record <T> (&self,
                              record : &DNSRecord,
                              flags  : DNSServiceFlags) -> DNSServiceErrorType {
        unsafe { DNSServiceRemoveRecord (self.ptr, record.ptr, flags) }
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

            match result {
                DNSServiceErrorType::NoError => Ok(service),
                error @ _ => Err (error),
            }
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

            match result {
                DNSServiceErrorType::NoError => Ok(service),
                error @ _ => Err (error),
            }
        }
    }

    pub fn create_connection () -> Result<DNSService, DNSServiceErrorType> {
        unsafe  {
            let mut service = DNSService::new ();
            let result = DNSServiceCreateConnection (&mut service.ptr);
            match result {
                DNSServiceErrorType::NoError => Ok(service),
                error @ _ => Err (error),
            }
        }
    }

    /*pub fn register_record (&self,
                            record_ref      : DNSRecord,
                            flags           : DNSServiceFlags,
                            interface_index : u32,
                            fullname        : &str,
                            rrtype          : DNSServiceType,
                            rrclass         : DNSServiceClass,
                            rdlen           : u16,
                            rdata           : )*/
}

impl Drop for DNSService {
    fn drop (&mut self) {
        unsafe { DNSServiceRefDeallocate(self.ptr) };
    }
}
