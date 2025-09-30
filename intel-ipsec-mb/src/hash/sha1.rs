use crate::error::MbMgrError;
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::IMB_SHA1_DIGEST_SIZE_IN_BYTES;
use std::os::raw::c_void;

// Todo fix this usize
// Todo: fix this unwrap in the future
impl MbMgr {
    /// Compute SHA-1 hash of the input buffer
    pub fn sha1(
        &mut self,
        buffer: &[u8],
    ) -> Result<[u8; IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
        let mut output = [0; IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize];
        self.exec(|mgr_ptr| unsafe {
            let sha1_fn = (*mgr_ptr).sha1.unwrap();
            sha1_fn(
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
                output.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(output)
    }

    /// Compute SHA-1 hash of one block (64 bytes)
    pub fn sha1_one_block(
        &mut self,
        buffer: &[u8],
    ) -> Result<[u8; IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
        let mut output = [0; IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize];
        self.exec(|mgr_ptr| unsafe {
            let sha1_fn = (*mgr_ptr).sha1_one_block.unwrap();
            sha1_fn(
                buffer.as_ptr() as *const c_void,
                output.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(output)
    }
}
