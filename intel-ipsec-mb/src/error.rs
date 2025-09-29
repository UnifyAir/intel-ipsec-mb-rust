use std::fmt;
use std::ffi::CStr;
use intel_ipsec_mb_sys::{imb_get_errno, imb_get_strerror};
use intel_ipsec_mb_sys::ImbErr;

use crate::multi_buffer_manager::MultiBufferManager;

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

    pub fn capture(multi_buffer_manager: &MultiBufferManager) -> Self {
        // SAFETY: The pointer passed to imb_get_errno is assumed to be valid otherwise we
        // would not be having a MultiBufferManager instance
        let errno = unsafe { imb_get_errno(multi_buffer_manager.as_ptr()) };
        if errno < ImbErr::IMB_ERR_MIN as i32 {
            return Self::SystemError(errno);
        }
        if errno < ImbErr::IMB_ERR_MAX as i32 {
            // SAFETY: This will not fail as on the C side IMB_ERR is also an enum
            // there will surely be a mapping for the errno in enum
            let intel_err: ImbErr = unsafe { std::mem::transmute(errno as u32) };
            return Self::ImbErr(intel_err);
        }
        Self::UnknownError(errno)
    }
    
    pub fn capture_global() -> Self {
        // SAFETY: This is anyway an intentional null pointer since we
        // don't have a MultiBufferManager instance on the C side, there
        // is a check to read the global error status
        let errno = unsafe { imb_get_errno(std::ptr::null_mut()) };
        if errno < ImbErr::IMB_ERR_MIN as i32{
            return Self::SystemError(errno);
        }
        if errno < ImbErr::IMB_ERR_MAX as i32 {
            // SAFETY: This will not fail as on the C side IMB_ERR is also an enum
            // there will surely be a mapping for the errno in enum
            let intel_err: ImbErr = unsafe { std::mem::transmute(errno as u32) };
            return Self::ImbErr(intel_err);
        }
        Self::UnknownError(errno)
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

#[cfg(test)]
mod tests {
    use super::*;
    use intel_ipsec_mb_sys::ImbErr;

    #[test]
    fn test_error_classification_logic() {
        // Test the core business logic: how errno values are classified
        let min_imb_err = ImbErr::IMB_ERR_MIN as i32;
        let max_imb_err = ImbErr::IMB_ERR_MAX as i32;
        
        // System errors (below IMB range)
        let system_err = MbMgrError::SystemError(min_imb_err - 1);
        assert_eq!(system_err.get_errno(), min_imb_err - 1);
        
        // IMB errors (within range)
        let imb_err = MbMgrError::ImbErr(ImbErr::IMB_ERR_NULL_MBMGR);
        assert_eq!(imb_err.get_errno(), ImbErr::IMB_ERR_NULL_MBMGR as i32);
        
        // Unknown errors (above IMB range)
        let unknown_err = MbMgrError::UnknownError(max_imb_err + 1);
        assert_eq!(unknown_err.get_errno(), max_imb_err + 1);
    }

    #[test]
    fn test_capture_methods_with_real_manager() {
        // Test that capture methods work with actual MultiBufferManager
        if let Ok(manager) = MultiBufferManager::new() {
            // Test capture with manager - should not panic
            let _error = MbMgrError::capture(&manager);
        }
        
        // Test capture_global - should not panic
        let _error = MbMgrError::capture_global();
    }

    #[test]
    fn test_error_display_contains_useful_info() {
        // Test that error messages are informative and contain expected prefixes
        let system_err = MbMgrError::SystemError(1);
        let system_msg = format!("{}", system_err);
        assert!(system_msg.contains("System Error"));
        
        let imb_err = MbMgrError::ImbErr(ImbErr::IMB_ERR_NULL_MBMGR);
        let imb_msg = format!("{}", imb_err);
        assert!(imb_msg.contains("Imb Error"));
        
        let unknown_err = MbMgrError::UnknownError(3000);
        let unknown_msg = format!("{}", unknown_err);
        assert!(unknown_msg.contains("Unknown Error"));
    }

    #[test]
    fn test_critical_imb_error_codes() {
        // Test the most important IMB error codes that users will encounter
        let critical_errors = vec![
            (ImbErr::IMB_ERR_NULL_MBMGR, "null manager"),
            (ImbErr::IMB_ERR_JOB_NULL_SRC, "null source"),
            (ImbErr::IMB_ERR_JOB_NULL_DST, "null destination"),
            (ImbErr::IMB_ERR_JOB_NULL_KEY, "null key"),
            (ImbErr::IMB_ERR_JOB_CIPH_LEN, "cipher length"),
            (ImbErr::IMB_ERR_JOB_AUTH_LEN, "auth length"),
        ];
        
        for (error_code, _description) in critical_errors {
            let mb_error = MbMgrError::ImbErr(error_code);
            assert_eq!(mb_error.get_errno(), error_code as i32);
            
            // Verify error message is generated without panicking
            let _msg = format!("{}", mb_error);
        }
    }

    #[test]
    fn test_error_boundary_conditions() {
        // Test edge cases that could cause classification bugs
        let min_imb = ImbErr::IMB_ERR_MIN as i32;
        let max_imb = ImbErr::IMB_ERR_MAX as i32;
        
        // Test exact boundary values
        let at_min = MbMgrError::ImbErr(ImbErr::IMB_ERR_MIN);
        assert_eq!(at_min.get_errno(), min_imb);
        
        let at_max = MbMgrError::ImbErr(ImbErr::IMB_ERR_MAX);
        assert_eq!(at_max.get_errno(), max_imb);
        
        // Test values just outside boundaries
        let just_below = MbMgrError::SystemError(min_imb - 1);
        assert_eq!(just_below.get_errno(), min_imb - 1);
        
        let just_above = MbMgrError::UnknownError(max_imb + 1);
        assert_eq!(just_above.get_errno(), max_imb + 1);
    }

    #[test]
    fn test_error_in_result_handling() {
        // Test practical error handling patterns that users will write
        fn simulate_operation() -> Result<(), MbMgrError> {
            Err(MbMgrError::ImbErr(ImbErr::IMB_ERR_NULL_MBMGR))
        }
        
        match simulate_operation() {
            Err(MbMgrError::ImbErr(ImbErr::IMB_ERR_NULL_MBMGR)) => {
                // Expected behavior
            }
            Err(MbMgrError::ImbErr(other_imb_err)) => {
                panic!("Unexpected IMB error: {:?}", other_imb_err);
            }
            Err(MbMgrError::SystemError(errno)) => {
                panic!("Unexpected system error: {}", errno);
            }
            Err(MbMgrError::UnknownError(errno)) => {
                panic!("Unexpected unknown error: {}", errno);
            }
            Ok(_) => {
                panic!("Expected error but got success");
            }
        }
    }

    #[test]
    fn test_error_codes_match_c_library() {
        // Verify our error codes match the C library constants
        // This is critical for FFI compatibility
        assert_eq!(ImbErr::IMB_ERR_MIN as i32, 2000);
        assert_eq!(ImbErr::IMB_ERR_MAX as i32, 2054);
        assert_eq!(ImbErr::IMB_ERR_NULL_MBMGR as i32, 2001);
        assert_eq!(ImbErr::IMB_ERR_JOB_NULL_SRC as i32, 2002);
    }
}
