// use crate::error::MbMgrError;
// use crate::mgr::MbMgr;
// use intel_ipsec_mb_sys::IMB_MD5_DIGEST_SIZE_IN_BYTES;
// use std::os::raw::c_void;

// impl MbMgr {
//     /// Compute MD5 hash of the input buffer (one block only)
//     /// Note: This function only processes one block at a time
//     pub fn md5_one_block(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_MD5_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_MD5_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let md5_fn = (*mgr_ptr).md5_one_block.unwrap();
//             md5_fn(
//                 buffer.as_ptr() as *const c_void,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }
// }
