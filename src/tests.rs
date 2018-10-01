extern crate libc;

use super::*;

extern "C" fn handler(
    buffer: *const ffi::zcm_recv_buf_t,
    channel: *const libc::c_char,
    usr: *mut libc::c_void,
) {
    let received_ptr = usr as *mut bool;
    let received = &mut unsafe { *received_ptr };

    println!(
        "buffer: {:?}, channel: {:?}, usr: {:?}",
        buffer, channel, usr
    );

    *received = true;
}

#[test]
fn foo() {
    let mut zcm = Zcm::new("ipc").unwrap();
    let channel = std::ffi::CString::new("foo").unwrap();

    let mut received = false;

    let sub = unsafe {
        ffi::zcm_subscribe(
            zcm.as_mut_ptr(),
            channel.as_ptr(),
            handler,
            &mut received as *mut bool as *mut libc::c_void,
        )
    };

    assert!(!sub.is_null());

    let message = std::ffi::CString::new("foo bar baz qux").unwrap();
    let msg_slice = message.as_bytes_with_nul();

    let pub_ret = unsafe {
        ffi::zcm_publish(
            zcm.as_mut_ptr(),
            channel.as_ptr(),
            msg_slice.as_ptr(),
            msg_slice.len() as u32,
        )
    };

    assert_eq!(pub_ret, 0);
    assert!(received);

    let unsub_ret = unsafe { ffi::zcm_unsubscribe(zcm.as_mut_ptr(), sub) };

    assert_eq!(unsub_ret, 0);
}
