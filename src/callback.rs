use utils::const_c_to_string;
use dnsrecord::DNSRecord;
use txtrecord::TXTRecordData;
use service::DNSService;
use ffi::{DNSServiceRef, DNSServiceFlags, DNSServiceErrorType, DNSRecordRef};
use libc::{c_void, c_char, uint32_t, c_uchar};

pub type SafeDomainEnumReplyCallback = Box<Fn(DNSService,
                                              DNSServiceFlags,
                                              u32,
                                              DNSServiceErrorType,
                                              &str) + 'static>;

pub type SafeRegisterReplyCallback = Box<Fn(DNSService,
                                            DNSServiceFlags,
                                            DNSServiceErrorType,
                                            &str,
                                            &str,
                                            &str) + 'static>;

pub type SafeBrowseReplyCallback = Box<Fn(DNSService,
                                          DNSServiceFlags,
                                          u32,
                                          DNSServiceErrorType,
                                          &str,
                                          &str,
                                          &str) + 'static>;

pub type SafeResolveReplyCallback = Box<Fn(DNSService,
                                           DNSServiceFlags,
                                           u32,
                                           DNSServiceErrorType,
                                           &str,
                                           &str,
                                           u16,
                                           TXTRecordData) + 'static>;

pub type SafeRegisterRecordReplyCallback = Box<Fn(DNSService,
                                                  DNSRecord,
                                                  DNSServiceFlags,
                                                  DNSServiceErrorType) + 'static>;

/*pub type SafeQueryRecordReplyCallback = Box<Fn(DNSService,
                                               DNSServiceFlags,
                                               u32,
                                               DNSServiceErrorType,
                                               &str,
                                               ) + 'static>*/

pub struct SafeDomainEnumReply <T> {
    pub callback : SafeDomainEnumReplyCallback,
    pub content  : T,
}

pub struct SafeRegisterReply <T> {
    pub callback : SafeRegisterReplyCallback,
    pub content  : T,
}

pub struct SafeBrowseReply <T> {
    pub callback : SafeBrowseReplyCallback,
    pub content  : T,
}

pub struct SafeResolveReply <T> {
    pub callback : SafeResolveReplyCallback,
    pub content  : T,
}

pub struct SafeRegisterRecordReply <T> {
    pub callback : SafeRegisterRecordReplyCallback,
    pub content  : T,
}

impl <T> SafeDomainEnumReply <T> {
    pub extern fn wrapper (service_ref     : DNSServiceRef,
                           flags           : DNSServiceFlags,
                           interface_index : uint32_t,
                           error_code      : DNSServiceErrorType,
                           reply_domain    : *const c_char,
                           context         : *mut c_void) {
        let context = context as *mut Option<SafeDomainEnumReply<T>>;
        unsafe {
            match *context {
                None => {},
                Some(ref callback_struct) => {
                    let safe_ref = DNSService { ptr: service_ref};
                    let safe_reply_domain = const_c_to_string (reply_domain);

                    (callback_struct.callback) (safe_ref, flags, interface_index, error_code,
                        &safe_reply_domain);
                },
            }
        }
    }
}

impl <T> SafeRegisterReply <T> {
    pub extern fn wrapper (service_ref     : DNSServiceRef,
                           flags           : DNSServiceFlags,
                           error_code      : DNSServiceErrorType,
                           name            : *const c_char,
                           regtype         : *const c_char,
                           domain          : *const c_char,
                           context         : *mut c_void) {
        let context = context as *mut Option<SafeRegisterReply<T>>;
        unsafe {
            match *context {
                None => {},
                Some(ref callback_struct) => {
                    let safe_ref = DNSService { ptr: service_ref};
                    let safe_name = const_c_to_string (name);
                    let safe_regtype = const_c_to_string (regtype);
                    let safe_domain = const_c_to_string (domain);

                    (callback_struct.callback) (safe_ref, flags, error_code, &safe_name,
                        &safe_regtype, &safe_domain);
                },
            }
        }
    }
}

impl <T> SafeBrowseReply <T> {
    pub extern fn wrapper (service_ref     : DNSServiceRef,
                           flags           : DNSServiceFlags,
                           interface_index : uint32_t,
                           error_code      : DNSServiceErrorType,
                           service_name    : *const c_char,
                           regtype         : *const c_char,
                           domain          : *const c_char,
                           context         : *mut c_void) {
        let context = context as *mut Option<SafeBrowseReply<T>>;
        unsafe {
            match *context {
                None => {},
                Some(ref callback_struct) => {
                    let safe_ref = DNSService { ptr: service_ref};
                    let safe_name = const_c_to_string (service_name);
                    let safe_regtype = const_c_to_string (regtype);
                    let safe_domain = const_c_to_string (domain);

                    (callback_struct.callback) (safe_ref, flags, interface_index, error_code,
                        &safe_name, &safe_regtype, &safe_domain);
                },
            }
        }
    }
}

impl <T> SafeResolveReply <T> {
    pub extern fn wrapper (service_ref     : DNSServiceRef,
                           flags           : DNSServiceFlags,
                           interface_index : u32,
                           error_code      : DNSServiceErrorType,
                           fullname        : *const c_char,
                           hosttarget      : *const c_char,
                           port            : u16,
                           txt_len         : u16,
                           txt_record      : *const c_uchar,
                           context         : *mut c_void) {
        let context = context as *mut Option<SafeResolveReply<T>>;
        unsafe {
            match *context {
                None => {},
                Some (ref callback_struct) => {
                    let safe_ref = DNSService { ptr: service_ref };
                    let safe_fullname = const_c_to_string (fullname);
                    let safe_hosttarget = const_c_to_string (hosttarget);
                    let safe_txtrecord = TXTRecordData { ptr: txt_record as *const c_void, len: txt_len };

                    (callback_struct.callback) (safe_ref, flags, interface_index, error_code,
                        &safe_fullname, &safe_hosttarget, port, safe_txtrecord);
                },
            }
        }
    }
}

impl <T> SafeRegisterRecordReply <T> {
    pub extern fn wrapper (service_ref     : DNSServiceRef,
                           record_ref      : DNSRecordRef,
                           flags           : DNSServiceFlags,
                           error_code      : DNSServiceErrorType,
                           context         : *mut c_void) {
        let context = context as *mut Option<SafeRegisterRecordReply<T>>;
        unsafe {
            match *context {
                None => {},
                Some (ref callback_struct) => {
                    let safe_service = DNSService { ptr: service_ref };
                    let safe_record = DNSRecord { ptr: record_ref };
                    (callback_struct.callback) (safe_service, safe_record, flags, error_code);
                }
            }
        }
    }
}
