use utils::{str_to_const_c, mut_c_to_string};
use libc::{malloc, free, c_char, c_void};
use ffi::{DNS_SERVICE_MAX_DOMAIN_NAME, DNSServiceConstructFullName};

pub fn dns_service_construct_fullname (service : &str,
                                       regtype : &str,
                                       domain  : &str) -> Option<String> {
    unsafe {
        let buffer = malloc (DNS_SERVICE_MAX_DOMAIN_NAME) as *mut c_char;
        let unsafe_service = str_to_const_c (service);
        let unsafe_regtype = str_to_const_c (regtype);
        let unsafe_domain = str_to_const_c (domain);

        match DNSServiceConstructFullName (buffer, unsafe_service, unsafe_regtype, unsafe_domain) {
            0 => {
                let fullname = mut_c_to_string (buffer);
                free (buffer as *mut c_void);
                Some (fullname)
            },
            _ => None,
        }
    }
}
