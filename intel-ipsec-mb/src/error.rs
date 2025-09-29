use std::fmt;
use std::ffi::CStr;
use intel_ipsec_mb_sys::{imb_get_errno, imb_get_strerror};
use intel_ipsec_mb_sys::ImbErr;

use crate::mgr::MbMgr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MbMgrError {
    SystemError(i32),
    ImbErr(ImbErr),
    UnknownError(i32),
}

impl MbMgrError {

    fn get_errno(&self) -> i32 {
        match self {
            Self::SystemError(errno) => *errno,
            Self::ImbErr(imb_err) => *imb_err as i32,
            Self::UnknownError(errno) => *errno,
        }
    }

    // Todo: Think of something to make the size of Return 4 bytes instead of 8 bytes
    pub fn capture(mb_mgr: &MbMgr) -> Option<Self> {
        // SAFETY: The pointer passed to imb_get_errno is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let errno = unsafe { imb_get_errno(mb_mgr.as_ptr()) };
        if errno == 0 {
            return None;
        }
        if errno < ImbErr::IMB_ERR_MIN as i32 {
            return Some(Self::SystemError(errno));
        }
        if errno < ImbErr::IMB_ERR_MAX as i32 {
            // SAFETY: This will not fail as on the C side IMB_ERR is also an enum
            // there will surely be a mapping for the errno in enum
            let intel_err: ImbErr = unsafe { std::mem::transmute(errno as u32) };
            return Some(Self::ImbErr(intel_err));
        }
        Some(Self::UnknownError(errno))
    }
    
    pub fn capture_global() -> Option<Self> {
        // SAFETY: This is anyway an intentional null pointer since we
        // don't have a MbMgr instance on the C side, there
        // is a check to read the global error status
        let errno = unsafe { imb_get_errno(std::ptr::null_mut()) };
        if errno == 0 {
            return None;
        }
        if errno < ImbErr::IMB_ERR_MIN as i32{
            return Some(Self::SystemError(errno));
        }
        if errno < ImbErr::IMB_ERR_MAX as i32 {
            // SAFETY: This will not fail as on the C side IMB_ERR is also an enum
            // there will surely be a mapping for the errno in enum
            let intel_err: ImbErr = unsafe { std::mem::transmute(errno as u32) };
            return Some(Self::ImbErr(intel_err));
        }
        Some(Self::UnknownError(errno))
    }
}

impl fmt::Display for MbMgrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: imb_get_strerror is a safe function
        // CStr::from_ptr will give a UTF-8 string because it is hard coded 
        // in the C library or in libc
        unsafe {
            let errno = self.get_errno();
            let c_str = imb_get_strerror(errno);
            let message = std::str::from_utf8_unchecked(CStr::from_ptr(c_str).to_bytes());
            match self {
                Self::SystemError(_) => write!(f, "System Error: {}", message),
                Self::ImbErr(_) => write!(f, "Imb Error: {}", message),
                Self::UnknownError(_) => write!(f, "Unknown Error: {}", message),
            }
        }
    }
}

impl std::error::Error for MbMgrError {}
