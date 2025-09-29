use intel_ipsec_mb_sys::{ImbMgr, alloc_mb_mgr, free_mb_mgr};
use std::fmt;
use std::ptr::NonNull;
use crate::error::MbMgrError;

#[derive(Debug, Clone, Copy, Default)]
pub struct MbMgrConfig {
    pub disable_shani: bool,
    pub disable_gfni: bool,
}

impl MbMgrConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn disable_shani(mut self) -> Self {
        self.disable_shani = true;
        self
    }
    
    pub fn disable_gfni(mut self) -> Self {
        self.disable_gfni = true;
        self
    }
    
    pub fn build(self) -> Result<MultiBufferManager, MbMgrError> {
        MultiBufferManager::with_config(self)
    }
    
    pub(crate) fn to_flags(self) -> u64 {
        let mut flags = 0u64;
        if self.disable_shani { flags |= 1 << 0; } // IMB_FLAG_SHANI_OFF
        if self.disable_gfni  { flags |= 1 << 1; } // IMB_FLAG_GFNI_OFF
        flags
    }
}



pub struct MultiBufferManager {
    mgr: NonNull<ImbMgr>,
}

impl MultiBufferManager {
    pub fn as_ptr(&self) -> *mut ImbMgr {
        self.mgr.as_ptr()
    }
}

impl fmt::Debug for MultiBufferManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mgr_ref = self.mgr.as_ref();
            f.debug_struct("MultiBufferManager")
                .field("flags", &format!("0x{:x}", mgr_ref.flags))
                .field("features", &format!("0x{:x}", mgr_ref.features))
                .field("used_arch_type", &mgr_ref.used_arch_type)
                .field("used_arch", &mgr_ref.used_arch)
                .field("imb_errno", &mgr_ref.imb_errno)
                .finish()
        }
    }
}

impl Drop for MultiBufferManager {
    fn drop(&mut self) {
        unsafe {
            free_mb_mgr(self.mgr.as_ptr());
        }
    }
}

impl MultiBufferManager {
    pub fn new() -> Result<Self, MbMgrError> {
        Self::with_config(MbMgrConfig::default())
    }

    // Todo: handle initialization without alloc_mb_mgr and using init_mb_mgr
    
    pub fn with_config(config: MbMgrConfig) -> Result<Self, MbMgrError> {
        unsafe {
            let mgr = alloc_mb_mgr(config.to_flags());
            
            if mgr.is_null() {
                return Err(MbMgrError::capture_global());
            }
            
            let mgr = NonNull::new_unchecked(mgr);
            let manager = Self { mgr };
            
            Ok(manager)
        }
    }
    
    pub fn builder() -> MbMgrConfig {
        MbMgrConfig::new()
    }
    
    // /// Get current flags (useful for debugging)
    // pub fn flags(&self) -> u64 {
    //     unsafe { self.mgr.as_ref().flags }
    // }
    
    // /// Get current features (shows what optimizations are actually enabled)
    // pub fn features(&self) -> u64 {
    //     unsafe { self.mgr.as_ref().features }
    // }
    
    // /// Check if specific features are enabled
    // pub fn has_feature(&self, feature_mask: u64) -> bool {
    //     unsafe { self.mgr.as_ref().features & feature_mask != 0 }
    // }
    
    // /// Get the architecture type being used
    // pub fn arch_type(&self) -> u8 {
    //     unsafe { self.mgr.as_ref().used_arch_type }
    // }
    
    // /// Get the architecture being used
    // pub fn arch(&self) -> u32 {
    //     unsafe { self.mgr.as_ref().used_arch }
    // }
    
    // /// Get last error code
    // pub fn errno(&self) -> i32 {
    //     unsafe { self.mgr.as_ref().imb_errno }
    // }
    
    // /// Get raw pointer (for passing to C functions)
    // pub(crate) fn as_ptr(&self) -> *mut IMB_MGR {
    //     self.mgr.as_ptr()
    // }
}

// // Make MultiBufferManager Send + Sync if needed for your threading model
// unsafe impl Send for MultiBufferManager {}
// unsafe impl Sync for MultiBufferManager {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_default_creation() {
//         let mgr = MultiBufferManager::new().expect("Failed to create default manager");
//         assert_eq!(mgr.flags(), 0); // No flags should be set
//     }

//     #[test]
//     fn test_builder_pattern() {
//         let mgr = MultiBufferManager::builder()
//             .disable_shani()
//             .disable_gfni()
//             .build()
//             .expect("Failed to create manager with flags");
        
//         assert_eq!(mgr.flags(), 3); // Both flags set: (1<<0) | (1<<1) = 3
//     }

//     #[test]
//     fn test_config_flags() {
//         let config = MbMgrConfig::new()
//             .disable_shani()
//             .disable_gfni();
        
//         assert_eq!(config.to_flags(), 3);
//     }
// }