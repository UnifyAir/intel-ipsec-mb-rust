use intel_ipsec_mb_sys::ImbMgr;
use intel_ipsec_mb_sys::alloc_mb_mgr;
use intel_ipsec_mb_sys::free_mb_mgr;

#[derive(Debug)]
pub struct MultiBufferManager {
    mgr: ImbMgr,
}

impl Drop for MultiBufferManager {
    fn drop(&mut self) {
        unsafe {
            free_mb_mgr(self.mgr);
        }
    }
}

impl MultiBufferManager {
    /// Create a new MultiBufferManager with the given flags.
    ///
    /// # Arguments
    ///
    /// * `flags` - Multi-buffer manager flags (e.g., IMB_FLAG_SHANI_OFF, IMB_FLAG_GFNI_OFF)
    ///
    /// # Panics
    ///
    /// Panics if allocation fails.
    pub fn with_flags(flags: u64) -> Self {
        unsafe {
            let mgr = alloc_mb_mgr(flags);
            if mgr.is_null() {
                panic!("Failed to allocate MultiBufferManager (alloc_mb_mgr returned NULL)");
            }
            Self { mgr }
        }
    }

    /// Create a new MultiBufferManager with default flags (0).
    pub fn new() -> Self {
        Self::with_flags(0)
    }
}