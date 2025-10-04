// use crate::error::MbMgrError;
// use crate::mgr::MbMgr;
// use intel_ipsec_mb_sys::{
//     IMB_SHA224_DIGEST_SIZE_IN_BYTES, IMB_SHA256_DIGEST_SIZE_IN_BYTES,
//     IMB_SHA384_DIGEST_SIZE_IN_BYTES, IMB_SHA512_DIGEST_SIZE_IN_BYTES,
// };
// use std::os::raw::c_void;

// impl MbMgr {
//     /// Compute SHA-224 hash of the input buffer
//     pub fn sha224(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA224_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA224_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha224_fn = (*mgr_ptr).sha224.unwrap();
//             sha224_fn(
//                 buffer.as_ptr() as *const c_void,
//                 buffer.len() as u64,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-256 hash of the input buffer
//     pub fn sha256(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA256_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA256_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha256_fn = (*mgr_ptr).sha256.unwrap();
//             sha256_fn(
//                 buffer.as_ptr() as *const c_void,
//                 buffer.len() as u64,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-384 hash of the input buffer
//     pub fn sha384(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA384_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA384_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha384_fn = (*mgr_ptr).sha384.unwrap();
//             sha384_fn(
//                 buffer.as_ptr() as *const c_void,
//                 buffer.len() as u64,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-512 hash of the input buffer
//     pub fn sha512(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA512_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA512_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha512_fn = (*mgr_ptr).sha512.unwrap();
//             sha512_fn(
//                 buffer.as_ptr() as *const c_void,
//                 buffer.len() as u64,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-224 hash of one block (64 bytes)
//     pub fn sha224_one_block(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA224_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA224_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha224_fn = (*mgr_ptr).sha224_one_block.unwrap();
//             sha224_fn(
//                 buffer.as_ptr() as *const c_void,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-256 hash of one block (64 bytes)
//     pub fn sha256_one_block(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA256_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA256_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha256_fn = (*mgr_ptr).sha256_one_block.unwrap();
//             sha256_fn(
//                 buffer.as_ptr() as *const c_void,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-384 hash of one block (128 bytes)
//     pub fn sha384_one_block(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA384_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA384_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha384_fn = (*mgr_ptr).sha384_one_block.unwrap();
//             sha384_fn(
//                 buffer.as_ptr() as *const c_void,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }

//     /// Compute SHA-512 hash of one block (128 bytes)
//     pub fn sha512_one_block(
//         &mut self,
//         buffer: &[u8],
//     ) -> Result<[u8; IMB_SHA512_DIGEST_SIZE_IN_BYTES as usize], MbMgrError> {
//         let mut output = [0; IMB_SHA512_DIGEST_SIZE_IN_BYTES as usize];
//         self.exec(|mgr_ptr| unsafe {
//             let sha512_fn = (*mgr_ptr).sha512_one_block.unwrap();
//             sha512_fn(
//                 buffer.as_ptr() as *const c_void,
//                 output.as_mut_ptr() as *mut c_void,
//             );
//         })?;
//         Ok(output)
//     }
// }
