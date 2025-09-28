//! Low-level unsafe bindings for Intel IPSec Multi-Buffer Crypto Library
//!
//! This crate provides raw FFI bindings to Intel's optimized cryptographic library.
//! 
//! # Safety
//! 
//! All functions in this crate are `unsafe` because they're direct FFI calls
//! to C code. Use the higher-level `intel-ipsec-mb` crate for safe APIs.
//!
//! # Example
//!
//! ```rust
//! use intel_ipsec_mb_sys::*;
//! 
//! unsafe {
//!     let mgr = imb_alloc_mb_mgr(0);
//!     assert!(!mgr.is_null());
//!     imb_free_mb_mgr(mgr);
//! }
//! ```

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

#[allow(improper_ctypes)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

pub use bindings::IMB_MGR as ImbMgr;
pub use bindings::IMB_JOB as ImbJob;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_allocation() {
        unsafe {
            // Test basic library linkage
            let mgr = alloc_mb_mgr(0);
            assert!(!mgr.is_null(), "Failed to allocate MB_MGR");
            
            // Clean up
            free_mb_mgr(mgr);
        }
    }

    #[test]
    fn test_version_info() {
        unsafe {
            // Test that we can call version functions
            let version_str = imb_get_version_str();
            assert!(!version_str.is_null(), "Version string should not be null");
            
            // Convert to Rust string for verification
            let c_str = std::ffi::CStr::from_ptr(version_str);
            let version = c_str.to_str().expect("Version should be valid UTF-8");
            
            // Just verify it's not empty
            assert!(!version.is_empty(), "Version string should not be empty");
            println!("Intel IPSec-MB Version: {}", version);
        }
    }

}