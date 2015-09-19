use ffi::{TXTRecordRef, TXTRecordCreate, TXTRecordDeallocate, TXTRecordSetValue, TXTRecordRemoveValue,
    TXTRecordGetLength, TXTRecordGetBytesPtr, TXTRecordContainsKey, TXTRecordGetValuePtr,
    TXTRecordGetCount, TXTRecordGetItemAtIndex, DNSServiceErrorType};
use std::mem::uninitialized;
use libc::{uint8_t, c_void, c_char, malloc, free};
use std::cmp::min;
use std::ops::Drop;
use std::ptr::{null, null_mut};
use utils::{str_to_const_c, mut_c_to_str};

pub struct TXTRecord {
    pub ptr    : TXTRecordRef,
    pub buffer : Option<Vec<u8>>,
}

pub struct TXTRecordItem <'a, T: 'a> {
    pub key       : String,
    pub value_len : u8,
    pub value     : &'a T,
}

impl TXTRecord {
    pub fn new () -> TXTRecord {
        let mut ret = TXTRecord {
            ptr    : unsafe { uninitialized () },
            buffer : None,
        };

        unsafe { TXTRecordCreate (&mut ret.ptr, 0, null_mut ()) };

        ret
    }

    pub fn new_with_buffer (size : u16) -> TXTRecord {
        let mut ret = TXTRecord {
            ptr    : unsafe { uninitialized () },
            buffer : Some (vec![0; size as usize]),
        };

        unsafe { TXTRecordCreate (&mut ret.ptr, size, &mut ret.buffer as *mut _ as *mut c_void) };

        ret
    }

    pub fn set_value <T> (&mut self,
                          key           : &str,
                          value_wrapper : Option<T>) -> DNSServiceErrorType
                          where T : Into<Vec<u8>> {
        unsafe {
            let new_key = str_to_const_c (key);
            match value_wrapper {
                None => TXTRecordSetValue (&mut self.ptr, new_key, 0, null ()),
                Some (value) => {
                    let value_array = value.into ();
                    TXTRecordSetValue (&mut self.ptr, new_key, value_array.len () as u8, value_array.as_ptr () as *const c_void)
                },
            }
        }
    }

    pub fn remove_value (&mut self,
                         key : &str) -> DNSServiceErrorType {
        unsafe { TXTRecordRemoveValue (&mut self.ptr, str_to_const_c (key)) }
    }

    pub fn get_length (&self) -> u16 {
        unsafe { TXTRecordGetLength (&self.ptr) }
    }

    pub fn get_bytes_ptr (&self) -> *const c_void {
        unsafe { TXTRecordGetBytesPtr (&self.ptr) }
    }

    pub fn contains_key (&self,
                         key : &str) -> bool {
        let result = unsafe { TXTRecordContainsKey (self.get_length (), self.get_bytes_ptr (), str_to_const_c (key)) };
        match result {
            1 => true,
            _ => false,
        }
    }

    pub fn get_value_ptr <'a, T> (&'a self,
                                  key : &'a str) ->  Option<TXTRecordItem<T>> {
        unsafe {
            let value_len : *mut uint8_t = uninitialized ();
            let value = TXTRecordGetValuePtr (self.get_length (), self.get_bytes_ptr (), str_to_const_c (key), value_len);

            if value == null () {
                None
            } else {
                let value = & *(value as *const T);

                Some(TXTRecordItem {
                    key : String::from (key),
                    value_len : *value_len,
                    value : value,
                })
            }
        }
    }

    pub fn get_count (&self) -> u16 {
        unsafe { TXTRecordGetCount (self.get_length (), self.get_bytes_ptr ()) }
    }

    pub fn get_item_at_index <T> (&self,
                                  index          : u16,
                                  key_buffer_len : Option<usize>) -> Result<TXTRecordItem<T>, DNSServiceErrorType> {
        unsafe {
            let keybuffer = match key_buffer_len {
                None => malloc (256),
                Some (value) => {
                    let limit = min (value, 256);
                    malloc (limit as u64)
                }
            } as *mut c_char;
            let value_len : *mut uint8_t = uninitialized ();
            let value_ptr = uninitialized ();

            match TXTRecordGetItemAtIndex (self.get_length (), self.get_bytes_ptr (), index, 256, keybuffer, value_len, value_ptr) {
                DNSServiceErrorType::NoError => {
                    let value = & *(*(value_ptr) as *const T);
                    let keyvalue = mut_c_to_str (keybuffer);
                    free (keybuffer as *mut c_void);

                    Ok(TXTRecordItem {
                        key : keyvalue,
                        value_len : *value_len,
                        value : value,
                    })
                },
                result @ _ => Err(result),
            }
        }
    }
}

impl Drop for TXTRecord {
    fn drop (&mut self) {
        unsafe { TXTRecordDeallocate (&mut self.ptr) };
    }
}
