use std::fmt;
use std::ffi::CStr;
use intel_ipsec_mb_sys::{imb_get_errno, imb_get_strerror};
use std::num::NonZeroI32;

use crate::mgr::MbMgr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbMgrError (NonZeroI32);

impl MbMgrError {

    pub fn kind(&self) -> MbMgrErrorKind {
        MbMgrErrorKind::from_code(self.0.get())
    }

    pub fn capture(mb_mgr: &mut MbMgr) -> Option<Self> {
        // SAFETY: The pointer passed to imb_get_errno is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let errno = unsafe { imb_get_errno(mb_mgr.as_ptr()) };
        if errno == 0 {
            return None;
        }
        // SAFETY: The errno is not zero at this point
        Some(Self (unsafe { NonZeroI32::new_unchecked(errno) } ))
    }
    
    pub fn capture_global() -> Option<Self> {
        // SAFETY: This is anyway an intentional null pointer since we
        // don't have a MbMgr instance on the C side, there
        // is a check to read the global error status
        let errno = unsafe { imb_get_errno(std::ptr::null_mut()) };
        if errno == 0 {
            return None;
        }
        // SAFETY: The errno is not zero at this point
        Some(Self (unsafe { NonZeroI32::new_unchecked(errno) } ))
    }
}

impl fmt::Display for MbMgrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: imb_get_strerror is a safe function
        // CStr::from_ptr will give a UTF-8 string because it is hard coded 
        // in the C library or in libc
        unsafe {
            let errno = self.0.get();
            let c_str = imb_get_strerror(errno);
            let message = std::str::from_utf8_unchecked(CStr::from_ptr(c_str).to_bytes());

            write!(f, "{}", message)
        }
    }
}

impl std::error::Error for MbMgrError {}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MbMgrErrorKind {
    OutOfMemory,
    
    NullMbMgr,
    JobNullSrc,
    JobNullDst,
    JobNullKey,
    JobNullIv,
    JobNullAuth,
    JobNullAad,
    JobCiphLen,
    JobAuthLen,
    JobIvLen,
    JobKeyLen,
    JobAuthTagLen,
    JobAadLen,
    JobSrcOffset,
    JobChainOrder,
    CiphMode,
    HashAlgo,
    JobNullAuthKey,
    JobNullSglCtx,
    JobNullNextIv,
    JobPonPli,
    NullSrc,
    NullDst,
    NullKey,
    NullExpKey,
    NullIv,
    NullAuth,
    NullAad,
    CiphLen,
    AuthLen,
    IvLen,
    KeyLen,
    AuthTagLen,
    AadLen,
    SrcOffset,
    NullAuthKey,
    NullCtx,
    JobNullHmacOpad,
    JobNullHmacIpad,
    JobNullXcbcK1Exp,
    JobNullXcbcK2,
    JobNullXcbcK3,
    JobCiphDir,
    JobNullGhashInitTag,
    MissingCpuflagsInitMgr,
    NullJob,
    QueueSpace,
    NullBurst,
    BurstSize,
    BurstOoo,
    Selftest,
    BurstSuiteId,
    JobSglState,
    
    UnknownError(i32),
}


impl MbMgrErrorKind {
    fn from_code(code: i32) -> Self {
        match code {
            12 => MbMgrErrorKind::OutOfMemory, // ENOMEM
            
            2001 => MbMgrErrorKind::NullMbMgr,
            2002 => MbMgrErrorKind::JobNullSrc,
            2003 => MbMgrErrorKind::JobNullDst,
            2004 => MbMgrErrorKind::JobNullKey,
            2005 => MbMgrErrorKind::JobNullIv,
            2006 => MbMgrErrorKind::JobNullAuth,
            2007 => MbMgrErrorKind::JobNullAad,
            2008 => MbMgrErrorKind::JobCiphLen,
            2009 => MbMgrErrorKind::JobAuthLen,
            2010 => MbMgrErrorKind::JobIvLen,
            2011 => MbMgrErrorKind::JobKeyLen,
            2012 => MbMgrErrorKind::JobAuthTagLen,
            2013 => MbMgrErrorKind::JobAadLen,
            2014 => MbMgrErrorKind::JobSrcOffset,
            2015 => MbMgrErrorKind::JobChainOrder,
            2016 => MbMgrErrorKind::CiphMode,
            2017 => MbMgrErrorKind::HashAlgo,
            2018 => MbMgrErrorKind::JobNullAuthKey,
            2019 => MbMgrErrorKind::JobNullSglCtx,
            2020 => MbMgrErrorKind::JobNullNextIv,
            2021 => MbMgrErrorKind::JobPonPli,
            2022 => MbMgrErrorKind::NullSrc,
            2023 => MbMgrErrorKind::NullDst,
            2024 => MbMgrErrorKind::NullKey,
            2025 => MbMgrErrorKind::NullExpKey,
            2026 => MbMgrErrorKind::NullIv,
            2027 => MbMgrErrorKind::NullAuth,
            2028 => MbMgrErrorKind::NullAad,
            2029 => MbMgrErrorKind::CiphLen,
            2030 => MbMgrErrorKind::AuthLen,
            2031 => MbMgrErrorKind::IvLen,
            2032 => MbMgrErrorKind::KeyLen,
            2033 => MbMgrErrorKind::AuthTagLen,
            2034 => MbMgrErrorKind::AadLen,
            2035 => MbMgrErrorKind::SrcOffset,
            2036 => MbMgrErrorKind::NullAuthKey,
            2037 => MbMgrErrorKind::NullCtx,
            2038 => MbMgrErrorKind::JobNullHmacOpad,
            2039 => MbMgrErrorKind::JobNullHmacIpad,
            2040 => MbMgrErrorKind::JobNullXcbcK1Exp,
            2041 => MbMgrErrorKind::JobNullXcbcK2,
            2042 => MbMgrErrorKind::JobNullXcbcK3,
            2043 => MbMgrErrorKind::JobCiphDir,
            2044 => MbMgrErrorKind::JobNullGhashInitTag,
            2045 => MbMgrErrorKind::MissingCpuflagsInitMgr,
            2046 => MbMgrErrorKind::NullJob,
            2047 => MbMgrErrorKind::QueueSpace,
            2048 => MbMgrErrorKind::NullBurst,
            2049 => MbMgrErrorKind::BurstSize,
            2050 => MbMgrErrorKind::BurstOoo,
            2051 => MbMgrErrorKind::Selftest,
            2052 => MbMgrErrorKind::BurstSuiteId,
            2053 => MbMgrErrorKind::JobSglState,
            
            _ => MbMgrErrorKind::UnknownError(code),
        }
    }
}
