use utils::const_c_to_string;
use service::DNSService;
use ffi::{DNSServiceRef, DNSServiceFlags, DNSServiceErrorType};
use libc::{c_void, c_char, uint32_t};

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

pub struct SafeDomainEnumReply <T> {
    pub callback : SafeDomainEnumReplyCallback,
    pub content : T,
}

pub struct SafeRegisterReply <T> {
    pub callback : SafeRegisterReplyCallback,
    pub content : T,
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

                    (callback_struct.callback) (safe_ref, flags, interface_index, error_code, &safe_reply_domain);
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

                    (callback_struct.callback) (safe_ref, flags, error_code, &safe_name, &safe_regtype, &safe_domain);
                },
            }
        }
    }
}
