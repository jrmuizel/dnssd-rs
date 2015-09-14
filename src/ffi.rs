use libc::{uint8_t, uint16_t, int32_t, uint32_t, c_void, c_char, c_uchar};
use std::mem::{transmute, zeroed};
use std::option::Option;
use std::clone::Clone;
use std::default::Default;

pub enum StructDNSServiceRef {}
pub type DNSServiceRef = *mut StructDNSServiceRef;

pub enum StructDNSRecordRef {}
pub type DNSRecordRef = *mut StructDNSRecordRef;

#[repr(C)]
#[derive(Copy)]
pub struct UnionTXTRecordRef {
    pub _bindgen_data_ : [u64; 2usize],
}
impl UnionTXTRecordRef {
    pub unsafe fn PrivateData(&mut self) -> *mut [::libc::c_char; 16usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn ForceNaturalAlignment(&mut self) -> *mut *mut ::libc::c_char {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}
impl Clone for UnionTXTRecordRef {
    fn clone(&self) -> Self { *self }
}
impl Default for UnionTXTRecordRef {
    fn default() -> Self { unsafe { zeroed() } }
}
pub type TXTRecordRef = UnionTXTRecordRef;

#[repr(C)]
pub enum DNSServiceFlags {
    MoreComing         = 0x1,
    Add                = 0x2,
    Default            = 0x4,
    NoAutoRename       = 0x8,
    Shared             = 0x10,
    Unique             = 0x20,
    BrowseDomains      = 0x40,
    RegistrationDomains = 0x80,
    LongLivedQuery     = 0x100,
    AllowRemoteQuery   = 0x200,
    ForceMulticast     = 0x400,
    ReturnCNAME        = 0x800,
}

pub enum DNSServiceClass {
    IN = 1,
}

#[repr(C)]
pub enum DNSServiceType {
    A        = 1,
    NS       = 2,
    MD       = 3,
    MF       = 4,
    CNAME    = 5,
    SOA      = 6,
    MB       = 7,
    MG       = 8,
    MR       = 9,
    NULL     = 10,
    WKS      = 11,
    PTR      = 12,
    HINFO    = 13,
    MINFO    = 14,
    MX       = 15,
    TXT      = 16,
    RP       = 17,
    AFSDB    = 18,
    X25      = 19,
    ISDN     = 20,
    RT       = 21,
    NSAP     = 22,
    NSAPPTR = 23,
    SIG      = 24,
    KEY      = 25,
    PX       = 26,
    GPOS     = 27,
    AAAA     = 28,
    LOC      = 29,
    NXT      = 30,
    EID      = 31,
    NIMLOC   = 32,
    SRV      = 33,
    ATMA     = 34,
    NAPTR    = 35,
    KX       = 36,
    CERT     = 37,
    A6       = 38,
    DNAME    = 39,
    SINK     = 40,
    OPT      = 41,
    TKEY     = 249,
    TSIG     = 250,
    IXFR     = 251,
    AXFR     = 252,
    MAILAB   = 253,
    MAILA    = 254,
    ANY      = 255,
}

#[repr(C)]
pub enum DNSServiceErrorType {
    NoError             = 0,
    Unknown             = -65537,
    NoSuchName          = -65538,
    NoMemory            = -65539,
    BadParam            = -65540,
    BadReference        = -65541,
    BadState            = -65542,
    BadFlags            = -65543,
    Unsupported         = -65544,
    NotInitialized      = -65545,
    AlreadyRegistered   = -65547,
    NameConflict        = -65548,
    Invalid             = -65549,
    Firewall            = -65550,
    Incompatible        = -65551,
    BadInterfaceIndex   = -65552,
    Refused             = -65553,
    NoSuchRecord        = -65554,
    NoAuth              = -65555,
    NoSuchKey           = -65556,
    NATTraversal        = -65557,
    DoubleNAT           = -65558,
    BadTime             = -65559,
}

pub const DNS_SERVICE_MAX_SERVICE_NAME : usize = 64;

pub const DNS_SERVICE_MAX_DOMAIN_NAME : usize = 1005;

pub const DNS_SERVICE_INTERFACE_INDEX_ANY : usize = 0;

pub const DNS_SERVICE_INTERFACE_INDEX_LOCAL_ONLY : u32 = ::std::u32::MAX -1;

pub type DNSServiceDomainEnumReply = Option<extern fn (DNSServiceRef,
                                                       DNSServiceFlags,
                                                       uint32_t,
                                                       DNSServiceErrorType,
                                                       *const c_char,
                                                       *mut c_void)>;

pub type DNSServiceRegisterReply = Option<extern fn (DNSServiceRef,
                                                     DNSServiceFlags,
                                                     DNSServiceErrorType,
                                                     *const c_char,
                                                     *const c_char,
                                                     *const c_char,
                                                     *mut c_void)>;

pub type DNSServiceBrowseReply = Option<extern fn (DNSServiceRef,
                                                   DNSServiceFlags,
                                                   uint32_t,
                                                   DNSServiceErrorType,
                                                   *const c_char,
                                                   *const c_char,
                                                   *const c_char,
                                                   *mut c_void)>;

pub type DNSServiceResolveReply = Option<extern fn (DNSServiceRef,
                                                    DNSServiceFlags,
                                                    uint32_t,
                                                    DNSServiceErrorType,
                                                    *const c_char,
                                                    *const c_char,
                                                    uint16_t,
                                                    uint16_t,
                                                    *const c_uchar,
                                                    *mut c_void)>;

pub type DNSServiceRegisterRecordReply = Option<extern fn (DNSServiceRef,
                                                           DNSRecordRef,
                                                           DNSServiceFlags,
                                                           DNSServiceErrorType,
                                                           *mut c_void)>;

pub type DNSServiceQueryRecordReply = Option<extern fn (DNSServiceRef,
                                                        DNSServiceFlags,
                                                        uint32_t,
                                                        DNSServiceErrorType,
                                                        *const c_char,
                                                        uint16_t,
                                                        uint16_t,
                                                        uint16_t,
                                                        *const c_void,
                                                        uint32_t,
                                                        *mut c_void)>;

#[link(name = "dns_sd")]
extern {
    pub fn DNSServiceRefSockFD (sdRef : DNSServiceRef) -> int32_t;

    pub fn DNSServiceProcessResult (sdRef : DNSServiceRef) -> DNSServiceErrorType;

    pub fn DNSServiceRefDeallocate (sdRef : DNSServiceRef);

    pub fn DNSServiceEnumerateDomains (sdRef          : *mut DNSServiceRef,
                                       flags          : DNSServiceFlags,
                                       interfaceIndex : uint32_t,
                                       callBack       : DNSServiceDomainEnumReply,
                                       context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceRegister (sdRef          : *mut DNSServiceRef,
                               flags          : DNSServiceFlags,
                               interfaceIndex : uint32_t,
                               name           : *const c_char,
                               regtype        : *const c_char,
                               domain         : *const c_char,
                               host           : *const c_char,
                               port           : uint16_t,
                               txtLen         : uint16_t,
                               txtRecord      : *const c_void,
                               callback       : DNSServiceRegisterReply,
                               context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceAddRecord (sdRef     : DNSServiceRef,
                                RecordRef : *mut DNSRecordRef,
                                flags     : DNSServiceFlags,
                                rrtype    : uint16_t,
                                rdlen     : uint16_t,
                                rdata     : *const c_void,
                                ttl       : uint32_t) -> DNSServiceErrorType;

    pub fn DNSServiceUpdateRecord (sdRef     : DNSServiceRef,
                                   RecordRef : DNSRecordRef,
                                   flags     : DNSServiceFlags,
                                   rdlen     : uint16_t,
                                   rdata     : *const c_void,
                                   ttl       : uint32_t) -> DNSServiceErrorType;

    pub fn DNSServiceRemoveRecord (sdRef     : DNSServiceRef,
                                   RecordRef : *mut DNSRecordRef,
                                   flags     : DNSServiceFlags) -> DNSServiceErrorType;

    pub fn DNSServiceBrowse (sdRef          : *mut DNSServiceRef,
                             flags          : DNSServiceFlags,
                             interfaceIndex : uint32_t,
                             regtype        : *const c_char,
                             domain         : *const c_char,
                             callback       : DNSServiceBrowseReply,
                             context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceResolve (sdRef          : *mut DNSServiceRef,
                              flags          : DNSServiceFlags,
                              interfaceIndex : uint32_t,
                              name           : *const c_char,
                              regtype        : *const c_char,
                              domain         : *const c_char,
                              callback       : DNSServiceResolveReply,
                              context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceCreateConnection (sdRef : *mut DNSServiceRef) -> DNSServiceErrorType;

    pub fn DNSServiceRegisterRecord (sdRef          : DNSServiceRef,
                                     RecordRef      : *mut DNSRecordRef,
                                     flags          : DNSServiceFlags,
                                     interfaceIndex : uint32_t,
                                     fullname       : *const c_char,
                                     rrtype         :  uint16_t,
                                     rrclass        : uint16_t,
                                     rdlen          : uint16_t,
                                     rdata          : *const c_void,
                                     ttl            : uint32_t,
                                     callBack       : DNSServiceRegisterRecordReply,
                                     context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceQueryRecord (sdRef          : *mut DNSServiceRef,
                                  flags          : DNSServiceFlags,
                                  interfaceIndex : uint32_t,
                                  fullname       : *const c_char,
                                  rrtype         :  uint16_t,
                                  rrclass        : uint16_t,
                                  callBack       : DNSServiceQueryRecordReply,
                                  context        : *mut c_void) -> DNSServiceErrorType;

    pub fn DNSServiceReconfirmRecord (flags          : DNSServiceFlags,
                                      interfaceIndex : uint32_t,
                                      fullname       : *const c_char,
                                      rrtype         :  uint16_t,
                                      rrclass        : uint16_t,
                                      rdlen          : uint16_t,
                                      rdata          : *const c_void) -> DNSServiceErrorType;

    pub fn DNSServiceConstructFullName (fullname : *mut c_char,
                                        service  : *const c_char,
                                        regtype  : *const c_char,
                                        domain   : *const c_char) -> int32_t;

    pub fn TXTRecordCreate (txtRecord : *mut TXTRecordRef,
                            bufferLen : uint16_t,
                            buffer    : *mut c_void);

    pub fn TXTRecordDeallocate (txtRecord : *mut TXTRecordRef);

    pub fn TXTRecordSetValue (txtRecord : *mut TXTRecordRef,
                              key       : *const c_char,
                              valueSize : uint8_t,
                              value     : *const c_void) -> DNSServiceErrorType;

    pub fn TXTRecordRemoveValue (txtRecord : *mut TXTRecordRef,
                                 key       : *const c_char) -> DNSServiceErrorType;

    pub fn TXTRecordGetLength (txtRecord : *const TXTRecordRef) -> uint16_t;

    pub fn TXTRecordGetBytesPtr (txtRecord : *const TXTRecordRef) -> *const c_void;

    pub fn TXTRecordContainsKey (txtLen    : uint16_t,
                                 txtRecord : *const c_void,
                                 key       : *const c_char) -> int32_t;

    pub fn TXTRecordGetValuePtr (txtLen    : uint16_t,
                                 txtRecord : *const c_void,
                                 key       : *const c_char,
                                 valueLen  : uint8_t) -> *const c_void;

    pub fn TXTRecordGetCount (txtLen    : uint16_t,
                              txtRecord : *const c_void,) -> uint16_t;

    pub fn TXTRecordGetItemAtIndex (txtLen    : uint16_t,
                                    txtRecord : *const c_void,
                                    index     : uint16_t,
                                    keyBufLen : uint16_t,
                                    key       : *mut c_char,
                                    valueLen  : uint8_t,
                                    value     : *const (*const c_void)) -> DNSServiceErrorType;
}
