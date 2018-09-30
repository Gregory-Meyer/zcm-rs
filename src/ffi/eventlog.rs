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

#[repr(C)]
pub struct zcm_eventlog_event_t {
    pub eventnum: i64,
    pub timestamp: i64,
    pub channellen: i32,
    pub channel: *mut libc::c_char,
    pub data: *mut u8,
}

#[repr(C)]
pub struct zcm_eventlog_t {
    pub f: *mut libc::FILE,
    pub eventcount: i64,
}

#[link(name = "zcm")]
extern "C" {
    pub fn zcm_eventlog_create(
        path: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> *mut zcm_eventlog_t;

    pub fn zcm_eventlog_destroy(eventlog: *mut zcm_eventlog_t);

    pub fn zcm_eventlog_get_fileptr(eventlog: *mut zcm_eventlog_t) -> *mut libc::FILE;

    pub fn zcm_eventlog_seek_to_timestamp(eventlog: *mut zcm_eventlog_t, ts: i64) -> libc::c_int;

    pub fn zcm_eventlog_read_next_event(eventlog: *mut zcm_eventlog_t)
        -> *mut zcm_eventlog_event_t;

    pub fn zcm_eventlog_read_prev_event(eventlog: *mut zcm_eventlog_t)
        -> *mut zcm_eventlog_event_t;

    pub fn zcm_eventlog_read_event_at_offset(
        eventlog: *mut zcm_eventlog_t,
        offset: libc::off_t,
    ) -> *mut zcm_eventlog_event_t;

    pub fn zcm_eventlog_free_event(event: *mut zcm_eventlog_event_t);

    pub fn zcm_eventlog_write_event(
        eventlog: *mut zcm_eventlog_t,
        event: *const zcm_eventlog_event_t,
    ) -> libc::c_int;
}
