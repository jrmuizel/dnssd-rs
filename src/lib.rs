#![crate_name = "dnssd"]
#![crate_type = "lib"]

extern crate libc;

pub mod ffi;
mod utils;
pub mod service;
pub mod callback;
pub mod txtrecord;
pub mod general;
