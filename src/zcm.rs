extern crate std;

use super::*;

pub struct Zcm {
    zcm: ffi::zcm_t,
}

impl Zcm {
    pub fn new(url: &str) -> Result<Zcm, NewError> {
        let url_owned = match std::ffi::CString::new(url) {
            Ok(u) => u,
            Err(e) => return Err(NewError::Nul(e)),
        };

        let mut zcm = Zcm {
            zcm: unsafe { std::mem::uninitialized() },
        };

        if unsafe { ffi::zcm_init(zcm.as_mut_ptr(), url_owned.as_ptr()) } != 0 {
            return Err(NewError::Error(zcm.errno().unwrap()));
        }

        Ok(zcm)
    }

    pub fn errno(&self) -> Option<Error> {
        let err = unsafe { ffi::zcm_errno(self.as_ptr()) };

        Error::from_raw(err)
    }

    pub fn strerror(&self) -> std::borrow::Cow<str> {
        let err = unsafe { ffi::zcm_strerror(self.as_ptr()) };

        unsafe { std::ffi::CStr::from_ptr(err) }.to_string_lossy()
    }

    pub fn flush(&mut self) {
        unsafe { ffi::zcm_flush(self.as_mut_ptr()) };
    }

    pub fn try_flush(&mut self) -> Result<(), Error> {
        let result = unsafe { ffi::zcm_try_flush(self.as_mut_ptr()) };

        match Error::from_raw(result) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    pub fn as_ptr(&self) -> *const ffi::zcm_t {
        &self.zcm as *const ffi::zcm_t
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::zcm_t {
        &mut self.zcm as *mut ffi::zcm_t
    }
}

impl Drop for Zcm {
    fn drop(&mut self) {
        unsafe { ffi::zcm_cleanup(&mut self.zcm as *mut ffi::zcm_t) }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Invalid,
    Again,
    Connect,
    Interrupted,
    Unknown,
    NumReturnCodes,
}

impl Error {
    pub fn from_raw(err: libc::c_int) -> Option<Error> {
        use ffi::zcm_return_codes::*;

        if err < 0 || err >= unsafe { std::mem::transmute(ZCM_NUM_RETURN_CODES) } {
            return Some(Error::NumReturnCodes);
        }

        match unsafe { std::mem::transmute(err) } {
            ZCM_EOK => None,
            ZCM_EINVALID => Some(Error::Invalid),
            ZCM_EAGAIN => Some(Error::Again),
            ZCM_ECONNECT => Some(Error::Connect),
            ZCM_EINTR => Some(Error::Interrupted),
            ZCM_EUNKNOWN => Some(Error::Unknown),
            _ => Some(Error::NumReturnCodes),
        }
    }

    pub fn as_int(&self) -> libc::c_int {
        use ffi::zcm_return_codes::*;

        let underlying = match self {
            Error::Invalid => ZCM_EINVALID,
            Error::Again => ZCM_EAGAIN,
            Error::Connect => ZCM_ECONNECT,
            Error::Interrupted => ZCM_EINTR,
            Error::Unknown => ZCM_EUNKNOWN,
            Error::NumReturnCodes => ZCM_NUM_RETURN_CODES,
        };

        unsafe { std::mem::transmute(underlying) }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str = unsafe { std::ffi::CStr::from_ptr(ffi::zcm_strerrno(self.as_int())) };

        write!(f, "{}", as_str.to_string_lossy())
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug)]
pub enum NewError {
    Nul(std::ffi::NulError),
    Error(Error),
}

impl std::fmt::Display for NewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NewError::Nul(e) => write!(f, "{}", e),
            NewError::Error(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for NewError {}
