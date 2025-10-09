use std::fmt;
use std::ffi::CStr;
use intel_ipsec_mb_sys::{imb_get_errno, imb_get_strerror};
use std::num::NonZeroI32;

use crate::mgr::MbMgr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MbError (pub NonZeroI32);

impl MbError {

    pub fn kind(&self) -> MbMgrErrorKind {
        MbMgrErrorKind::from_code(self.0.get())
    }

    pub fn from_kind(kind: MbMgrErrorKind) -> Self {
        Self(NonZeroI32::new(kind.to_code()).unwrap())
    }

    pub fn capture(mb_mgr: &MbMgr) -> Option<Self> {
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

impl fmt::Display for MbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.get() > 3000 {
            return write!(f, "Rust implementation error: {}", self.kind().to_code());
        } 

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

impl std::error::Error for MbError {}



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

    InvalidOutputSize,
    IllegalJobState,
    NoJobAvailable,
    ChannelClosed,
    CompletionFailed,

    RuntimeError,
    
    UnknownError(i32),
}


impl MbMgrErrorKind {
    pub fn to_code(self) -> i32 {
        match self {
            MbMgrErrorKind::OutOfMemory => 12,
            MbMgrErrorKind::NullMbMgr => 2001,
            MbMgrErrorKind::JobNullSrc => 2002,
            MbMgrErrorKind::JobNullDst => 2003,
            MbMgrErrorKind::JobNullKey => 2004,
            MbMgrErrorKind::JobNullIv => 2005,
            MbMgrErrorKind::JobNullAuth => 2006,
            MbMgrErrorKind::JobNullAad => 2007,
            MbMgrErrorKind::JobCiphLen => 2008,
            MbMgrErrorKind::JobAuthLen => 2009,
            MbMgrErrorKind::JobIvLen => 2010,
            MbMgrErrorKind::JobKeyLen => 2011,
            MbMgrErrorKind::JobAuthTagLen => 2012,
            MbMgrErrorKind::JobAadLen => 2013,
            MbMgrErrorKind::JobSrcOffset => 2014,
            MbMgrErrorKind::JobChainOrder => 2015,
            MbMgrErrorKind::CiphMode => 2016,
            MbMgrErrorKind::HashAlgo => 2017,
            MbMgrErrorKind::JobNullAuthKey => 2018,
            MbMgrErrorKind::JobNullSglCtx => 2019,
            MbMgrErrorKind::JobNullNextIv => 2020,
            MbMgrErrorKind::JobPonPli => 2021,
            MbMgrErrorKind::NullSrc => 2022,
            MbMgrErrorKind::NullDst => 2023,
            MbMgrErrorKind::NullKey => 2024,
            MbMgrErrorKind::NullExpKey => 2025,
            MbMgrErrorKind::NullIv => 2026,
            MbMgrErrorKind::NullAuth => 2027,
            MbMgrErrorKind::NullAad => 2028,
            MbMgrErrorKind::CiphLen => 2029,
            MbMgrErrorKind::AuthLen => 2030,
            MbMgrErrorKind::IvLen => 2031,
            MbMgrErrorKind::KeyLen => 2032,
            MbMgrErrorKind::AuthTagLen => 2033,
            MbMgrErrorKind::AadLen => 2034,
            MbMgrErrorKind::SrcOffset => 2035,
            MbMgrErrorKind::NullAuthKey => 2036,
            MbMgrErrorKind::NullCtx => 2037,
            MbMgrErrorKind::JobNullHmacOpad => 2038,
            MbMgrErrorKind::JobNullHmacIpad => 2039,
            MbMgrErrorKind::JobNullXcbcK1Exp => 2040,
            MbMgrErrorKind::JobNullXcbcK2 => 2041,
            MbMgrErrorKind::JobNullXcbcK3 => 2042,
            MbMgrErrorKind::JobCiphDir => 2043,
            MbMgrErrorKind::JobNullGhashInitTag => 2044,
            MbMgrErrorKind::MissingCpuflagsInitMgr => 2045,
            MbMgrErrorKind::NullJob => 2046,
            MbMgrErrorKind::QueueSpace => 2047,
            MbMgrErrorKind::NullBurst => 2048,
            MbMgrErrorKind::BurstSize => 2049,
            MbMgrErrorKind::BurstOoo => 2050,
            MbMgrErrorKind::Selftest => 2051,
            MbMgrErrorKind::BurstSuiteId => 2052,
            MbMgrErrorKind::JobSglState => 2053,

            // Rust implementation errors (codes > 3000)
            MbMgrErrorKind::InvalidOutputSize => 3001,
            MbMgrErrorKind::IllegalJobState => 3002,
            MbMgrErrorKind::NoJobAvailable => 3003,
            MbMgrErrorKind::ChannelClosed => 3004,
            MbMgrErrorKind::CompletionFailed => 3005,

            MbMgrErrorKind::RuntimeError => 3006,

            MbMgrErrorKind::UnknownError(code) => code,
        }
    }

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

            // Rust implementation errors (codes > 3000)
            3001 => MbMgrErrorKind::InvalidOutputSize,
            3002 => MbMgrErrorKind::IllegalJobState,
            3003 => MbMgrErrorKind::NoJobAvailable,
            3004 => MbMgrErrorKind::ChannelClosed,
            3005 => MbMgrErrorKind::CompletionFailed,

            3006 => MbMgrErrorKind::RuntimeError,

            _ => MbMgrErrorKind::UnknownError(code),
        }
    }
}
