use crate::error::{MbMgrError, MbMgrErrorKind};
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::*;
use intel_ipsec_mb_sys::ImbJob;
use crate::job::MbJob;
use std::os::raw::c_void;


pub trait Sha1 {
    fn sha1(
        &mut self,
        buffer: impl AsRef<[u8]>,
        output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError>;

    fn sha1_one_block(
        &mut self,
        buffer: impl AsRef<[u8]>,
        output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError>;

    fn fill_job(
        &mut self,
        job: &mut MbJob,
        buffer: impl AsRef<[u8]>,
        output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError>;

}

// Todo fix this usize
// Todo: fix this unwrap in the future
impl Sha1 for MbMgr {
    /// Compute SHA-1 hash of the input buffer
    fn sha1(
        &mut self,
        buffer: impl AsRef<[u8]>,
        mut output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError> {
        let buffer_slice = buffer.as_ref();
        let output_slice = output.as_mut();

        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbMgrError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }

        self.exec(|mgr_mut_ptr| unsafe {
            let sha1_fn = (*mgr_mut_ptr).sha1.unwrap();
            sha1_fn(
                buffer_slice.as_ptr() as *const c_void,
                buffer_slice.len() as u64,
                output_slice.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(())
    }

    /// Compute SHA-1 hash of one block (64 bytes)
    fn sha1_one_block(
        &mut self,
        buffer: impl AsRef<[u8]>,
        mut output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError> {

        let buffer_slice = buffer.as_ref();
        let output_slice = output.as_mut();

        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbMgrError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }

        self.exec(|mgr_mut_ptr| unsafe {
            let sha1_fn = (*mgr_mut_ptr).sha1_one_block.unwrap();
            sha1_fn(
                buffer_slice.as_ptr() as *const c_void,
                output_slice.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(())
    }

    fn fill_job(
        &mut self,
        job: &mut MbJob,
        buffer: impl AsRef<[u8]>,
        mut output: impl AsMut<[u8]>,
    ) -> Result<(), MbMgrError> {
        let buffer_slice = buffer.as_ref();
        let output_slice = output.as_mut();
        
        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbMgrError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }
        
        // Get owned copy of ImbJob
        let mut imb_job: ImbJob = unsafe { *job.as_mut_ptr()? };
        
        // Set hash algorithm to plain SHA-1
        imb_job.hash_alg = IMB_HASH_ALG_IMB_AUTH_SHA_1;
        
        // Configure the input buffer (using __bindgen_anon_1 for src)
        imb_job.__bindgen_anon_1.src = buffer_slice.as_ptr();
        imb_job.hash_start_src_offset_in_bytes = 0;
        imb_job.__bindgen_anon_5.msg_len_to_hash_in_bytes = buffer_slice.len() as u64;
        
        // Configure the output buffer for auth tag
        imb_job.auth_tag_output = output_slice.as_mut_ptr();
        imb_job.auth_tag_output_len_in_bytes = IMB_SHA1_DIGEST_SIZE_IN_BYTES as u64;
        
        // Initialize status
        imb_job.status = IMB_STATUS_IMB_STATUS_COMPLETED;
        
        Ok(())
    }
}
