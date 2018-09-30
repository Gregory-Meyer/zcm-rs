#![allow(non_camel_case_types)]

extern crate libc;

use ffi::*;

pub type zcm_trans_create_func = extern "C" fn(url: *mut zcm_url_t) -> *mut zcm_trans_t;

#[link(name = "zcm")]
extern "C" {
    pub fn zcm_transport_register(
        name: *const libc::c_char,
        desc: *const libc::c_char,
        creator: zcm_trans_create_func,
    );

    pub fn zcm_transport_find(name: *const libc::c_char) -> zcm_trans_create_func;

    pub fn zcm_transport_help(f: *mut libc::FILE);
}
