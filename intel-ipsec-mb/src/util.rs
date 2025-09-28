use std::ffi::CStr;
use intel_ipsec_mb_sys::{imb_get_version_str, imb_get_version};

pub fn get_version_str() -> &'static str {
    // SAFETY: The pointer returned by imb_get_version_str() is assumed to be valid and
    // points to a NUL-terminated C string for the lifetime of the program, as guaranteed
    // by the underlying library. We use from_ptr to create a CStr, and then convert it
    // to a Rust &str using from_utf8_unchecked, assuming the version string is valid UTF-8.
    unsafe {
        let cstr = CStr::from_ptr(imb_get_version_str());
        str::from_utf8_unchecked(cstr.to_bytes())
    }
}


pub fn get_version() -> u32 {
    // SAFETY: The imb_get_version() function returns a u32 version number from the
    // underlying library. It does not dereference any pointers or require any
    // special invariants from the caller, so it is safe to call as long as the
    // library is correctly loaded and initialized.
    unsafe {
        imb_get_version() as u32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_str() {
        let version = get_version_str();
        assert!(!version.is_empty());
        println!("Library version string: {}", version);
    }

    #[test]
    fn test_get_version() {
        let version = get_version();
        assert!(version > 0);
        println!("Library version number: {}", version);
    }
}
