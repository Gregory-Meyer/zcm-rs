#![allow(non_camel_case_types)]

extern crate libc;

use ffi::*;

#[repr(C)]
pub enum zcm_type {
    ZCM_BLOCKING,
    ZCM_NONBLOCKING,
}

#[repr(C)]
pub enum zcm_return_codes {
    ZCM_EOK = 0,
    ZCM_EINVALID = 1,
    ZCM_EAGAIN = 2,
    ZCM_ECONNECT = 3,
    ZCM_EINTR = 4,
    ZCM_EUNKNOWN = 5,
    ZCM_NUM_RETURN_CODES = 6,
}

pub enum zcm_sub_t {}

pub type zcm_msg_handler_t =
    extern "C" fn(*const zcm_recv_buf_t, *const libc::c_char, *mut libc::c_void);

#[repr(C)]
pub struct zcm_t {
    pub m_type: zcm_type,
    pub m_impl: *mut libc::c_void,
    pub err: libc::c_int,
}

#[repr(C)]
pub struct zcm_recv_buf_t {
    pub recv_utime: i64,
    pub zcm: *mut zcm_t,
    pub data: *mut u8,
    pub data_size: u32,
}

#[link(name = "zcm")]
extern "C" {
    pub fn zcm_retcode_name_to_enum(zcm_retcode_name: *const libc::c_char) -> libc::c_int;

    pub fn zcm_create(url: *const libc::c_char) -> *mut zcm_t;

    pub fn zcm_create_trans(zt: *mut zcm_trans_t) -> *mut zcm_t;

    pub fn zcm_destroy(zt: *mut zcm_t);

    pub fn zcm_init(zcm: *mut zcm_t, url: *const libc::c_char) -> libc::c_int;

    pub fn zcm_init_trans(zcm: *mut zcm_t, zt: *mut zcm_trans_t) -> libc::c_int;

    pub fn zcm_cleanup(zcm: *mut zcm_t);

    pub fn zcm_errno(zcm: *const zcm_t) -> libc::c_int;

    pub fn zcm_strerror(zcm: *const zcm_t) -> *const libc::c_char;

    pub fn zcm_strerrno(err: libc::c_int) -> *const libc::c_char;

    pub fn zcm_subscribe(
        zcm: *mut zcm_t,
        channel: *const libc::c_char,
        cb: zcm_msg_handler_t,
        usr: *mut libc::c_void,
    ) -> *mut zcm_sub_t;

    pub fn zcm_try_subscribe(
        zcm: *mut zcm_t,
        channel: *const libc::c_char,
        cb: zcm_msg_handler_t,
        usr: *mut libc::c_void,
    ) -> *mut zcm_sub_t;

    pub fn zcm_try_unsubscribe(zcm: *mut zcm_t, sub: *mut zcm_sub_t) -> libc::c_int;

    pub fn zcm_publish(
        zcm: *mut zcm_t,
        channel: *const libc::c_char,
        data: *const u8,
        len: u32,
    ) -> libc::c_int;

    pub fn zcm_flush(zcm: *mut zcm_t);

    pub fn zcm_try_flush(zcm: *mut zcm_t) -> libc::c_int;

    pub fn zcm_run(zcm: *mut zcm_t);

    pub fn zcm_start(zcm: *mut zcm_t);

    pub fn zcm_stop(zcm: *mut zcm_t);

    pub fn zcm_try_stop(zcm: *mut zcm_t) -> libc::c_int;

    pub fn zcm_pause(zcm: *mut zcm_t);

    pub fn zcm_resume(zcm: *mut zcm_t);

    pub fn zcm_handle(zcm: *mut zcm_t) -> libc::c_int;

    pub fn zcm_set_queue_size(zcm: *mut zcm_t, numMsgs: u32);

    pub fn zcm_try_set_queue_size(zcm: *mut zcm_t, numMsgs: u32) -> libc::c_int;

    pub fn zcm_handle_nonblock(zcm: *mut zcm_t) -> libc::c_int;
}

pub const ZCM_MAJOR_VERSION: libc::c_int = 1;
pub const ZCM_MINOR_VERSION: libc::c_int = 0;
pub const ZCM_MICRO_VERSION: libc::c_int = 0;
