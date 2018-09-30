// BSD 3-Clause License
//
// Copyright (c) 2018, Gregory Meyer
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS AS IS
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
