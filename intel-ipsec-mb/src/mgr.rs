use crate::config::MbMgrConfig;
use crate::error::MbError;
use intel_ipsec_mb_sys::{ImbMgr, alloc_mb_mgr, free_mb_mgr, init_mb_mgr_auto};
use std::fmt;
use std::ptr::NonNull;

use std::marker::PhantomData;

pub struct MbMgr {
    mgr: NonNull<ImbMgr>,
    // outstanding_jobs: RefCell<HashMap<MbJob, JobState>>,
    _not_thread_safe: PhantomData<*const ()>
}


impl MbMgr {
    // For operations that don't mutate (reading state, etc.)
    pub fn as_ptr(&self) -> *mut ImbMgr {
        self.mgr.as_ptr()
    }
    
    // Temporary disabled as we are not using it, it was there for
    // safety, but it is not needed
    // pub fn as_mut_ptr(&mut self) -> *mut ImbMgr {
    //     self.mgr.as_ptr()
    // }
}

//Todo fix this
impl fmt::Debug for MbMgr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mgr_ref = self.mgr.as_ref();
            f.debug_struct("MbMgr")
                .field("flags", &format!("0x{:x}", mgr_ref.flags))
                .field("features", &format!("0x{:x}", mgr_ref.features))
                .field("used_arch_type", &mgr_ref.used_arch_type)
                .field("used_arch", &mgr_ref.used_arch)
                .field("imb_errno", &mgr_ref.imb_errno)
                .finish()
        }
    }
}

impl Drop for MbMgr {
    fn drop(&mut self) {
        unsafe {
            free_mb_mgr(self.mgr.as_ptr());
        }
    }
}

impl MbMgr {
    pub fn new() -> Result<Self, MbError> {
        Self::with_config(MbMgrConfig::default())
    }

    pub fn with_config(config: MbMgrConfig) -> Result<Self, MbError> {
        unsafe {
            let mgr = alloc_mb_mgr(config.to_flags());
            
            if let Some(err) = MbError::capture_global() {
                return Err(err);
            }

            let mgr = NonNull::new_unchecked(mgr);
            let mut manager = Self {
                mgr,
                _not_thread_safe: PhantomData,
            };

            Self::exec(&mut manager, |mgr| init_mb_mgr_auto(mgr, std::ptr::null_mut()))?;

            Ok(manager)
        }
    }

    pub fn builder() -> MbMgrConfig {
        MbMgrConfig::new()
    }

    pub(crate) fn exec<F, R>(&self, f: F) -> Result<R, MbError>
    where
        F: FnOnce(*mut ImbMgr) -> R,
    {
        let result = f(self.as_ptr());

        match MbError::capture(self) {
            Some(err) => Err(err),
            None => Ok(result),
        }
    }
}
