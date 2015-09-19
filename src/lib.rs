#![crate_name = "dnssd"]
#![crate_type = "lib"]

#![feature(box_syntax)]

extern crate libc;

pub mod ffi;
mod utils;
pub mod record;
pub mod service;
pub mod callback;
pub mod txtrecord;
