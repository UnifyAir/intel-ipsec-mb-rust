use crate::error::{MbError, MbMgrErrorKind};
use crate::job::MbJob;
use crate::mgr::MbMgr;
use crate::operation::Operation;
use intel_ipsec_mb_sys::IMB_SNOW3G_DIGEST_LEN;
use intel_ipsec_mb_sys::Snow3gKeySchedule;
use intel_ipsec_mb_sys::{
    IMB_SNOW3G_IV_LEN_IN_BYTES, ImbChainOrder, ImbCipherDirection, ImbCipherMode, ImbHashAlg,
    snow3g_f9_iv_gen,
};
use std::mem::MaybeUninit;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct Snow3gUia2<'buf, 'out, B: AsRef<[u8]> + ?Sized + 'buf, O: AsMut<[u8]> + ?Sized + 'out> {
    pub buffer: &'buf B,
    pub output: &'out mut O,
    pub key: &'out B,
    pub iv: &'out mut O,
    pub count: u32,
    pub fresh: u32,
    pub dir: ImbCipherDirection,
    pub exp_key: MaybeUninit<Snow3gKeySchedule>,
}

impl<'anchor, 'buf, 'out, B, O> Operation<'anchor> for Snow3gUia2<'buf, 'out, B, O>
where
    'buf: 'anchor,
    'out: 'anchor,
    B: AsRef<[u8]> + ?Sized + 'buf,
    O: AsMut<[u8]> + ?Sized + 'out,
{
    fn fill_job(&mut self, job: &MbJob, mgr: &MbMgr) -> Result<&'anchor (), MbError> {

        self.snow_3g_uia2_iv_gen()?;

        let output_slice = self.output.as_mut();
        if output_slice.len() < IMB_SNOW3G_DIGEST_LEN as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidOutputSize));
        }
        let exp_key_ptr = self.exp_key.as_mut_ptr();
        let is_key_sched = mgr.exec(|mgr_ptr| unsafe {
            let snow3g_init_key_sched = (*mgr_ptr).snow3g_init_key_sched.unwrap();
            snow3g_init_key_sched(self.key.as_ref().as_ptr() as *const c_void, exp_key_ptr)
        })?;

        if is_key_sched != 0 {
            return Err(MbError::from_kind(MbMgrErrorKind::InvalidParams));
        }

        let buffer_slice = self.buffer.as_ref();
        let iv_ref = self.iv.as_mut();

        // SAFETY: The MbMgr is assumed to be properly initialized before this.
        // The MbMgr instance must be properly initialized before calling fill_job.
        // This function assumes that the underlying manager pointer is valid and points to a
        // correctly initialized IMB_MGR structure. If the manager is not initialized, using
        // the returned job pointer or filling the job may result in undefined behavior.
        let imb_job = unsafe { &mut *job.as_ptr() };

        imb_job.hash_alg = ImbHashAlg::IMB_AUTH_SNOW3G_UIA2_BITLEN;

        imb_job.__bindgen_anon_1.src = buffer_slice.as_ptr();
        imb_job.hash_start_src_offset_in_bytes = 0;

        imb_job.__bindgen_anon_5.msg_len_to_hash_in_bytes = buffer_slice.len() as u64;

        imb_job.auth_tag_output = output_slice.as_mut_ptr();
        imb_job.auth_tag_output_len_in_bytes = IMB_SNOW3G_DIGEST_LEN as u64;

        imb_job.u.SNOW3G_UIA2._key = exp_key_ptr as *const c_void;
        imb_job.u.SNOW3G_UIA2._iv = iv_ref.as_ptr() as *const c_void;

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

impl<'buf, 'out, B: AsRef<[u8]> + ?Sized + 'buf, O: AsMut<[u8]> + ?Sized + 'out>
    Snow3gUia2<'buf, 'out, B, O>
{
    fn snow_3g_uia2_iv_gen(&mut self) -> Result<(), MbError> {
        if self.iv.as_mut().len() < IMB_SNOW3G_IV_LEN_IN_BYTES as usize {
            return Err(MbError::from_kind(MbMgrErrorKind::IvLen));
        }

        let iv_ptr = self.iv.as_mut();

        match unsafe {
            snow3g_f9_iv_gen(
                self.count,
                self.fresh,
                self.dir as u8,
                iv_ptr.as_mut_ptr() as *mut c_void,
            )
        } {
            0 => Ok(()),
            _ => Err(MbError::from_kind(MbMgrErrorKind::InvalidParams)),
        }
    }
}
