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
