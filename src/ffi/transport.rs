extern crate libc;

use ffi::*;

#[repr(C)]
pub struct zcm_msg_t {
    pub utime: u64,
    pub channel: *const libc::c_char,
    pub len: libc::size_t,
    pub buf: *const u8,
}

#[repr(C)]
pub struct zcm_trans_t {
    pub trans_type: zcm_type,
    pub vtbl: *mut zcm_trans_methods_t,
}

#[repr(C)]
pub struct zcm_trans_methods_t {
    pub get_mtu: extern "C" fn(*mut zcm_trans_t) -> libc::size_t,
    pub sendmsg: extern "C" fn(*mut zcm_trans_t, zcm_msg_t) -> libc::c_int,
    pub recvmsg_enable: extern "C" fn(*mut zcm_trans_t, *const libc::c_char, bool) -> libc::c_int,
    pub recvmsg: extern "C" fn(*mut zcm_trans_t, *mut zcm_msg_t, libc::c_int) -> libc::c_int,
    pub update: extern "C" fn(*mut zcm_trans_t) -> libc::c_int,
    pub destroy: extern "C" fn(*mut zcm_trans_t) -> libc::c_int,
}

pub unsafe fn zcm_trans_get_mtu(zt: *mut zcm_trans_t) -> libc::size_t {
    ((*(*zt).vtbl).get_mtu)(zt)
}

pub unsafe fn zcm_trans_sendmsg(zt: *mut zcm_trans_t, msg: zcm_msg_t) -> libc::c_int {
    ((*(*zt).vtbl).sendmsg)(zt, msg)
}

pub unsafe fn zcm_trans_recvmsg_enable(
    zt: *mut zcm_trans_t,
    channel: *const libc::c_char,
    enable: bool,
) -> libc::c_int {
    ((*(*zt).vtbl).recvmsg_enable)(zt, channel, enable)
}

pub unsafe fn zcm_trans_recvmsg(
    zt: *mut zcm_trans_t,
    msg: *mut zcm_msg_t,
    timeout: libc::c_int,
) -> libc::c_int {
    ((*(*zt).vtbl).recvmsg)(zt, msg, timeout)
}

pub unsafe fn zcm_trans_update(zt: *mut zcm_trans_t) -> libc::c_int {
    ((*(*zt).vtbl).update)(zt)
}

pub unsafe fn zcm_trans_destroy(zt: *mut zcm_trans_t) -> libc::c_int {
    ((*(*zt).vtbl).destroy)(zt)
}
