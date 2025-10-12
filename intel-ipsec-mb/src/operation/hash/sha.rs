use crate::error::{MbError, MbMgrErrorKind};
use crate::job::MbJob;
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::*;
use std::os::raw::c_void;
use crate::operation::Operation;
#[derive(Debug)]
pub struct Sha1<'buf, 'out, B: AsRef<[u8]> + ?Sized + 'buf, O: AsMut<[u8]> + ?Sized + 'out> {
    pub buffer: &'buf B,
    pub output: &'out mut O,
}

impl<'anchor, 'buf, 'out, B, O> Operation<'anchor> 
    for Sha1<'buf, 'out, B, O>
where
    'buf: 'anchor, 
    'out: 'anchor, 
    B: AsRef<[u8]> + ?Sized + 'buf,
    O: AsMut<[u8]> + ?Sized + 'out,
{
    fn fill_job(&mut self, job: &MbJob, _mgr: &MbMgr) -> Result<&'anchor (), MbError> {
        let buffer_slice = self.buffer.as_ref();
        let output_slice = self.output.as_mut();
        
        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }
        
        // SAFETY: The MbMgr is assumed to be properly initialized before this.
        // The MbMgr instance must be properly initialized before calling fill_job.
        // This function assumes that the underlying manager pointer is valid and points to a
        // correctly initialized IMB_MGR structure. If the manager is not initialized, using
        // the returned job pointer or filling the job may result in undefined behavior.
        let imb_job = unsafe {&mut *job.as_ptr() };
        
        imb_job.hash_alg = ImbHashAlg::IMB_AUTH_SHA_1;
        imb_job.__bindgen_anon_1.src = buffer_slice.as_ptr();
        imb_job.hash_start_src_offset_in_bytes = 0;
        imb_job.__bindgen_anon_5.msg_len_to_hash_in_bytes = buffer_slice.len() as u64;
        imb_job.auth_tag_output = output_slice.as_mut_ptr();
        imb_job.auth_tag_output_len_in_bytes = IMB_SHA1_DIGEST_SIZE_IN_BYTES as u64;
        imb_job.chain_order = ImbChainOrder::IMB_ORDER_HASH_CIPHER;
        imb_job.cipher_mode = ImbCipherMode::IMB_CIPHER_NULL;
        imb_job.__bindgen_anon_4.msg_len_to_cipher_in_bytes = 0;
        imb_job.__bindgen_anon_3.cipher_start_src_offset_in_bytes = 0;
        imb_job.__bindgen_anon_2.dst = buffer_slice.as_ptr() as *mut u8;
        imb_job.cipher_direction = ImbCipherDirection::IMB_DIR_ENCRYPT;
        imb_job.enc_keys = std::ptr::null();
        imb_job.dec_keys = std::ptr::null();
        imb_job.key_len_in_bytes = 0;
        imb_job.iv = std::ptr::null();
        imb_job.iv_len_in_bytes = 0;
        
        Ok(&())
    }
}

#[derive(Debug)]
pub struct Sha1OneBlock<'buf, 'out, B: AsRef<[u8]> + ?Sized + 'buf, O: AsMut<[u8]> + ?Sized + 'out> {
    pub buffer: &'buf B,
    pub output: &'out mut O,
}

impl<'anchor, 'buf, 'out, B, O> Operation<'anchor> 
    for Sha1OneBlock<'buf, 'out, B, O>
where
    'buf: 'anchor, 
    'out: 'anchor, 
    B: AsRef<[u8]> + ?Sized + 'buf,
    O: AsMut<[u8]> + ?Sized + 'out,
{
    fn fill_job(&mut self, job: &MbJob, _mgr: &MbMgr) -> Result<&'anchor (), MbError> {
        let buffer_slice = self.buffer.as_ref();
        let output_slice = self.output.as_mut();
        
        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }
        
        // SAFETY: The MbMgr is assumed to be properly initialized before this.
        // The MbMgr instance must be properly initialized before calling fill_job.
        // This function assumes that the underlying manager pointer is valid and points to a
        // correctly initialized IMB_MGR structure. If the manager is not initialized, using
        // the returned job pointer or filling the job may result in undefined behavior.
        let imb_job = unsafe {&mut *job.as_ptr() };
        
        imb_job.hash_alg = ImbHashAlg::IMB_AUTH_SHA_1;
        imb_job.__bindgen_anon_1.src = buffer_slice.as_ptr();
        imb_job.hash_start_src_offset_in_bytes = 0;
        imb_job.__bindgen_anon_5.msg_len_to_hash_in_bytes = 64u64;
        imb_job.auth_tag_output = output_slice.as_mut_ptr();
        imb_job.auth_tag_output_len_in_bytes = IMB_SHA1_DIGEST_SIZE_IN_BYTES as u64;
        imb_job.chain_order = ImbChainOrder::IMB_ORDER_HASH_CIPHER;
        imb_job.cipher_mode = ImbCipherMode::IMB_CIPHER_NULL;
        imb_job.__bindgen_anon_4.msg_len_to_cipher_in_bytes = 0;
        imb_job.__bindgen_anon_3.cipher_start_src_offset_in_bytes = 0;
        imb_job.__bindgen_anon_2.dst = buffer_slice.as_ptr() as *mut u8;
        imb_job.cipher_direction = ImbCipherDirection::IMB_DIR_ENCRYPT;
        imb_job.enc_keys = std::ptr::null();
        imb_job.dec_keys = std::ptr::null();
        imb_job.key_len_in_bytes = 0;
        imb_job.iv = std::ptr::null();
        imb_job.iv_len_in_bytes = 0;
        
        Ok(&())
    }
}



impl MbMgr {
    pub fn sha1<B, O>(&self, buffer: &B, output: &mut O) -> Result<(), MbError>
    where
        B: AsRef<[u8]>,
        O: AsMut<[u8]>,
    {
        let buffer_slice = buffer.as_ref();
        let output_slice = output.as_mut();

        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }

        // SAFETY: The MbMgr is assumed to be properly initialized before this.
        self.exec(|mgr_ptr| unsafe {
            let sha1_fn = (*mgr_ptr).sha1.unwrap();
            sha1_fn(
                buffer_slice.as_ptr() as *const c_void,
                buffer_slice.len() as u64,
                output_slice.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(())
    }

    pub fn sha1_one_block<B, O>(&mut self, buffer: &B, output: &mut O) -> Result<(), MbError>
    where
        B: AsRef<[u8]>,
        O: AsMut<[u8]>,
    {
        let buffer_slice = buffer.as_ref();
        let output_slice = output.as_mut();

        if output_slice.len() < IMB_SHA1_DIGEST_SIZE_IN_BYTES as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }

        // SAFETY: The MbMgr is assumed to be properly initialized before this.
        self.exec(|mgr_mut_ptr| unsafe {
            let sha1_fn = (*mgr_mut_ptr).sha1_one_block.unwrap();
            sha1_fn(
                buffer_slice.as_ptr() as *const c_void,
                output_slice.as_mut_ptr() as *mut c_void,
            );
        })?;
        Ok(())
    }
}
