#![allow(non_camel_case_types)]

extern crate libc;

pub const ZCM_OPTS_MAX: libc::c_int = 128;

#[repr(C)]
pub struct zcm_url_opts_t {
    pub numopts: libc::size_t,
    pub name: [*const libc::c_char; ZCM_OPTS_MAX as usize],
    pub value: [*const libc::c_char; ZCM_OPTS_MAX as usize],
}

pub enum zcm_url_t {}

#[link(name = "zcm")]
extern "C" {
    pub fn zcm_url_create(url: *const libc::c_char) -> *mut zcm_url_t;

    pub fn zcm_url_destroy(u: *mut zcm_url_t);

    pub fn zcm_url_protocol(u: *mut zcm_url_t) -> *const libc::c_char;

    pub fn zcm_url_address(u: *mut zcm_url_t) -> *const libc::c_char;

    pub fn zcm_url_opts(u: *mut zcm_url_t) -> *mut zcm_url_opts_t;
}
