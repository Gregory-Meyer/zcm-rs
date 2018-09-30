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
